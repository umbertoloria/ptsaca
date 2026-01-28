#![allow(warnings)]

use crate::alg::console_logger::ConsoleLogger;
use crate::alg::file_logger::FileLogger;
use crate::extra::suites::generation::main_generation;
use crate::factorization::cfl::cfl;
use crate::factorization::icfl::icfl;
use crate::suite::only_compute;
use clap::{Parser, Subcommand};
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use std::process;
use suite::full_suite;

mod alg;
mod extra;
mod factorization;
mod files;
mod plot;
mod prefix_tree;
mod suffix_array;
mod suite;

#[derive(Parser)]
#[command(name = "ptsaca")]
#[command(about = "A CLI tool for generation and execution of PTSACA", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generates a Fasta file containing a random genome of given length on ACGT alphabet
    GenFf {
        /// A positive number (e.g., 700000)
        #[arg(value_parser = clap::value_parser!(u64).range(0..))]
        length: u64,

        /// Path to the output file (e.g., generated/123_700.fasta)
        path: PathBuf,
    },

    /// Executes CFL
    Cfl {
        /// The string to factorize (e.g., AAABCAABCADCAABCA)
        src: String,
    },

    /// Executes ICFL
    Icfl {
        /// The string to factorize (e.g., AAABCAABCADCAABCA)
        src: String,
    },

    /// Executes PTSACA
    Run {
        /// Path of the source Fasta file (e.g., generated/002_70.fasta)
        src_path: PathBuf,

        /// Chunk size (e.g., 4)
        chunk_size: usize,

        /// Path to the output Suffix Array file (e.g., out/002_70_sa.txt)
        out_file_sa_path: PathBuf,
    },

    /// Executes the main program logic
    RunProgram,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::GenFf { length, path } => {
            let file_result = OpenOptions::new().write(true).create_new(true).open(&path);
            match file_result {
                Ok(_file) => {
                    main_generation(*length, _file);
                }
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                    eprintln!("Error: The file {:?} already exists. Stopping.", path);
                    process::exit(1);
                }
                Err(e) => {
                    eprintln!("Error: Could not create file: {}", e);
                    process::exit(1);
                }
            }
        }

        Commands::Cfl { src } => {
            // let src = "AAABCAABCADCAABCA";
            println!("CFL({})", src);
            let factors = cfl(src);
            for factor in factors {
                println!("{}", factor);
            }
            println!();
        }

        Commands::Icfl { src } => {
            // let src = "AAABCAABCADCAABCA";
            println!("ICFL({})", src);
            let factors = icfl(src);
            for factor in factors {
                println!("{}", factor);
            }
            println!();
        }

        Commands::Run {
            src_path,
            chunk_size,
            out_file_sa_path: out_path,
        } => {
            let fasta_file_name = src_path.to_str().expect("Unable to read source file path");

            // TODO: Don't use this way anymore
            let verbose = cfg!(feature = "verbose");

            if let Some(p) = Path::new(out_path).parent() {
                if !p.exists() && !p.as_os_str().is_empty() {
                    panic!("Unable to open destination file");
                }
            }
            let out_sa_file = File::create(out_path).expect("Unable to open destination file");

            let file_logger = FileLogger::new(
                //
                None,
                None,
                Some(out_sa_file),
                None,
                None,
            );
            let console_logger = ConsoleLogger::new(verbose);
            only_compute(fasta_file_name, *chunk_size, file_logger, console_logger);
        }

        Commands::RunProgram => {
            main_run_program();
        }
    }
}

fn main_run_program() {
    // Chunk Size Interval
    let chunk_size_vec_000 = create_chunk_size_interval_and_none(2, 7);
    let chunk_size_vec_001 = create_chunk_size_interval_and_none(2, 8);
    let chunk_size_vec_002m = create_chunk_size_interval_and_none(2, 24);
    let chunk_size_vec_70 = merge_chunk_size_intervals(
        create_chunk_size_interval(2, 9),
        merge_chunk_size_intervals(
            create_chunk_size_of_steps(10, 100, 10),
            merge_chunk_size_intervals(
                create_chunk_size_of_steps(100, 1_000, 100),
                merge_chunk_size_intervals(
                    create_chunk_size_of_steps(1_000, 10_000, 1_000),
                    merge_chunk_size_intervals(
                        create_chunk_size_of_steps(10_000, 56_000, 1000),
                        vec![Some(56_000), Some(56_137), None],
                    ),
                ),
            ),
        ),
    );
    let chunk_size_vec_700_1 = merge_chunk_size_intervals(
        create_chunk_size_interval(4, 9),
        merge_chunk_size_intervals(
            create_chunk_size_of_steps(10, 100, 10),
            vec![Some(100), Some(500), Some(1_000)],
        ),
    );
    let chunk_size_vec_700_2 = merge_chunk_size_intervals(
        vec![Some(2_000), Some(5_000)],
        merge_chunk_size_intervals(
            vec![Some(10_000), Some(50_000)],
            merge_chunk_size_intervals(
                create_chunk_size_of_steps(100_000, 500_000, 100_000),
                vec![Some(500_000), Some(598_865), None],
            ),
        ),
    );
    let chunk_size_vec_dna = vec![Some(6)];
    /*
    let chunk_size_vec = create_chunk_size_interval(1, 50);
    // let chunk_size_vec = create_chunk_size_interval(5, 200);
    // let chunk_size_vec = create_chunk_size_interval(5, 30);
    // let chunk_size_vec = create_chunk_size_interval(5, 5);
    // let chunk_size_vec = create_chunk_size_of_thousands_with_steps(1, 70);
    */

    // Logging?
    let le = true;
    let lf = false;
    // let lf = true;
    let lts = false;
    // let lts = true;
    let dp = true;
    // let dp = false;

    // full_suite("000", &chunk_size_vec_000, 25, 10, le, lf, lts, dp);
    // full_suite("001", &chunk_size_vec_001, 25, 10, le, lf, lts, dp);
    // full_suite("002_mini", &chunk_size_vec_002m, 30, 10, le, lf, lts, dp);
    full_suite("002_70", &chunk_size_vec_70, 200_000, 10, le, lf, false, dp);
    /*full_suite(
        "002_70",
        &vec![Some(6), Some(56_137)],
        200_000,
        1,
        false,
        true,
        true,
        false,
    );*/
    // full_suite("002_700", &chunk_size_vec_700_1, 1_600_000, 10, le, lf, false, dp);
    // full_suite("002_700", &chunk_size_vec_700_2, 30_000_000, 3, le, lf, false, dp);
    // full_suite("002_7000", &create_chunk_size_interval(5, 30), 50_000_000, 3, le, lf, false, dp);

    // DNAs
    // full_suite("dna50", &chunk_size_vec_dna, 1_000_000, 1, le, lf, lts, dp);
    // full_suite("dna10", &chunk_size_vec_dna, 1_000_000, 5, le, lf, lts, dp);
    // full_suite("dna200", &chunk_size_vec_dna, 1_000_000, 5, le, lf, lts, dp);
    // full_suite("dna400", &chunk_size_vec_dna, 1_000_000, 5, le, lf, lts, dp);
}

fn create_chunk_size_interval(min: usize, max: usize) -> Vec<Option<usize>> {
    (min..=max).map(|x| Some(x)).collect()
}

fn create_chunk_size_interval_and_none(min: usize, max: usize) -> Vec<Option<usize>> {
    let mut result = create_chunk_size_interval(min, max);
    result.push(None);
    result
}

fn create_chunk_size_of_steps(min: usize, max_excl: usize, step: usize) -> Vec<Option<usize>> {
    let mut result = Vec::new();
    let mut curr = min;
    while curr < max_excl {
        result.push(Some(curr));
        curr += step;
    }
    result
}

fn merge_chunk_size_intervals(
    a: Vec<Option<usize>>,
    mut b: Vec<Option<usize>>,
) -> Vec<Option<usize>> {
    let mut result = a;
    result.append(&mut b);
    result
}

fn create_chunk_size_of_thousands_with_steps(min: usize, max: usize) -> Vec<Option<usize>> {
    (min..=max)
        .map(|x| (x * 1_000, x * 1_000 + 250, x * 1_000 + 500, x * 1_000 + 750))
        .flat_map(|a| vec![a.0, a.1, a.2, a.3])
        .map(|x| Some(x))
        .collect()
}
