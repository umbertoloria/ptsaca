use crate::suffix_array::sorter::sort_pair_vector_of_indexed_strings;
use std::time::{Duration, Instant};

pub struct ClassicSacaResult<'a> {
    pub suffix_array: Vec<usize>,
    pub suffix_array_pairs: Vec<(usize, &'a str)>,
    pub duration: Duration,
}
impl<'a> ClassicSacaResult<'a> {
    pub fn verify_saca(&self, suffix_array: Vec<usize>) -> bool {
        {
            let mut success = true;
            if suffix_array.len() != self.suffix_array.len() {
                success = false;
                println!("Computed Suffix Array is insufficient in size");
            } else {
                let mut i = 0;
                while i < self.suffix_array.len() {
                    let clas_sa_item = self.suffix_array[i];
                    let inn_sa_item = suffix_array[i];
                    if inn_sa_item != clas_sa_item {
                        println!("Computed Suffix Array is insufficient: element [{}] should be \"{}\" but is \"{}\"", i, clas_sa_item, inn_sa_item);
                        success = false;
                    }
                    i += 1;
                }
            }
            if !success {
                println!(" > Suffix Array: {:?}", suffix_array);
                println!("Computed Suffix Array is WRONG!!! :(");
                return true;
            }
        };
        false
    }
}

pub fn compute_classic_suffix_array(src: &str) -> ClassicSacaResult {
    let before = Instant::now();

    let mut suffix_array_pairs = Vec::new();
    // Create array of global suffixes
    for i in 0..src.len() {
        suffix_array_pairs.push((i, &src[i..]));
    }
    // Create sort by comparing global suffixes
    sort_pair_vector_of_indexed_strings(&mut suffix_array_pairs);
    // Extracting only indexes of previous array of pairs
    let mut suffix_array_indexes = Vec::new();
    for &(index, _) in &suffix_array_pairs {
        suffix_array_indexes.push(index);
    }
    let after = Instant::now();
    let duration = after - before;

    if cfg!(feature = "verbose") {
        for &(index, suffix) in &suffix_array_pairs {
            println!(" > SUFFIX_ARRAY [{:3}] = {}", index, suffix);
        }
    }

    // println!("Total time: {}", duration.as_secs_f32());

    ClassicSacaResult {
        suffix_array: suffix_array_indexes,
        suffix_array_pairs,
        duration,
    }
}
