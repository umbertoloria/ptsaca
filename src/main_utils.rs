use crate::extra::suites::generation::main_generation;
use crate::factorization::cfl::cfl;
use crate::factorization::icfl::icfl;
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::process;

// MAIN GENERATE FASTA FILE
pub fn main_generate_fasta_file(length: &u64, path: &&PathBuf) {
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

// MAIN FACTORIZATION: CFL
pub fn main_factorization_cfl(src: &String) {
    // let src = "AAABCAABCADCAABCA";
    println!("CFL({})", src);
    let factors = cfl(src);
    for factor in factors {
        println!("{}", factor);
    }
    println!();
}

// MAIN FACTORIZATION: ICFL
pub fn main_factorization_icfl(src: &String) {
    // let src = "AAABCAABCADCAABCA";
    println!("ICFL({})", src);
    let factors = icfl(src);
    for factor in factors {
        println!("{}", factor);
    }
    println!();
}
