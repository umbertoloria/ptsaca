use crate::factorization::custom_factorization::get_custom_factors_and_more_using_chunk_size;
use crate::factorization::icfl::get_icfl_indexes;
use crate::prefix_tree::monitor::{ExecutionTiming, Monitor};
use crate::prefix_tree::tree::{create_tree, Tree};
use std::time::Duration;

pub struct PTSaca {
    pub str_chars: Vec<char>,
    pub chunk_size: Option<usize>,
    pub icfl_indexes: Vec<usize>,
    pub factor_indexes: Vec<usize>,
    pub idx_to_is_custom: Vec<bool>,
    pub idx_to_icfl_factor: Vec<usize>,
    pub tree: Tree,
    pub suffix_array: Vec<usize>,
}
impl PTSaca {
    pub fn new(chunk_size: Option<usize>) -> Self {
        Self {
            str_chars: Vec::new(),
            chunk_size,
            icfl_indexes: Vec::new(),
            factor_indexes: Vec::new(),
            idx_to_is_custom: Vec::new(),
            idx_to_icfl_factor: Vec::new(),
            tree: Tree::new(),
            suffix_array: Vec::new(),
        }
    }

    pub fn p1_factorization(&mut self, str: &str) {
        // ICFL Factorization
        let str_chars = str.chars().collect::<Vec<_>>();
        let icfl_indexes = get_icfl_indexes(&str_chars);
        // Custom Factorization
        let (
            //
            factor_indexes,
            idx_to_is_custom,
            idx_to_icfl_factor,
        ) = get_custom_factors_and_more_using_chunk_size(&icfl_indexes, self.chunk_size, str.len());
        self.str_chars = str_chars;
        self.icfl_indexes = icfl_indexes;
        self.factor_indexes = factor_indexes;
        self.idx_to_is_custom = idx_to_is_custom;
        self.idx_to_icfl_factor = idx_to_icfl_factor;
    }

    pub fn p2_tree(&mut self, monitor: &mut Monitor) {
        self.tree = create_tree(
            &self.str_chars,
            &self.factor_indexes,
            &self.icfl_indexes,
            &self.idx_to_is_custom,
            monitor,
        );
    }

    pub fn p3_suffix_array(&mut self, str: &str, monitor: &mut Monitor) {
        self.suffix_array = self.tree.compute_suffix_array(
            str,
            &self.icfl_indexes,
            &self.idx_to_is_custom,
            &self.idx_to_icfl_factor,
            monitor,
        );
    }
}

pub type PhasesDurations = (u64, u64, u64);
pub fn print_ptsaca_durations(chunk_size_or_zero: usize, micros: PhasesDurations) {
    println!("[CHUNK SIZE={chunk_size_or_zero}]");
    print_duration(" > Phase 1: Factorization ", micros.0);
    print_duration(" > Phase 2: Prefix Tree   ", micros.1);
    print_duration(" > Phase 3: Suffix Array  ", micros.2);
}

pub fn get_phases_duration_from_execution_timing(et: &ExecutionTiming) -> PhasesDurations {
    let p1_duration = et.p1_fact.dur.as_micros() as u64;
    let p2_duration = et.p2_tree.dur.as_micros() as u64;
    let p3_duration = et.p3_sa.dur.as_micros() as u64;
    (
        //
        p1_duration,
        p2_duration,
        p3_duration,
    )
}

pub fn print_duration(prefix: &str, micros: u64) {
    let duration = Duration::from_micros(micros);
    println!(
        "{}: {:10} micros / {:10.3} seconds",
        prefix,
        duration.as_micros(),
        duration.as_secs_f64()
    );
}
