mod types;
mod parse;
mod solver;
mod loop_solver;

use z3::{Config, Context};
use clap::Parser;
use types::{Args, AbacPolicy};
use solver::solve_abac_z3;
use loop_solver::{
    solve_abac_loop, solve_abac_loop_parallel, solve_abac_loop_optimized, 
    solve_abac_loop_lookup, solve_abac_loop_bitmask, solve_abac_loop_cached
};


fn main() {
    println!("Hello, world!");
}