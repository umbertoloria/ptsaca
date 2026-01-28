use crate::alg::console_logger::ConsoleLogger;
use crate::alg::executor::compute_ptsaca;
use crate::alg::file_logger::FileLogger;
use crate::alg::ptsaca::{get_phases_duration_from_execution_timing, print_ptsaca_durations};
use crate::files::fasta::get_fasta_content;
use std::fs::File;
use std::path::{Path, PathBuf};

pub fn main_run_command(src_path: &PathBuf, chunk_size: usize, out_path: &PathBuf) {
    // Source file
    let src_fasta_fp = src_path.to_str().expect("Unable to read source file path");

    // Suffix Array Output file
    if let Some(p) = Path::new(out_path).parent() {
        if !p.exists() && !p.as_os_str().is_empty() {
            panic!("Unable to open destination file");
        }
    }
    let out_sa_file = File::create(out_path).expect("Unable to open destination file");

    // Compute string
    println!("\n\nCOMPUTING SUITE ON FILE: \"{}\"\n", src_fasta_fp);

    let str = &get_fasta_content(src_fasta_fp).expect("Unable to open source file");

    // SETUP
    let file_logger = FileLogger::new(
        //
        None,
        None,
        Some(out_sa_file),
        None,
        None,
    );
    let console_logger = ConsoleLogger::new();

    // EXECUTION
    let (suffix_array, execution_info) =
        compute_ptsaca(file_logger, console_logger, str, Some(chunk_size));

    // PRINTING DURATIONS
    let phases_durations =
        get_phases_duration_from_execution_timing(&execution_info.execution_timing);
    print_ptsaca_durations(chunk_size, phases_durations);
}
