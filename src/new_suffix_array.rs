use crate::factorization::custom_factorization::get_custom_factors_and_more_using_chunk_size;
use crate::factorization::icfl::get_icfl_indexes;
use crate::factorization::logging::log_factorization;
use crate::files::json::dump_json_in_file;
use crate::files::paths::{
    get_path_for_project_factorization_file, get_path_for_project_folder,
    get_path_for_project_mini_tree_file, get_path_for_project_outcome_file_json,
    get_path_for_project_suffix_array_file, get_path_for_project_timing_file_json,
};
use crate::prefix_tree::log_execution_info::ExecutionInfoFileFormat;
use crate::prefix_tree::log_execution_outcome::ExecutionOutcomeFileFormat;
use crate::prefix_tree::logging::{log_tree, TreeLogMode};
use crate::prefix_tree::monitor::{ExecutionInfo, Monitor};
use crate::prefix_tree::tree::{create_tree, Tree};
use crate::suffix_array::logger::{log_suffix_array, make_sure_directory_exist};

// INNOVATIVE SUFFIX ARRAY
pub struct InnovativeSuffixArrayComputationResults {
    pub suffix_array: Vec<usize>,
    pub execution_info: ExecutionInfo,
}
pub struct InnTry {
    project_folder: String,
    project_factorization_file: String,
    project_mini_tree_file: String,
    project_suffix_array_file: String,
    project_outcome_file_json: String,
    project_timing_file_json: String,
    factor_indexes: Vec<usize>,
    icfl_indexes: Vec<usize>,
}
impl InnTry {
    fn new(fasta_file_name: &str, chunk_size_or_zero: usize) -> Self {
        let project_folder = get_path_for_project_folder(fasta_file_name);
        let project_factorization_file =
            get_path_for_project_factorization_file(fasta_file_name, chunk_size_or_zero);
        let project_mini_tree_file =
            get_path_for_project_mini_tree_file(fasta_file_name, chunk_size_or_zero);
        let project_suffix_array_file =
            get_path_for_project_suffix_array_file(fasta_file_name, chunk_size_or_zero);
        let project_outcome_file_json =
            get_path_for_project_outcome_file_json(fasta_file_name, chunk_size_or_zero);
        let project_timing_file_json =
            get_path_for_project_timing_file_json(fasta_file_name, chunk_size_or_zero);
        Self {
            project_folder,
            project_factorization_file,
            project_mini_tree_file,
            project_suffix_array_file,
            project_outcome_file_json,
            project_timing_file_json,
            factor_indexes: Vec::new(),
            icfl_indexes: Vec::new(),
        }
    }
    fn log_fact(&self, str: &str) {
        make_sure_directory_exist(&self.project_folder);
        log_factorization(
            &self.factor_indexes,
            &self.icfl_indexes,
            str,
            &self.project_factorization_file,
        );
    }
    fn log_trees(&self, tree: &Tree, str_chars: &Vec<char>) {
        make_sure_directory_exist(&self.project_folder);
        /*
        log_tree(
            &tree,
            TreeLogMode::Tree,
            get_path_for_project_tree_file(fasta_file_name, chunk_size_or_zero),
            &str_chars,
        );
        log_tree(
            &tree,
            TreeLogMode::FullTree,
            get_path_for_project_full_tree_file(fasta_file_name, chunk_size_or_zero),
            &str_chars,
        );
        */
        log_tree(
            &tree,
            TreeLogMode::MiniTree,
            &self.project_mini_tree_file,
            &str_chars,
        );
    }
    fn log_suffix_array(&self, suffix_array: &Vec<usize>) {
        log_suffix_array(
            //
            &suffix_array,
            &self.project_suffix_array_file,
        );
    }
    fn log_execution(&self, execution_info: &ExecutionInfo) {
        make_sure_directory_exist(&self.project_folder);
        // Execution Outcome JSON file
        let execution_outcome_file_format =
            ExecutionOutcomeFileFormat::new(&execution_info.execution_outcome);
        dump_json_in_file(
            &execution_outcome_file_format,
            &self.project_outcome_file_json,
        );

        // Execution Timing JSON file
        let execution_timing_file_format =
            ExecutionInfoFileFormat::new(&execution_info.execution_timing);
        dump_json_in_file(
            //
            &execution_timing_file_format,
            &self.project_timing_file_json,
        );
    }
}
pub fn compute_innovative_suffix_array(
    fasta_file_name: &str,
    str: &str,
    chunk_size: Option<usize>,
    log_execution: bool,
    log_fact: bool,
    log_trees_and_suffix_array: bool,
) -> InnovativeSuffixArrayComputationResults {
    let chunk_size_or_zero = chunk_size.unwrap_or(0);

    let mut instance = InnTry::new(fasta_file_name, chunk_size_or_zero);

    let mut monitor = Monitor::new();
    monitor.whole_duration.start();

    // FACTORIZATION
    monitor.p1_fact.start();
    // ICFL Factorization
    let str_chars = str.chars().collect::<Vec<_>>();
    let icfl_indexes = get_icfl_indexes(&str_chars);
    // Custom Factorization
    let (
        //
        factor_indexes,
        idx_to_is_custom,
        idx_to_icfl_factor,
    ) = get_custom_factors_and_more_using_chunk_size(&icfl_indexes, chunk_size, str.len());
    monitor.p1_fact.stop();

    instance.factor_indexes.extend(factor_indexes);
    instance.icfl_indexes.extend(icfl_indexes);

    // + Extra
    if log_fact {
        instance.log_fact(str);
    }
    // - Extra

    // TREE
    monitor.p2_tree.start();
    let mut tree = create_tree(
        &str_chars,
        &instance.factor_indexes,
        &instance.icfl_indexes,
        &idx_to_is_custom,
        &mut monitor,
    );
    monitor.p2_tree.stop();

    // + Extra
    if cfg!(feature = "verbose") {
        println!("Before SUFFIX ARRAY PHASE");
        print_for_human_like_debug(
            str,
            &instance.icfl_indexes,
            &instance.factor_indexes,
            &idx_to_icfl_factor,
            &idx_to_is_custom,
        );
        tree.print(&str_chars);
    }
    if log_trees_and_suffix_array {
        instance.log_trees(&tree, &str_chars);
    }
    // - Extra

    // SUFFIX ARRAY
    monitor.p3_sa.start();
    let suffix_array = tree.compute_suffix_array(
        str,
        &instance.icfl_indexes,
        &idx_to_is_custom,
        &idx_to_icfl_factor,
        &mut monitor,
    );
    monitor.p3_sa.stop();
    monitor.whole_duration.stop();

    // + Extra
    if cfg!(feature = "verbose") {
        println!("After SUFFIX ARRAY PHASE");
        tree.print(&str_chars);
    }
    if log_trees_and_suffix_array {
        instance.log_suffix_array(&suffix_array);
    }
    let execution_info = monitor.transform_info_execution_info();
    if log_execution {
        instance.log_execution(&execution_info);
    }
    // println!("Total time: {}", duration.as_secs_f32());
    // - Extra

    InnovativeSuffixArrayComputationResults {
        suffix_array,
        execution_info,
    }
}
fn print_for_human_like_debug(
    str: &str,
    icfl_indexes: &Vec<usize>,
    factor_indexes: &Vec<usize>,
    idx_to_icfl_factor: &Vec<usize>,
    idx_to_is_custom: &Vec<bool>,
    // depths: &Vec<usize>,
) {
    // CHAR INDEXES
    for i in 0..str.len() {
        print!(" {:2} ", i);
    }
    println!();
    // CHARS
    for i in 0..str.len() {
        print!("  {} ", &str[i..i + 1]);
    }
    println!();
    // IDX TO ICFL FACTOR
    for i in 0..str.len() {
        print!(" {:2} ", idx_to_icfl_factor[i]);
    }
    println!("   <= IDX TO ICFL FACTOR {:?}", icfl_indexes);
    let mut i = 0;

    print_indexes_list(&icfl_indexes, str.len());
    println!("<= ICFL FACTOR INDEXES {:?}", icfl_indexes);
    print_indexes_list(&factor_indexes, str.len());
    println!("<= FACTOR INDEXES {:?}", factor_indexes);

    // IDX TO IS CUSTOM FACTOR
    i = 0;
    while i < str.len() {
        print!("  {} ", if idx_to_is_custom[i] { "x" } else { " " });
        i += 1;
    }
    println!("   <= IDX TO IS CUSTOM FACTOR");
    /*for i in 0..str.len() {
        print!(" {:2} ", depths[i]);
    }
    println!("   <= DEPTHS");*/
}
fn print_indexes_list(indexes_list: &Vec<usize>, str_length: usize) {
    let mut iter = &mut indexes_list.iter();
    iter.next(); // Skipping the first because it's always "0".
    let mut last = 0;
    print!("|");
    while let Some(&custom_factor_index) = iter.next() {
        print!("{}|", " ".repeat((custom_factor_index - last) * 4 - 1));
        last = custom_factor_index;
    }
    print!("{}|  ", " ".repeat((str_length - last) * 4 - 1));
}
