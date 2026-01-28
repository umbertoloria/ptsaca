use crate::alg::ptsaca::PTSaca;
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
use crate::prefix_tree::monitor::ExecutionInfo;
use crate::suffix_array::logger::{log_suffix_array, make_sure_directory_exist};
use std::fs::File;

pub struct FileLogger {
    file_factorization: Option<File>,
    file_mini_tree: Option<File>,
    file_suffix_array: Option<File>,
    file_json_outcome: Option<File>,
    file_json_timing: Option<File>,
}
impl FileLogger {
    pub fn new(
        file_factorization: Option<File>,
        file_mini_tree: Option<File>,
        file_suffix_array: Option<File>,
        file_json_outcome: Option<File>,
        file_json_timing: Option<File>,
    ) -> Self {
        Self {
            file_factorization,
            file_mini_tree,
            file_suffix_array,
            file_json_outcome,
            file_json_timing,
        }
    }
    pub fn new_from_flags(
        fasta_file_name: &str,
        chunk_size: Option<usize>,
        log_fact: bool,
        log_trees: bool,
        log_suffix_array: bool,
        log_execution: bool,
    ) -> Self {
        let chunk_size_or_zero = chunk_size.unwrap_or(0);
        let project_folder = get_path_for_project_folder(fasta_file_name);
        make_sure_directory_exist(&project_folder);

        let project_factorization_file = if log_fact {
            Some(
                File::create(get_path_for_project_factorization_file(
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
                File::create(get_path_for_project_mini_tree_file(
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
                File::create(get_path_for_project_suffix_array_file(
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
                File::create(get_path_for_project_outcome_file_json(
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
                File::create(get_path_for_project_timing_file_json(
                    fasta_file_name,
                    chunk_size_or_zero,
                ))
                .unwrap(),
            )
        } else {
            None
        };
        Self {
            file_factorization: project_factorization_file,
            file_mini_tree: project_mini_tree_file,
            file_suffix_array: project_suffix_array_file,
            file_json_outcome: project_outcome_file_json,
            file_json_timing: project_timing_file_json,
        }
    }

    pub fn log_fact(&mut self, ptsaca: &PTSaca, str: &str) {
        if let Some(project_factorization_file) = &mut self.file_factorization {
            log_factorization(
                &ptsaca.factor_indexes,
                &ptsaca.icfl_indexes,
                str,
                project_factorization_file,
            );
        }
    }
    pub fn log_trees(&mut self, ptsaca: &PTSaca) {
        if let Some(project_mini_tree_file) = &mut self.file_mini_tree {
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
    pub fn log_suffix_array(&mut self, ptsaca: &PTSaca) {
        if let Some(project_suffix_array_file) = &mut self.file_suffix_array {
            log_suffix_array(
                //
                &ptsaca.suffix_array,
                project_suffix_array_file,
            );
        }
    }
    pub fn log_execution(&mut self, execution_info: &ExecutionInfo) {
        if let Some(project_outcome_file_json) = &mut self.file_json_outcome {
            // Execution Outcome JSON file
            let execution_outcome_file_format =
                ExecutionOutcomeFileFormat::new(&execution_info.execution_outcome);
            dump_json_in_file(&execution_outcome_file_format, project_outcome_file_json);
        }

        if let Some(project_timing_file_json) = &mut self.file_json_timing {
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
}
