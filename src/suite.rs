use crate::alg::avg_output::PTSacaAverageOutput;
use crate::alg::console_logger::ConsoleLogger;
use crate::alg::executor::compute_ptsaca;
use crate::alg::file_logger::FileLogger;
use crate::alg::ptsaca::{get_phases_duration_from_execution_timing, print_ptsaca_durations};
use crate::files::fasta::get_fasta_content;
use crate::files::paths::get_path_in_generated_folder;
use crate::suffix_array::classic_suffix_array::compute_classic_suffix_array;

pub fn only_compute(
    fasta_file_name: &str,
    chunk_size: usize,
    file_logger: FileLogger,
    console_logger: ConsoleLogger,
) {
    println!("\n\nCOMPUTING SUITE ON FILE: \"{}\"\n", fasta_file_name);

    // READING FILE
    let str = &get_fasta_content(fasta_file_name).expect("Unable to open source file");

    // EXECUTION
    let (suffix_array, execution_info) =
        compute_ptsaca(file_logger, console_logger, str, Some(chunk_size));

    // PRINTING DURATIONS
    let phases_durations =
        get_phases_duration_from_execution_timing(&execution_info.execution_timing);
    print_ptsaca_durations(chunk_size, phases_durations);
}

// SUITE COMPLETE FOR CLASSIC VS INNOVATIVE COMPUTATION
pub fn full_suite(
    fasta_file_name: &str,
    chunk_size_vec: &Vec<Option<usize>>,
    max_duration_in_micros: u32,
    num_attempts: usize,
    log_execution: bool,
    log_fact: bool,
    log_trees_and_suffix_array: bool,
    draw_plot: bool,
) {
    println!("\n\nCOMPUTING SUITE ON FILE: \"{}\"\n", fasta_file_name);

    let verbose = cfg!(feature = "verbose");

    // READING FILE
    let str = &get_fasta_content(&get_path_in_generated_folder(fasta_file_name))
        .expect("Unable to open source file");

    // AVG. OUTPUT
    let mut avg_output = PTSacaAverageOutput::new(num_attempts);

    // MULTIPLE ATTEMPTS
    for i_attempt in 1..=num_attempts {
        println!(" > NUM ATTEMPT: {}/{}", i_attempt, num_attempts);

        // CLASSIC SACA
        let csa_result = compute_classic_suffix_array(str);
        avg_output.add_classic_saca_duration(csa_result.duration.as_micros() as u64);

        // PTSACA EXECUTIONS
        let mut i = 0;
        for &chunk_size in chunk_size_vec {
            // EXECUTION
            let file_logger = FileLogger::new_from_flags(
                fasta_file_name,
                chunk_size,
                log_execution,
                log_fact,
                log_trees_and_suffix_array,
                log_trees_and_suffix_array,
            );
            let console_logger = ConsoleLogger::new(verbose);
            let (suffix_array, execution_info) =
                compute_ptsaca(file_logger, console_logger, str, chunk_size);

            // VERIFICATION
            if csa_result.verify_saca(suffix_array) {
                break;
            }

            // UPDATE AVG. OUTPUT DATA
            avg_output.add_ptsaca_phase_durations(i, chunk_size, &execution_info.execution_timing);
            i += 1;
        }
    }

    // CALCULATING MEANS AND PRINTING
    avg_output.print(draw_plot, fasta_file_name, max_duration_in_micros);
}
