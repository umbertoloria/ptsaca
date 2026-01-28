use crate::alg::console_logger::ConsoleLogger;
use crate::alg::file_logger::FileLogger;
use crate::alg::ptsaca::PTSaca;
use crate::prefix_tree::monitor::{ExecutionInfo, Monitor};

pub fn compute_ptsaca(
    mut file_logger: FileLogger,
    console_logger: ConsoleLogger,
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

    file_logger.log_fact(&instance, str);

    monitor.p2_tree.start();

    // --- PHASE 2 ---
    instance.p2_tree(&mut monitor);

    monitor.p2_tree.stop();

    console_logger.log_p3_before(&instance, str);
    file_logger.log_trees(&instance);

    monitor.p3_sa.start();

    // --- PHASE 3 ---
    instance.p3_suffix_array(str, &mut monitor);

    monitor.p3_sa.stop();
    monitor.whole_duration.stop();

    console_logger.log_p3_after(&instance);
    file_logger.log_suffix_array(&instance);

    let execution_info = monitor.transform_info_execution_info();
    file_logger.log_execution(&execution_info);

    (
        //
        instance.suffix_array,
        execution_info,
    )
}
