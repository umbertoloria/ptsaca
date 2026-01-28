use crate::alg::ptsaca::{
    get_phases_duration_from_execution_timing, print_duration, print_ptsaca_durations,
    PTSacaPhasesDurations,
};
use crate::plot::plot::draw_plot_from_monitor;
use crate::prefix_tree::monitor::ExecutionTiming;

pub struct PTSacaAverageOutput {
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
        let phases_durations = get_phases_duration_from_execution_timing(et);
        self.ptsaca_executions_durations.push((
            //
            chunk_size,
            phases_durations.0,
            phases_durations.1,
            phases_durations.2,
        ));
    }
    pub fn print(&self, draw_plot: bool, fasta_file_name: &str, max_duration_in_micros: u32) {
        // CONSOLE
        let num_attempts = self.ptsaca_executions_durations.len();

        println!("CLASSIC SUFFIX ARRAY CALCULATION");
        let mean_classic_micros =
            (self.sum_classic_saca_micros as f32 / num_attempts as f32) as u64;
        print_duration(" > Sorting GSs duration   ", mean_classic_micros);
        println!("PTSACA CALCULATION");

        let mut chunk_size_and_phase_micros_list = Vec::new();
        for (chunk_size, p1_duration, p2_duration, p3_duration) in &self.ptsaca_executions_durations
        {
            let micros: PTSacaPhasesDurations = (
                (*p1_duration as f32 / num_attempts as f32) as u64,
                (*p2_duration as f32 / num_attempts as f32) as u64,
                (*p3_duration as f32 / num_attempts as f32) as u64,
            );
            let chunk_size_or_zero = chunk_size.unwrap_or(0);
            print_ptsaca_durations(chunk_size_or_zero, micros);
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
