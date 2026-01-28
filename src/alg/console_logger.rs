use crate::alg::ptsaca::PTSaca;

pub struct ConsoleLogger {
    verbose: bool,
}
impl ConsoleLogger {
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }
    pub fn log_p3_before(&self, instance: &PTSaca, str: &str) {
        if self.verbose {
            println!("Before SUFFIX ARRAY PHASE");
            print_for_human_like_debug(
                str,
                &instance.icfl_indexes,
                &instance.factor_indexes,
                &instance.idx_to_icfl_factor,
                &instance.idx_to_is_custom,
            );
            instance.tree.print(&instance.str_chars);
        }
    }
    pub fn log_p3_after(&self, instance: &PTSaca) {
        if self.verbose {
            println!("After SUFFIX ARRAY PHASE");
            instance.tree.print(&instance.str_chars);
        }
    }
}

fn print_for_human_like_debug(
    str: &str,
    icfl_indexes: &Vec<usize>,
    factor_indexes: &Vec<usize>,
    idx_to_icfl_factor: &Vec<usize>,
    idx_to_is_custom: &Vec<bool>,
    // depths: &Vec<usize>,
) {
    // CHAR INDEXES
    for i in 0..str.len() {
        print!(" {:2} ", i);
    }
    println!();
    // CHARS
    for i in 0..str.len() {
        print!("  {} ", &str[i..i + 1]);
    }
    println!();
    // IDX TO ICFL FACTOR
    for i in 0..str.len() {
        print!(" {:2} ", idx_to_icfl_factor[i]);
    }
    println!("   <= IDX TO ICFL FACTOR {:?}", icfl_indexes);
    let mut i = 0;

    print_indexes_list(&icfl_indexes, str.len());
    println!("<= ICFL FACTOR INDEXES {:?}", icfl_indexes);
    print_indexes_list(&factor_indexes, str.len());
    println!("<= FACTOR INDEXES {:?}", factor_indexes);

    // IDX TO IS CUSTOM FACTOR
    i = 0;
    while i < str.len() {
        print!("  {} ", if idx_to_is_custom[i] { "x" } else { " " });
        i += 1;
    }
    println!("   <= IDX TO IS CUSTOM FACTOR");
    /*for i in 0..str.len() {
        print!(" {:2} ", depths[i]);
    }
    println!("   <= DEPTHS");*/
}
fn print_indexes_list(indexes_list: &Vec<usize>, str_length: usize) {
    let mut iter = &mut indexes_list.iter();
    iter.next(); // Skipping the first because it's always "0".
    let mut last = 0;
    print!("|");
    while let Some(&custom_factor_index) = iter.next() {
        print!("{}|", " ".repeat((custom_factor_index - last) * 4 - 1));
        last = custom_factor_index;
    }
    print!("{}|  ", " ".repeat((str_length - last) * 4 - 1));
}
