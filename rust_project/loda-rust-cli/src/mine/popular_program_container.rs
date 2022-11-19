use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use serde::Deserialize;
use rand::Rng;
use rand::seq::SliceRandom;

/// The file `program_popularity.csv`, has a column `popularity` 
/// contain values in the range 0 to 9.
/// 
/// The value `0` is for the unpopular programs, that no other programs depend on.
/// 
/// The value `1` is the little-used programs, where 1 or more programs depend on it.
/// 
/// The values `2..8` are for the medium-used programs.
/// 
/// The value `9` is the most-used programs, that lots of other programs depends on.
const NUMBER_OF_CLUSTERS: u8 = 10;


/// Some programs are more useful than other programs.
///
/// This is a data structure for picking a popular program.
///
/// Without this data structure, it would be terrible time consuming
/// making a weighted choice among the programs.
///
/// Examples of popular programs are: fibonacci, primes, factorial, sqrt(2).
/// A program is popular when lots of other programs depend on it.
/// The popular programs are assigned `cluster_id` 9.
///
/// However the majority of programs are the unpopular programs.
/// These are rarely used. Few other programs depend on these.
/// A recently added program, starts out without any other programs depending on it.
/// Over time, a program may gradually become more popular.
/// The unpopular programs are assigned `cluster_id` 0.
///
/// In between there are the programs of medium usage.
/// These programs are assigned `cluster_id` 1..8.
///
/// On initialization the `program_popularity.csv` is loaded.
/// This CSV file have been generated by using the PageRank algorithm,
/// and dividing the data into 10 clusters.
#[derive(Clone, Debug)]
pub struct PopularProgramContainer {
    cluster_program_ids: Vec<Vec<u32>>,
}

impl PopularProgramContainer {
    pub fn load(path: &Path) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        process_csv_into_clusters(&mut reader)
    }

    #[allow(dead_code)]
    fn cluster_program_ids(&self) -> &Vec<Vec<u32>> {
        &self.cluster_program_ids
    }

    #[allow(dead_code)]
    pub fn choose_weighted_by_popularity<R: Rng + ?Sized>(&self, rng: &mut R) -> Option<u32> {
        let cluster_weight_vec: Vec<(usize,usize)> = vec![
            (0, 1), // Low probability for choosing an unpopular program.
            (1, 2),
            (2, 4),
            (3, 8),
            (4, 16),
            (5, 32),
            (6, 64),
            (7, 128),
            (8, 256),
            (9, 512), // High probablility for choosing a popular program.
        ];
        assert!(cluster_weight_vec.len() == (NUMBER_OF_CLUSTERS as usize));
        let cluster_id: &usize = &cluster_weight_vec.choose_weighted(rng, |item| item.1).unwrap().0;
        let program_ids: &Vec<u32> = &self.cluster_program_ids[*cluster_id];
        if program_ids.is_empty() {
            // The CSV file is supposed to have several program_ids for every cluster_id.
            // No matter what cluster_id is picked, there should be at least 1 program.
            // Return None, in the unfortunate case there isn't any program_ids for the picked cluser_id.
            return None;
        }
        let program_id: u32 = match program_ids.choose(rng) {
            Some(program_id) => *program_id,
            None => {
                // For a non-empty vector, this shouldn't happen.
                return None;
            }
        };
        Some(program_id)
    }

    fn choose_from_cluster<R: Rng + ?Sized>(&self, rng: &mut R, cluster_id: u8) -> Option<u32> {
        assert!(self.cluster_program_ids.len() == (NUMBER_OF_CLUSTERS as usize));
        assert!((cluster_id as usize) < self.cluster_program_ids.len());
        let program_ids: &Vec<u32> = &self.cluster_program_ids[cluster_id as usize];
        if program_ids.is_empty() {
            // The CSV file is supposed to have several program_ids for every cluster_id.
            // No matter what cluster_id is picked, there should be at least 1 program.
            // Return None, in the unfortunate case there isn't any program_ids for the picked cluser_id.
            return None;
        }
        let program_id: u32 = match program_ids.choose(rng) {
            Some(program_id) => *program_id,
            None => {
                // For a non-empty vector, this shouldn't happen.
                return None;
            }
        };
        Some(program_id)
    }

    #[allow(dead_code)]
    pub fn choose_most_popular<R: Rng + ?Sized>(&self, rng: &mut R) -> Option<u32> {
        self.choose_from_cluster(rng, 9)
    }

    #[allow(dead_code)]
    pub fn choose_medium_popular<R: Rng + ?Sized>(&self, rng: &mut R) -> Option<u32> {
        let cluster_id: u8 = rng.gen_range(1..8);
        self.choose_from_cluster(rng, cluster_id)
    }

    #[allow(dead_code)]
    pub fn choose_least_popular<R: Rng + ?Sized>(&self, rng: &mut R) -> Option<u32> {
        self.choose_from_cluster(rng, 0)
    }
}

#[derive(Debug)]
pub enum ProgramPopularityError {
    PopularityClusterIdOutOfBounds,
}

impl fmt::Display for ProgramPopularityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::PopularityClusterIdOutOfBounds => 
                write!(f, "Cluster id is out of bounds")
        }
    }
}

impl Error for ProgramPopularityError {}

fn process_csv_into_clusters(reader: &mut dyn BufRead) -> Result<PopularProgramContainer, Box<dyn Error>> {
    let records: Vec<Record> = process_csv_data(reader)?;
    convert_records_to_clusters(records)
}

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "program id")]
    program_id: u32,

    #[serde(rename = "popularity")]
    popularity_cluster_id: u8,
}

impl Record {
    #[cfg(test)]
    fn new(program_id: u32, popularity_cluster_id: u8) -> Self {
        Self {
            program_id: program_id,
            popularity_cluster_id: popularity_cluster_id,
        }
    }
}

fn process_csv_data(reader: &mut dyn BufRead) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut records = Vec::<Record>::new();
    let mut csv_reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(reader);
    for result in csv_reader.deserialize() {
        let record: Record = result?;
        records.push(record);
    }
    Ok(records)
}

fn convert_records_to_clusters(records: Vec<Record>) -> Result<PopularProgramContainer, Box<dyn Error>> {
    // Ensure there isn't too many clusters
    let mut max_cluster_id: u8 = 0;
    for record in &records {
        if max_cluster_id < record.popularity_cluster_id {
            max_cluster_id = record.popularity_cluster_id;
        }
    }
    if max_cluster_id >= NUMBER_OF_CLUSTERS {
        return Err(Box::new(ProgramPopularityError::PopularityClusterIdOutOfBounds));
    }

    // Identify program_ids for each cluster
    let mut clusters: Vec<Vec<u32>> = vec!();
    for cluster_id in 0..NUMBER_OF_CLUSTERS {
        let mut program_ids: Vec<u32> = vec!();
        for record in &records {
            if record.popularity_cluster_id == cluster_id {
                program_ids.push(record.program_id);
            }
        }
        clusters.push(program_ids);
    }

    let container = PopularProgramContainer {
        cluster_program_ids: clusters
    };
    Ok(container)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_10000_process_csv_data() {
        let data = "\
program id;popularity
4;1

5;9
6;8
7;3
";
        let mut input: &[u8] = data.as_bytes();
        let records: Vec<Record> = process_csv_data(&mut input).unwrap();
        let strings: Vec<String> = records.iter().map(|record| {
            format!("{} {}", record.program_id, record.popularity_cluster_id)
        }).collect();
        let strings_joined: String = strings.join(",");
        assert_eq!(strings_joined, "4 1,5 9,6 8,7 3");
    }

    #[test]
    fn test_10001_convert_records_to_clusters_error_too_many_clusters() {
        let records: Vec<Record> = vec![
            // Cluster 9 is the highest allowed cluster.
            // Here using cluster 10 is beyond the max cluster and should trigger an error.
            Record::new(666, 10),
        ];
        let result = convert_records_to_clusters(records);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_10002_convert_records_to_clusters_success() {
        let records: Vec<Record> = vec![
            // 3 items in cluster 0
            Record::new(101, 0),
            Record::new(102, 0),
            Record::new(103, 0),

            // 1 item in cluster 4
            Record::new(301, 4),

            // 2 items in cluster 9
            Record::new(901, 9),
            Record::new(902, 9),
        ];
        let container: PopularProgramContainer = convert_records_to_clusters(records).unwrap();
        let cluster_program_ids: &Vec<Vec<u32>> = container.cluster_program_ids();
        assert_eq!(cluster_program_ids.len(), 10);
        assert_eq!(cluster_program_ids[0].len(), 3);
        assert_eq!(cluster_program_ids[4].len(), 1);
        assert_eq!(cluster_program_ids[9].len(), 2);
    }
}
