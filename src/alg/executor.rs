use crate::alg::console_logger::ConsoleLogger;
use crate::alg::file_logger::FileLogger;
use crate::alg::ptsaca::{
    get_phases_duration_from_execution_timing, print_ptsaca_durations, PTSaca,
};
use crate::prefix_tree::monitor::{ExecutionInfo, Monitor};

pub fn compute_ptsaca(
    str: &str,
    chunk_size: Option<usize>,
    mut file_logger: FileLogger,
    console_logger: ConsoleLogger,
) -> PTSacaResults {
    let mut ptsaca = PTSaca::new(chunk_size);

    let mut monitor = Monitor::new();
    monitor.whole_duration.start();
    monitor.p1_fact.start();

    // --- PHASE 1 ---
    ptsaca.p1_factorization(str);

    monitor.p1_fact.stop();

    file_logger.log_fact(&ptsaca, str);

    monitor.p2_tree.start();

    // --- PHASE 2 ---
    ptsaca.p2_tree(&mut monitor);

    monitor.p2_tree.stop();

    console_logger.log_p3_before(&ptsaca, str);
    file_logger.log_trees(&ptsaca);

    monitor.p3_sa.start();

    // --- PHASE 3 ---
    ptsaca.p3_suffix_array(str, &mut monitor);

    monitor.p3_sa.stop();
    monitor.whole_duration.stop();

    console_logger.log_p3_after(&ptsaca);
    file_logger.log_suffix_array(&ptsaca);

    let execution_info = monitor.transform_info_execution_info();
    file_logger.log_execution(&execution_info);

    PTSacaResults {
        ptsaca,
        execution_info,
    }
}

pub struct PTSacaResults {
    pub ptsaca: PTSaca,
    pub execution_info: ExecutionInfo,
}
impl PTSacaResults {
    pub fn print_phases_duration(&self) {
        let phases_durations =
            get_phases_duration_from_execution_timing(&self.execution_info.execution_timing);
        let chunk_size_or_zero = self.ptsaca.chunk_size.unwrap_or(0);
        print_ptsaca_durations(chunk_size_or_zero, phases_durations);
    }
}
