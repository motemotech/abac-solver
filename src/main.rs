mod types;
mod parse;
mod solver;
mod loop_solver;

use z3::{Config, Context};
use clap::Parser;

use types::{Args, AbacPolicy};
use solver::solve_abac_z3;
use loop_solver::solve_abac_loop;

fn main() {
    let args = Args::parse();
    
    println!("🔍 ABAC Policy Analyzer - Comprehensive Benchmark Mode");
    
    // List of test files with their descriptions
    let test_files = vec![
        ("data/university.abac", "University (Small)", 10),
        ("data/edocument.abac", "E-Document (Medium)", 15),
        ("data/edocument_extended.abac", "E-Document Extended", 10),
        ("data/edocument_extended_large.abac", "E-Document Extended Large", 100),
        ("data/workforce.abac", "Workforce (Medium)", 15),
    ];
    
    println!("\n📊 Multi-Dataset Benchmark Results");
    println!("====================================");
    
    for (file_path, description, max_users) in test_files {
        println!("\n🗂️  Testing: {} - {}", description, file_path);
        
        // Parse ABAC file
        let policy = match AbacPolicy::parse_from_file(file_path) {
            Ok(p) => {
                println!("✅ Parsed {} users, {} resources, {} rules", 
                         p.users.len(), p.resources.len(), p.rules.len());
                p
            },
            Err(e) => {
                eprintln!("❌ Parse error for {}: {}", file_path, e);
                continue;
            }
        };
        
        let actual_max_users = std::cmp::min(max_users, policy.users.len());
        println!("🎯 Testing with {} users...", actual_max_users);
        
        // Run Z3-based implementation
        let ctx = Context::new(&Config::new());
        let z3_times = solve_abac_z3(&ctx, &policy, actual_max_users);
        
        // Run loop-based implementation  
        let loop_times = solve_abac_loop(&policy, actual_max_users);
        
        // Calculate and display results
        if !z3_times.is_empty() && !loop_times.is_empty() {
            let z3_avg = z3_times.iter().sum::<f64>() / z3_times.len() as f64;
            let loop_avg = loop_times.iter().sum::<f64>() / loop_times.len() as f64;
            
            println!("  🔬 Z3 Average: {:.3} ms/user", z3_avg);
            println!("  🔄 Loop Average: {:.3} ms/user", loop_avg);
            
            if loop_avg < z3_avg {
                let speedup = z3_avg / loop_avg;
                println!("  ⚡ Loop is {:.2}x faster", speedup);
            } else {
                let speedup = loop_avg / z3_avg;
                println!("  ⚡ Z3 is {:.2}x faster", speedup);
            }
        }
    }
    
    // If user specified a specific file, run detailed benchmark on it
    if args.file != "data/university.abac" {
        println!("\n🎯 Detailed Benchmark for: {}", args.file);
        println!("=====================================");
        
        let policy = match AbacPolicy::parse_from_file(&args.file) {
            Ok(p) => {
                println!("✅ Parsed {} users, {} resources, {} rules", 
                         p.users.len(), p.resources.len(), p.rules.len());
                p
            },
            Err(e) => {
                eprintln!("❌ Parse error: {}", e);
                return;
            }
        };
        
        let ctx = Context::new(&Config::new());
        let z3_times = solve_abac_z3(&ctx, &policy, args.max_users);
        let loop_times = solve_abac_loop(&policy, args.max_users);
        
        // Detailed results
        if !z3_times.is_empty() {
            let z3_avg = z3_times.iter().sum::<f64>() / z3_times.len() as f64;
            let z3_min = z3_times.iter().fold(f64::INFINITY, |a, &b| a.min(b));
            let z3_max = z3_times.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
            
            println!("\n🔬 Z3-based approach:");
            println!("  Average time per user: {:.3} ms", z3_avg);
            println!("  Min time: {:.3} ms", z3_min);
            println!("  Max time: {:.3} ms", z3_max);
            println!("  Total time: {:.3} ms", z3_times.iter().sum::<f64>());
        }
        
        if !loop_times.is_empty() {
            let loop_avg = loop_times.iter().sum::<f64>() / loop_times.len() as f64;
            let loop_min = loop_times.iter().fold(f64::INFINITY, |a, &b| a.min(b));
            let loop_max = loop_times.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
            
            println!("\n🔄 Loop-based approach:");
            println!("  Average time per user: {:.3} ms", loop_avg);
            println!("  Min time: {:.3} ms", loop_min);
            println!("  Max time: {:.3} ms", loop_max);
            println!("  Total time: {:.3} ms", loop_times.iter().sum::<f64>());
        }
        
        if !z3_times.is_empty() && !loop_times.is_empty() {
            let z3_avg = z3_times.iter().sum::<f64>() / z3_times.len() as f64;
            let loop_avg = loop_times.iter().sum::<f64>() / loop_times.len() as f64;
            
            println!("\n⚡ Performance Comparison:");
            if z3_avg < loop_avg {
                let speedup = loop_avg / z3_avg;
                println!("  Z3 is {:.2}x faster than loop approach", speedup);
            } else {
                let speedup = z3_avg / loop_avg;
                println!("  Loop is {:.2}x faster than Z3 approach", speedup);
            }
            
            println!("  Difference: {:.3} ms per user", (z3_avg - loop_avg).abs());
        }
    }
}