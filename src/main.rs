mod types;
mod parse;
mod solver;
mod loop_solver;

use z3::{Config, Context};
/*
 🚀 ABAC Solver High-Performance Optimization Techniques
 
 1. 🔬 Z3-based: SMT solver with true constraint reasoning
 2. 🔄 Loop-based: Simple nested loops (baseline)
 3. ⚡ Parallel: Multi-threading with Rayon (CPU cores)
 4. 🚀 Optimized: Early termination + rule efficiency ordering
 5. 🔍 Lookup: Pre-indexed attribute tables for fast filtering
 6. 🎯 Bitmask: Ultra-fast bitwise operations for attribute matching
 7. 💾 Cached: Memoization for repeated computations
 
 Performance Characteristics:
 - Z3: Excellent for complex logic, overhead for simple comparisons
 - Loop: Predictable performance, good for small datasets  
 - Parallel: Scales with CPU cores (2-8x speedup)
 - Optimized: Smart rule ordering reduces unnecessary checks
 - Lookup: Fast O(1) attribute lookups for large rule sets
 - Bitmask: CPU-optimized bitwise operations (experimental)
 - Cached: Reduces redundant computations in similar users
*/

use clap::Parser;

use types::{Args, AbacPolicy};
use solver::solve_abac_z3;
use loop_solver::{
    solve_abac_loop, solve_abac_loop_parallel, solve_abac_loop_optimized, 
    solve_abac_loop_lookup, solve_abac_loop_bitmask, solve_abac_loop_cached
};


fn main() {
    let args = Args::parse();
    
    println!("🔍 ABAC Policy Analyzer - Comprehensive Benchmark Mode");
    
    // List of test files with their descriptions
    let test_files = vec![
        // ("data/university.abac", "University (Small)"),
        // ("data/edocument.abac", "E-Document (Medium)"),
        // ("data/edocument_extended.abac", "E-Document Extended"),
        ("data/edocument_extended_large.abac", "E-Document Extended Large"),
        // ("data/workforce.abac", "Workforce (Medium)"),
    ];
    
    println!("\n📊 Multi-Dataset Benchmark Results");
    println!("====================================");
    
    for (file_path, description) in test_files {
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
        
        // Test with ALL users instead of limiting
        let max_users_to_test = policy.users.len();
        let is_small_dataset = policy.users.len() <= 50 && policy.resources.len() <= 100;
        
        println!("🎯 Testing with ALL {} users...", max_users_to_test);
        
        if is_small_dataset {
            println!("📊 Running ALL optimization techniques (small dataset)");
        } else {
            println!("📊 Running core techniques only (large dataset - advanced techniques skipped)");
        }
        
        // Run different implementations based on dataset size
        let ctx = Context::new(&Config::new());
        
        let (z3_matches, z3_time) = solve_abac_z3(&ctx, &policy, max_users_to_test);
        let (loop_matches, loop_time) = solve_abac_loop(&policy, max_users_to_test);
        let (parallel_matches, parallel_time) = solve_abac_loop_parallel(&policy, max_users_to_test);
        let (optimized_matches, optimized_time) = solve_abac_loop_optimized(&policy, max_users_to_test);
        let (lookup_matches, lookup_time) = solve_abac_loop_lookup(&policy, max_users_to_test);
        
        // Run advanced techniques only on small datasets to avoid long execution times
        let ((bitmask_matches, bitmask_time), (cached_matches, cached_time)) = if is_small_dataset {
            (
                solve_abac_loop_bitmask(&policy, max_users_to_test),
                solve_abac_loop_cached(&policy, max_users_to_test)
            )
        } else {
            ((Vec::new(), 0.0), (Vec::new(), 0.0))
        };
        
        // Display results with match counts and timing
        let all_results = vec![
            ("Z3", &z3_matches, z3_time),
            ("Loop", &loop_matches, loop_time),
            ("Parallel", &parallel_matches, parallel_time),
            ("Optimized", &optimized_matches, optimized_time),
            ("Lookup", &lookup_matches, lookup_time),
        ];
        
        // Add advanced techniques for small datasets
        let mut extended_results = all_results;
        if is_small_dataset {
            extended_results.push(("Bitmask", &bitmask_matches, bitmask_time));
            extended_results.push(("Cached", &cached_matches, cached_time));
        }
        
        for (name, matches, elapsed_time) in &extended_results {
            let icon = match *name {
                "Z3" => "🔬",
                "Loop" => "🔄",
                "Parallel" => "⚡",
                "Optimized" => "🚀",
                "Lookup" => "🔍",
                "Bitmask" => "🎯",
                "Cached" => "💾",
                _ => "📊"
            };
            println!("  {} {} Found {} matching combinations in {:.2}ms", icon, name, matches.len(), elapsed_time);
        }
        
        // Consistency check
        if !extended_results.is_empty() {
            let first_match_count = extended_results[0].1.len();
            let all_consistent = extended_results.iter().all(|(_, matches, _)| matches.len() == first_match_count);
            
            if all_consistent {
                println!("  ✅ All algorithms found consistent results ({} matches)", first_match_count);
            } else {
                println!("  ⚠️  Inconsistent results detected:");
                for (name, matches, _) in &extended_results {
                    println!("    {}: {} matches", name, matches.len());
                }
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
        let (z3_matches, z3_time) = solve_abac_z3(&ctx, &policy, policy.users.len());
        let (loop_matches, loop_time) = solve_abac_loop(&policy, policy.users.len());
        let (parallel_matches, parallel_time) = solve_abac_loop_parallel(&policy, policy.users.len());
        let (optimized_matches, optimized_time) = solve_abac_loop_optimized(&policy, policy.users.len());
        let (lookup_matches, lookup_time) = solve_abac_loop_lookup(&policy, policy.users.len());
        
        // Detailed results for all approaches
        let approaches = vec![
            ("🔬 Z3-based", &z3_matches, z3_time),
            ("🔄 Loop-based", &loop_matches, loop_time),
            ("⚡ Parallel Loop", &parallel_matches, parallel_time),
            ("🚀 Optimized Loop", &optimized_matches, optimized_time),
            ("🔍 Lookup-based", &lookup_matches, lookup_time),
        ];
        
        for (name, matches, elapsed_time) in approaches {
            println!("\n{} approach:", name);
            println!("  Total matches found: {} in {:.2}ms", matches.len(), elapsed_time);
            
            if !matches.is_empty() {
                // Show some example matches
                println!("  Example matches (first 5):");
                for (i, (user, resource, rule_idx)) in matches.iter().take(5).enumerate() {
                    println!("    {}. User '{}' -> Resource '{}' (Rule {})", i+1, user, resource, rule_idx);
                }
            }
        }
        
        // Consistency check for detailed benchmark
        let all_approaches = vec![
            ("Z3", &z3_matches),
            ("Loop", &loop_matches),
            ("Parallel", &parallel_matches),
            ("Optimized", &optimized_matches),
            ("Lookup", &lookup_matches),
        ];
        
        if !all_approaches.is_empty() {
            let first_match_count = all_approaches[0].1.len();
            let all_consistent = all_approaches.iter().all(|(_, matches)| matches.len() == first_match_count);
            
            if all_consistent {
                println!("\n✅ All algorithms found consistent results ({} matches)", first_match_count);
            } else {
                println!("\n⚠️  Inconsistent results detected:");
                for (name, matches) in &all_approaches {
                    println!("  {}: {} matches", name, matches.len());
                }
            }
        }
    }

    
    println!("\n🎉 Analysis Complete! Summary:");
    println!("================================");
    println!("📋 Combination Enumeration System: Lists all matching (user, resource, rule) combinations");
    println!("🔬 Multiple solving approaches: Z3, Loop, Parallel, Optimized, Lookup, Bitmask, Cached");
    println!("⚡ Scalability: Designed for large datasets (10k users x 10k resources)");
    println!("🚀 Consistency validation: All approaches should find identical match sets");
}