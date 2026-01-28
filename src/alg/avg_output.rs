use crate::plot::plot::draw_plot_from_monitor;
use crate::prefix_tree::monitor::ExecutionTiming;
use std::time::Duration;

pub struct PTSacaAverageOutput {
    sum_classic_saca_micros: Option<u64>,
    ptsaca_executions_durations: Vec<(Option<usize>, u64, u64, u64)>,
}
impl PTSacaAverageOutput {
    pub fn new() -> Self {
        Self {
            sum_classic_saca_micros: None,
            ptsaca_executions_durations: Vec::new(),
        }
    }
    pub fn add_classic_saca_duration(&mut self, micros: u64) {
        match &mut self.sum_classic_saca_micros {
            Some(sum) => {
                *sum += micros;
            }
            None => {
                self.sum_classic_saca_micros = Some(micros);
            }
        }
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

        let mut mean_classic_micros = 0; // TODO: Improve this
        if let Some(sum_classic_saca_micros) = self.sum_classic_saca_micros {
            println!("CLASSIC SUFFIX ARRAY CALCULATION");
            mean_classic_micros = (sum_classic_saca_micros as f32 / num_attempts as f32) as u64;
            print_duration(" > Sorting GSs duration   ", mean_classic_micros);
            println!("PTSACA CALCULATION");
        }

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

fn print_duration(prefix: &str, micros: u64) {
    let duration = Duration::from_micros(micros);
    println!(
        "{}: {:10} micros / {:10.3} seconds",
        prefix,
        duration.as_micros(),
        duration.as_secs_f64()
    );
}
