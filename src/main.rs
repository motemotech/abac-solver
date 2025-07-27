use std::env;
use std::io::{self, Write};

mod types;
mod simple_loop;
mod example_data;
mod z3_solver;

use crate::types::university_types::{UniversityAbacData, UniversityAbac, UniversityDomainParser};
use crate::types::edocument_types::{EdocumentAbacData, EdocumentAbac};
use crate::types::types::GenericAbacParser;
use simple_loop::{simple_loop, improved_simple_loop, parallel_indexed_loop};

#[derive(Debug, Clone)]
enum Domain {
    University,
    Edocument,
}

impl Domain {
    fn file_path(&self) -> &'static str {
        match self {
            Domain::University => "data/university.abac",
            Domain::Edocument => "data/edocument_10000.abac",
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Domain::University => "University",
            Domain::Edocument => "E-Document Management",
        }
    }

    fn output_filename(&self) -> &'static str {
        match self {
            Domain::University => "parsed_university.json",
            Domain::Edocument => "parsed_edocument.json",
        }
    }
}

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    solver: String,

    #[arg(short, long)]
    json_path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.solver.as_str() {
        "simple" => {
            println!("Running simple loop solver...");
            let json_content: String = std::fs::read_to_string(&args.json_path)
                .map_err(|e| format!("Failed to read JSON file {}: {}", &args.json_path, e))?;
            let parsed_abac: EdocumentAbac = serde_json::from_str(&json_content)
                .map_err(|e| format!("Failed to parse JSON from {}: {}", &args.json_path, e))?;
            parallel_indexed_loop(parsed_abac)?;
        }
        "z3" => {
            println!("Running z3 solver...");
            z3_solver::solve_real_world_scenario(&args.json_path)?;
        }
        _ => {
            eprintln!("Unknown solver: {}. Available solvers: simple, z3", args.solver);
            std::process::exit(1);
        }
    }

    Ok(())
}

fn select_domain_interactive() -> Result<Domain, Box<dyn std::error::Error>> {
    println!("=== ABAC ソルバー ===");
    println!("実行するドメインを選択してください:");
    println!("1. University (大学管理システム)");
    println!("2. E-Document Management (文書管理システム)");
    print!("選択 (1-2): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    match input.trim() {
        "1" => Ok(Domain::University),
        "2" => Ok(Domain::Edocument),
        _ => {
            println!("無効な選択です。デフォルトでUniversityを使用します。");
            Ok(Domain::University)
        }
    }
}

/// Generic function to run analysis for any domain
fn run_analysis(domain: Domain) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== {}.abacファイルをパースします ===", domain.name());
    
    match domain {
        Domain::University => {
            let parser = GenericAbacParser::new(UniversityDomainParser);
            let parsed_abac = parser.parse_file(domain.file_path())?;
            
            println!("=== パース結果をJSONに出力中... ===");
            let parsed_abac_copy = parsed_abac.clone();
            output_to_json(parsed_abac_copy, domain)?;

            println!("=== 詳細分析を実行します ===");
            parallel_indexed_loop(parsed_abac);
            // improved_simple_loop(parsed_abac);
        },
        Domain::Edocument => {
            let json_file_path = "output/edocument_with_clearance.json";
            println!("=== {} を読み込み中... ===", json_file_path);
            let json_content: String = std::fs::read_to_string(json_file_path)
                .map_err(|e| format!("Failed to read JSON file {}: {}", json_file_path, e))?;
            let parsed_abac: EdocumentAbacData = serde_json::from_str(&json_content)
                .map_err(|e| format!("Failed to parse JSON from {}: {}", json_file_path, e))?;
            
            println!("=== 詳細分析を実行します ===");
            parallel_indexed_loop(parsed_abac);
        },
    }

    Ok(())
}

/// Generic function to output any ABAC data to JSON
fn output_to_json<T: serde::Serialize>(parsed_abac: T, domain: Domain) -> Result<(), Box<dyn std::error::Error>> {
    let json_string = serde_json::to_string_pretty(&parsed_abac)?;
    let output_file = format!("output/{}", domain.output_filename());
    std::fs::create_dir_all("output")?;
    std::fs::write(&output_file, json_string)?;
    println!("=== 出力完了: {} ===", output_file);
    Ok(())
}
