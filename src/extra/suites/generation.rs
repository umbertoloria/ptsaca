use crate::files::fasta::save_fasta_with_content;
use rand::prelude::*;
use std::fs::File;

const GENETIC_ALPHABET: [char; 4] = ['A', 'C', 'G', 'T'];

const MAX_CHARS_PER_ITERATION: i128 = 100_000;

pub fn main_generation(num_length: u64, file: File, verbose: bool) {
    let mut num_length = i128::from(num_length);

    let num_lines_max = num_length / MAX_CHARS_PER_ITERATION;

    let mut output = String::new();

    if verbose {
        println!("Generating file with {} chars", num_length);
    }

    while num_length > 0 {
        let num_chars_this_iteration = MAX_CHARS_PER_ITERATION.min(num_length);
        let mut i = 0;
        while i < num_chars_this_iteration {
            let rand_index = rand::random_range(0..GENETIC_ALPHABET.len());
            let rand_char = GENETIC_ALPHABET[rand_index];
            output.push(rand_char);

            i += 1;
            num_length -= 1;
        }
        let num_lines_left = num_length / MAX_CHARS_PER_ITERATION;

        if verbose {
            println!(
                " -> {}/{} lines: {} chars done, left {}",
                num_lines_left, num_lines_max, num_chars_this_iteration, num_length
            );
        }
    }

    save_fasta_with_content(file, output, verbose);

    if verbose {
        println!("OK!");
    }
}
