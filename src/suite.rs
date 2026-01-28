use crate::alg::executor::{compute_ptsaca, PTSacaOutputBuffer};
use crate::files::fasta::get_fasta_content;
use crate::files::paths::get_path_in_generated_folder;
use crate::plot::plot::draw_plot_from_monitor;
use crate::prefix_tree::monitor::ExecutionTiming;
use crate::suffix_array::classic_suffix_array::compute_classic_suffix_array;
use std::fs::File;
use std::time::Duration;

pub fn only_compute(
    fasta_file_name: &str,
    chunk_size: usize,
    project_factorization_file: Option<File>,
    project_mini_tree_file: Option<File>,
    project_suffix_array_file: Option<File>,
    project_outcome_file_json: Option<File>,
    project_timing_file_json: Option<File>,
    verbose: bool,
) {
    println!("\n\nCOMPUTING SUITE ON FILE: \"{}\"\n", fasta_file_name);

    // READING FILE
    let str = &get_fasta_content(fasta_file_name).expect("Unable to open source file");

    // SUMS FOR MEAN
    let mut sum_innovative_micros_vec = Vec::new();
    sum_innovative_micros_vec.push((0, 0, 0));

    // INNOVATIVE SUFFIX ARRAY
    let mut i = 0;

    let executor = PTSacaOutputBuffer::new(
        project_factorization_file,
        project_mini_tree_file,
        project_suffix_array_file,
        project_outcome_file_json,
        project_timing_file_json,
        verbose,
    );
    let (suffix_array, execution_info) = compute_ptsaca(executor, str, Some(chunk_size));

    let et = &execution_info.execution_timing;
    sum_innovative_micros_vec[i].0 += et.p1_fact.dur.as_micros() as u64;
    sum_innovative_micros_vec[i].1 += et.p2_tree.dur.as_micros() as u64;
    sum_innovative_micros_vec[i].2 += et.p3_sa.dur.as_micros() as u64;
    i += 1;

    // CALCULATING MEANS AND PRINTING
    println!("INNOVATIVE SUFFIX ARRAY CALCULATION");
    let mut chunk_size_and_phase_micros_list = Vec::new();
    let mut i = 0;

    let sum_micros = &sum_innovative_micros_vec[i];
    let micros = (
        (sum_micros.0 as f32) as u64,
        (sum_micros.1 as f32) as u64,
        (sum_micros.2 as f32) as u64,
    );
    let chunk_size_or_zero = chunk_size;
    println!("[CHUNK SIZE={chunk_size_or_zero}]");
    print_duration(" > Phase 1: Factorization ", micros.0);
    print_duration(" > Phase 2: Prefix Tree   ", micros.1);
    print_duration(" > Phase 3: Suffix Array  ", micros.2);
    chunk_size_and_phase_micros_list.push((chunk_size_or_zero, micros));
    i += 1;

    // PLOT
    /*
    // TODO: Enable plots
    if draw_plot {
        draw_plot_from_monitor(
            fasta_file_name,
            mean_classic_micros,
            chunk_size_and_phase_micros_list,
            max_duration_in_micros,
        );
    }
    */
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

    // SUMS FOR MEAN
    let mut avg_output = PTSacaAverageOutput::new();

    // MULTIPLE ATTEMPTS
    for i_attempt in 1..=num_attempts {
        println!(" > NUM ATTEMPT: {}/{}", i_attempt, num_attempts);

        // CLASSIC SACA
        let csa_result = compute_classic_suffix_array(str);
        avg_output.add_classic_saca_duration(csa_result.duration.as_micros() as u64);

        // PTSACA EXECUTIONS
        for &chunk_size in chunk_size_vec {
            // EXECUTION
            let output_buffer = PTSacaOutputBuffer::new_from_flags(
                fasta_file_name,
                chunk_size,
                log_execution,
                log_fact,
                log_trees_and_suffix_array,
                log_trees_and_suffix_array,
                verbose,
            );
            let (suffix_array, execution_info) = compute_ptsaca(output_buffer, str, chunk_size);

            // VERIFICATION
            if csa_result.verify_saca(suffix_array) {
                break;
            }

            // UPDATE AVG. DATA
            avg_output.add_ptsaca_phase_durations(chunk_size, &execution_info.execution_timing);
        }
    }

    // CALCULATING MEANS AND PRINTING
    avg_output.print(draw_plot, fasta_file_name, max_duration_in_micros);
}

fn print_duration(prefix: &str, micros: u64) {
    let duration = Duration::from_micros(micros);
    println!(
        "{}: {:10} micros / {:10.3} seconds",
        prefix,
        duration.as_micros(),
        duration.as_secs_f64()
    );
}

struct PTSacaAverageOutput {
    sum_classic_saca_micros: u64,
    ptsaca_executions_durations: Vec<(Option<usize>, u64, u64, u64)>,
}
impl PTSacaAverageOutput {
    pub fn new() -> Self {
        Self {
            sum_classic_saca_micros: 0,
            ptsaca_executions_durations: Vec::new(),
        }
    }
    pub fn add_classic_saca_duration(&mut self, micros: u64) {
        self.sum_classic_saca_micros += micros;
    }
    pub fn add_ptsaca_phase_durations(&mut self, chunk_size: Option<usize>, et: &ExecutionTiming) {
        let p1_duration = et.p1_fact.dur.as_micros() as u64;
        let p2_duration = et.p2_tree.dur.as_micros() as u64;
        let p3_duration = et.p3_sa.dur.as_micros() as u64;
        self.ptsaca_executions_durations.push((
            //
            chunk_size,
            p1_duration,
            p2_duration,
            p3_duration,
        ));
    }
    pub fn print(&self, draw_plot: bool, fasta_file_name: &str, max_duration_in_micros: u32) {
        // CONSOLE
        let num_attempts = self.ptsaca_executions_durations.len();
        println!("CLASSIC SUFFIX ARRAY CALCULATION");
        let mean_classic_micros =
            (self.sum_classic_saca_micros as f32 / num_attempts as f32) as u64;
        print_duration(" > Sorting GSs duration   ", mean_classic_micros);
        println!("INNOVATIVE SUFFIX ARRAY CALCULATION");
        let mut chunk_size_and_phase_micros_list = Vec::new();
        for (chunk_size, p1_duration, p2_duration, p3_duration) in &self.ptsaca_executions_durations
        {
            let micros = (
                (*p1_duration as f32 / num_attempts as f32) as u64,
                (*p2_duration as f32 / num_attempts as f32) as u64,
                (*p3_duration as f32 / num_attempts as f32) as u64,
            );
            let chunk_size_or_zero = chunk_size.unwrap_or(0);
            println!("[CHUNK SIZE={chunk_size_or_zero}]");
            print_duration(" > Phase 1: Factorization ", micros.0);
            print_duration(" > Phase 2: Prefix Tree   ", micros.1);
            print_duration(" > Phase 3: Suffix Array  ", micros.2);
            chunk_size_and_phase_micros_list.push((chunk_size_or_zero, micros));
        }

        // PLOT
        if draw_plot {
            draw_plot_from_monitor(
                fasta_file_name,
                mean_classic_micros,
                chunk_size_and_phase_micros_list,
                max_duration_in_micros,
            );
        }
    }
}
