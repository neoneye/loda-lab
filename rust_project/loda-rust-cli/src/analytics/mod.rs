//! Prepare data needed for mining, by analyzing the existing programs.
mod analytics_error;
mod analyze_dependencies;
mod analyze_instruction_constant;
mod analyze_instruction_ngram;
mod analyze_source_ngram;
mod analyze_program_complexity;
mod analyze_target_ngram;
mod batch_program_analyzer;
mod deny_file;
mod dont_mine;
mod histogram_stripped_file;
mod program_rank;
mod validate_programs;

pub use analytics_error::AnalyticsError;
pub use analyze_dependencies::AnalyzeDependencies;
pub use analyze_instruction_constant::AnalyzeInstructionConstant;
pub use analyze_instruction_ngram::AnalyzeInstructionNgram;
pub use analyze_program_complexity::AnalyzeProgramComplexity;
pub use analyze_source_ngram::AnalyzeSourceNgram;
pub use analyze_target_ngram::AnalyzeTargetNgram;
pub use batch_program_analyzer::{BatchProgramAnalyzer, BatchProgramAnalyzerContext, BatchProgramAnalyzerPlugin, BatchProgramAnalyzerPluginItem};
pub use deny_file::load_program_ids_from_deny_file;
pub use dont_mine::DontMine;
pub use histogram_stripped_file::HistogramStrippedFile;
pub use program_rank::compute_program_rank;
pub use validate_programs::ValidatePrograms;
