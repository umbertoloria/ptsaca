use crate::factorization::custom_factorization::get_custom_factors_and_more_using_chunk_size;
use crate::factorization::icfl::get_icfl_indexes;
use crate::prefix_tree::monitor::Monitor;
use crate::prefix_tree::tree::{create_tree, Tree};

pub struct PTSaca {
    pub str_chars: Vec<char>,
    pub icfl_indexes: Vec<usize>,
    pub factor_indexes: Vec<usize>,
    pub idx_to_is_custom: Vec<bool>,
    pub idx_to_icfl_factor: Vec<usize>,
    pub tree: Tree,
    pub suffix_array: Vec<usize>,
}
impl PTSaca {
    pub fn new() -> Self {
        Self {
            str_chars: Vec::new(),
            icfl_indexes: Vec::new(),
            factor_indexes: Vec::new(),
            idx_to_is_custom: Vec::new(),
            idx_to_icfl_factor: Vec::new(),
            tree: Tree::new(),
            suffix_array: Vec::new(),
        }
    }

    pub fn p1_factorization(&mut self, str: &str, chunk_size: Option<usize>) {
        // ICFL Factorization
        let str_chars = str.chars().collect::<Vec<_>>();
        let icfl_indexes = get_icfl_indexes(&str_chars);
        // Custom Factorization
        let (
            //
            factor_indexes,
            idx_to_is_custom,
            idx_to_icfl_factor,
        ) = get_custom_factors_and_more_using_chunk_size(&icfl_indexes, chunk_size, str.len());
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
