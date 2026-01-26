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
use std::fs::{File, OpenOptions};

pub struct PTSacaExecutor {
    project_factorization_file: Option<File>,
    project_mini_tree_file: Option<File>,
    project_suffix_array_file: Option<File>,
    project_outcome_file_json: Option<File>,
    project_timing_file_json: Option<File>,
    verbose: bool,
}
impl PTSacaExecutor {
    pub fn new(
        project_factorization_file: Option<File>,
        project_mini_tree_file: Option<File>,
        project_suffix_array_file: Option<File>,
        project_outcome_file_json: Option<File>,
        project_timing_file_json: Option<File>,
        verbose: bool,
    ) -> Self {
        Self {
            project_factorization_file,
            project_mini_tree_file,
            project_suffix_array_file,
            project_outcome_file_json,
            project_timing_file_json,
            verbose,
        }
    }
    pub fn new_from_flags(
        fasta_file_name: &str,
        chunk_size: Option<usize>,
        log_fact: bool,
        log_trees: bool,
        log_suffix_array: bool,
        log_execution: bool,
        verbose: bool,
    ) -> Self {
        let chunk_size_or_zero = chunk_size.unwrap_or(0);
        let project_folder = get_path_for_project_folder(fasta_file_name);
        make_sure_directory_exist(&project_folder);

        let project_factorization_file = if log_fact {
            Some(
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(get_path_for_project_factorization_file(
                        fasta_file_name,
                        chunk_size_or_zero,
                    ))
                    .unwrap(),
            )
        } else {
            None
        };
        let project_mini_tree_file = if log_trees {
            Some(
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(get_path_for_project_mini_tree_file(
                        fasta_file_name,
                        chunk_size_or_zero,
                    ))
                    .unwrap(),
            )
        } else {
            None
        };
        let project_suffix_array_file = if log_suffix_array {
            Some(
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(get_path_for_project_suffix_array_file(
                        fasta_file_name,
                        chunk_size_or_zero,
                    ))
                    .unwrap(),
            )
        } else {
            None
        };
        let project_outcome_file_json = if log_execution {
            Some(
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(get_path_for_project_outcome_file_json(
                        fasta_file_name,
                        chunk_size_or_zero,
                    ))
                    .unwrap(),
            )
        } else {
            None
        };
        let project_timing_file_json = if log_execution {
            Some(
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(get_path_for_project_timing_file_json(
                        fasta_file_name,
                        chunk_size_or_zero,
                    ))
                    .unwrap(),
            )
        } else {
            None
        };
        Self::new(
            project_factorization_file,
            project_mini_tree_file,
            project_suffix_array_file,
            project_outcome_file_json,
            project_timing_file_json,
            verbose,
        )
    }

    fn log_fact(&mut self, ptsaca: &PTSaca, str: &str) {
        if let Some(project_factorization_file) = &mut self.project_factorization_file {
            log_factorization(
                &ptsaca.factor_indexes,
                &ptsaca.icfl_indexes,
                str,
                project_factorization_file,
            );
        }
    }
    fn log_trees(&mut self, ptsaca: &PTSaca) {
        if let Some(project_mini_tree_file) = &mut self.project_mini_tree_file {
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
                &ptsaca.tree,
                TreeLogMode::MiniTree,
                project_mini_tree_file,
                &ptsaca.str_chars,
            );
        }
    }
    fn log_suffix_array(&mut self, ptsaca: &PTSaca) {
        if let Some(project_suffix_array_file) = &mut self.project_suffix_array_file {
            log_suffix_array(
                //
                &ptsaca.suffix_array,
                project_suffix_array_file,
            );
        }
    }
    fn log_execution(&mut self, execution_info: &ExecutionInfo) {
        if let Some(project_outcome_file_json) = &mut self.project_outcome_file_json {
            // Execution Outcome JSON file
            let execution_outcome_file_format =
                ExecutionOutcomeFileFormat::new(&execution_info.execution_outcome);
            dump_json_in_file(&execution_outcome_file_format, project_outcome_file_json);
        }

        if let Some(project_timing_file_json) = &mut self.project_timing_file_json {
            // Execution Timing JSON file
            let execution_timing_file_format =
                ExecutionInfoFileFormat::new(&execution_info.execution_timing);
            dump_json_in_file(
                //
                &execution_timing_file_format,
                project_timing_file_json,
            );
        }
    }

    fn verbose_print_debug_before(&self, ptsaca: &PTSaca, str: &str) {
        if self.verbose {
            println!("Before SUFFIX ARRAY PHASE");
            print_for_human_like_debug(
                str,
                &ptsaca.icfl_indexes,
                &ptsaca.factor_indexes,
                &ptsaca.idx_to_icfl_factor,
                &ptsaca.idx_to_is_custom,
            );
            ptsaca.tree.print(&ptsaca.str_chars);
        }
    }
    fn print_debug_after(&self, ptsaca: &PTSaca) {
        if self.verbose {
            println!("After SUFFIX ARRAY PHASE");
            ptsaca.tree.print(&ptsaca.str_chars);
        }
    }
}

pub struct PTSaca {
    str_chars: Vec<char>,
    icfl_indexes: Vec<usize>,
    factor_indexes: Vec<usize>,
    idx_to_is_custom: Vec<bool>,
    idx_to_icfl_factor: Vec<usize>,
    tree: Tree,
    suffix_array: Vec<usize>,
}
impl PTSaca {
    fn new() -> Self {
        Self {
            str_chars: Vec::new(),
            icfl_indexes: Vec::new(),
            factor_indexes: Vec::new(),
            idx_to_is_custom: Vec::new(),
            idx_to_icfl_factor: Vec::new(),
            tree: Tree::new(),
            suffix_array: Vec::new(),
        }
    }

    fn p1_factorization(&mut self, str: &str, chunk_size: Option<usize>) {
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
        self.str_chars = str_chars;
        self.icfl_indexes = icfl_indexes;
        self.factor_indexes = factor_indexes;
        self.idx_to_is_custom = idx_to_is_custom;
        self.idx_to_icfl_factor = idx_to_icfl_factor;
    }
    fn p2_tree(&mut self, monitor: &mut Monitor) {
        self.tree = create_tree(
            &self.str_chars,
            &self.factor_indexes,
            &self.icfl_indexes,
            &self.idx_to_is_custom,
            monitor,
        );
    }
    fn p3_suffix_array(&mut self, str: &str, monitor: &mut Monitor) {
        self.suffix_array = self.tree.compute_suffix_array(
            str,
            &self.icfl_indexes,
            &self.idx_to_is_custom,
            &self.idx_to_icfl_factor,
            monitor,
        );
    }
}

pub fn compute_ptsaca(
    mut executor: PTSacaExecutor,
    str: &str,
    chunk_size: Option<usize>,
) -> (Vec<usize>, ExecutionInfo) {
    let mut instance = PTSaca::new();

    let mut monitor = Monitor::new();
    monitor.whole_duration.start();
    monitor.p1_fact.start();

    // --- PHASE 1 ---
    instance.p1_factorization(str, chunk_size);

    monitor.p1_fact.stop();
    executor.log_fact(&instance, str);
    monitor.p2_tree.start();

    // --- PHASE 2 ---
    instance.p2_tree(&mut monitor);

    monitor.p2_tree.stop();
    executor.verbose_print_debug_before(&instance, str);
    executor.log_trees(&instance);
    monitor.p3_sa.start();

    // --- PHASE 3 ---
    instance.p3_suffix_array(str, &mut monitor);

    monitor.p3_sa.stop();
    monitor.whole_duration.stop();
    executor.print_debug_after(&instance);
    executor.log_suffix_array(&instance);
    let execution_info = monitor.transform_info_execution_info();
    executor.log_execution(&execution_info);

    (
        //
        instance.suffix_array,
        execution_info,
    )
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
