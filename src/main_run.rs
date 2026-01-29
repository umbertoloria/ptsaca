use crate::alg::console_logger::ConsoleLogger;
use crate::alg::executor::compute_ptsaca;
use crate::alg::file_logger::FileLogger;
use crate::files::fasta::get_fasta_content;
use std::fs::File;
use std::path::Path;

pub fn main_run_command(src_path: &Path, chunk_size: usize, out_path: &Path) {
    // Suffix Array Output file
    if let Some(p) = Path::new(out_path).parent() {
        if !p.exists() && !p.as_os_str().is_empty() {
            panic!("Unable to open destination file");
        }
    }
    let out_sa_file = File::create(out_path).expect("Unable to open destination file");

    // Compute string
    println!("\n\nCOMPUTING SUITE ON FILE: {:?}\n", src_path);
    let str = &get_fasta_content(src_path).expect("Unable to open source file");

    let mut file_logger = FileLogger::new();
    file_logger.set_file_suffix_array(out_sa_file);
    let console_logger = ConsoleLogger::new();
    let results = compute_ptsaca(str, Some(chunk_size), file_logger, console_logger);

    results.print_phases_duration();
}
