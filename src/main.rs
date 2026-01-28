#![allow(warnings)]

use crate::cli::cli_init;
use clap::{Parser, Subcommand};

mod alg;
mod cli;
mod extra;
mod factorization;
mod files;
mod main_debug_program;
mod main_run;
mod main_utils;
mod plot;
mod prefix_tree;
mod suffix_array;
mod suite;

fn main() {
    cli_init();
}
