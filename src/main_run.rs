use crate::alg::console_logger::ConsoleLogger;
use crate::alg::file_logger::FileLogger;
use crate::suite::only_compute;
use std::fs::File;
use std::path::{Path, PathBuf};

pub fn main_run_command(src_path: &PathBuf, chunk_size: &usize, out_path: &PathBuf) {
    let fasta_file_name = src_path.to_str().expect("Unable to read source file path");

    // TODO: Don't use this way anymore
    let verbose = cfg!(feature = "verbose");

    if let Some(p) = Path::new(out_path).parent() {
        if !p.exists() && !p.as_os_str().is_empty() {
            panic!("Unable to open destination file");
        }
    }
    let out_sa_file = File::create(out_path).expect("Unable to open destination file");

    let file_logger = FileLogger::new(
        //
        None,
        None,
        Some(out_sa_file),
        None,
        None,
    );
    let console_logger = ConsoleLogger::new(verbose);
    only_compute(fasta_file_name, *chunk_size, file_logger, console_logger);
}
