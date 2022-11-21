use crate::analytics::Analytics;
use crate::config::Config;
use crate::oeis::{load_terms_to_program_id_set, TermsToProgramIdSet};
use super::{CreateFunnel, Funnel, FunnelConfig};
use super::{create_genome_mutate_context, GenomeMutateContext};
use super::SharedWorkerState;
use super::MinerWorkerMessageWithAnalytics;
use super::{create_prevent_flooding, PreventFlooding};
use super::{MinerSyncExecute, MinerSyncExecuteStatus};
use bastion::prelude::*;
use num_bigint::{BigInt, ToBigInt};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::time::Instant;

const ANALYTICS_INTERVAL_MILLIS: u64 = 10000;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AnalyticsWorkerMessage {
    RunLaunchProcedure,
    RegenerateAnalyticsJob,
}

pub async fn analytics_worker(
    ctx: BastionContext,
    config: Config,
    prevent_flooding: Arc<Mutex<PreventFlooding>>,
    shared_worker_state: Arc<Mutex<SharedWorkerState>>,
) -> Result<(), ()> {
    let miner_worker_distributor = Distributor::named("miner_worker");
    let mut progress_time = Instant::now();
    loop {
        let elapsed: u128 = progress_time.elapsed().as_millis();
        if elapsed >= (ANALYTICS_INTERVAL_MILLIS as u128) {
            println!("analytics");
            progress_time = Instant::now();
        }

        let timeout = Duration::from_millis(ANALYTICS_INTERVAL_MILLIS);
        let message: SignedMessage = match ctx.try_recv_timeout(timeout).await {
            Ok(message) => message,
            Err(error) => {
                if let ReceiveError::Timeout(_duration) = error {
                    debug!("analytics_worker: timeout happened");
                    continue;
                }
                error!("analytics_worker: Unknown error happened. error: {:?}", error);
                continue;
            }
        };
        let mut should_run_launch_procedure: bool = false;
        let mut should_perform_sync: bool = false;
        MessageHandler::new(message)
            .on_tell(|message: AnalyticsWorkerMessage, _| {
                println!(
                    "analytics_worker: child {}, received broadcast message: {:?}",
                    ctx.current().id(),
                    message
                );
                match message {
                    AnalyticsWorkerMessage::RunLaunchProcedure => {
                        should_run_launch_procedure = true;
                    },
                    AnalyticsWorkerMessage::RegenerateAnalyticsJob => {
                        should_perform_sync = true;
                    },
                }
            })
            .on_fallback(|unknown, _sender_addr| {
                error!(
                    "analytics_worker {}, received an unknown message!:\n{:?}",
                    ctx.current().id(),
                    unknown
                );
            });

        let mut analytics_to_be_performed = AnalyticsTypeToPerform::RegenerateIfExpired;

        if should_run_launch_procedure || should_perform_sync {
            let executable_path: PathBuf = config.miner_sync_executable();
            let status: MinerSyncExecuteStatus = match MinerSyncExecute::execute(&executable_path) {
                Ok(value) => value,
                Err(error) => {
                    Bastion::stop();
                    panic!("Problem executing MinerSyncExecute: {:?}", error);
                }
            };
            println!("Successfully executed MinerSyncExecute. status: {:?}", status);

            match status {
                MinerSyncExecuteStatus::Changed => {
                    // Data has been modified, then analytics needs to be regenerated.
                    analytics_to_be_performed = AnalyticsTypeToPerform::ForceRegenerate;
                },
                MinerSyncExecuteStatus::NoChange => {
                    // Data is already uptodate, then skip no need to regenerate analytics.
                    analytics_to_be_performed = AnalyticsTypeToPerform::RegenerateIfExpired;
                }
            }
        }

        if should_run_launch_procedure {
            let analytics_run_result: anyhow::Result<()> = match analytics_to_be_performed {
                AnalyticsTypeToPerform::RegenerateIfExpired => {
                    println!("BEFORE analytics - run_if_expired");
                    Analytics::run_if_expired()
                },
                AnalyticsTypeToPerform::ForceRegenerate => {
                    println!("BEFORE analytics - run_force");
                    Analytics::run_force()
                }
            };
            match analytics_run_result {
                Ok(()) => {},
                Err(error) => {
                    Bastion::stop();
                    panic!("AFTER analytics. error: {:?}", error);
                }
            }

            let prevent_flooding_x: PreventFlooding = match create_prevent_flooding(&config) {
                Ok(value) => value,
                Err(error) => {
                    Bastion::stop();
                    panic!("analytics_worker: create_prevent_flooding failed. error: {:?}", error);
                }
            };
            match prevent_flooding.lock() {
                Ok(mut instance) => {
                    *instance = prevent_flooding_x;
                },
                Err(error) => {
                    Bastion::stop();
                    panic!("analytics_worker: Unable to populate PreventFlooding mechanism. error: {:?}", error);
                }
            }

            println!("populating terms_to_program_id");
            let oeis_stripped_file: PathBuf = config.oeis_stripped_file();
            let padding_value: BigInt = FunnelConfig::WILDCARD_MAGIC_VALUE.to_bigint().unwrap();
            let terms_to_program_id_result = load_terms_to_program_id_set(
                &oeis_stripped_file, 
                FunnelConfig::MINIMUM_NUMBER_OF_REQUIRED_TERMS, 
                FunnelConfig::TERM_COUNT,
                &padding_value
            );
            let terms_to_program_id: TermsToProgramIdSet = match terms_to_program_id_result {
                Ok(value) => value,
                Err(error) => {
                    Bastion::stop();
                    panic!("analytics_worker: Unable to load terms for program ids. error: {:?}", error);
                }
            };
            let terms_to_program_id_arc: Arc<TermsToProgramIdSet> = Arc::new(terms_to_program_id);

            println!("populating funnel");
            let funnel: Funnel = Funnel::create_funnel_with_file_data(&config);
            println!("populating genome_mutate_context");
            let genome_mutate_context: GenomeMutateContext = create_genome_mutate_context(&config);
            
            // Pass on funnel+genome_mutate_context to miner_workers
            println!("analytics_worker: sending analytics data to miner_workers");
            let instance = MinerWorkerMessageWithAnalytics::new(
                funnel,
                genome_mutate_context,
                terms_to_program_id_arc,
            );
            let arc_instance = Arc::new(instance);
            let tell_result = miner_worker_distributor.tell_everyone(arc_instance);
            if let Err(error) = tell_result {
                Bastion::stop();
                panic!("analytics_worker: Unable to send MinerWorkerMessageWithAnalytics to miner_worker_distributor. error: {:?}", error);
            }
    
            thread::sleep(Duration::from_millis(1000));
            println!("AFTER analytics. ok");
            match shared_worker_state.lock() {
                Ok(mut state) => {
                    *state = SharedWorkerState::Mining;
                },
                Err(error) => {
                    Bastion::stop();
                    panic!("analytics_worker: Unable to change state=Mining. error: {:?}", error);
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum AnalyticsTypeToPerform {
    RegenerateIfExpired,
    ForceRegenerate
}
