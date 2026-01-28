use crate::{
    main_factorization_cfl, main_factorization_icfl, main_generate_fasta_file, main_run,
    main_run_debug_program,
};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ptsaca")]
#[command(about = "A CLI tool for generation and execution of PTSACA", long_about = None)]
struct Cli {
    #[command(subcommand)]
    pub command: Commands,
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

pub fn cli_init() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::GenFf { length, path } => {
            main_generate_fasta_file(length, &path);
        }

        Commands::Cfl { src } => {
            main_factorization_cfl(src);
        }

        Commands::Icfl { src } => {
            main_factorization_icfl(src);
        }

        Commands::Run {
            src_path,
            chunk_size,
            out_file_sa_path,
        } => {
            main_run(src_path, chunk_size, out_file_sa_path);
        }

        Commands::RunProgram => {
            main_run_debug_program();
        }
    }
}
