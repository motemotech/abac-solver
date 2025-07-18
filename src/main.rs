use z3::{Config, Context, Solver, Sort, FuncDecl, Symbol, DatatypeBuilder, ast::{Ast, Dynamic}};
use std::collections::HashMap;
use std::fs;
use std::env;
use std::io::{self, Write};

mod types;
mod university_types;
mod edocument_types;
mod simple_loop;

use crate::university_types::{UniversityAbacData, UniversityAbac, UniversityDomainParser};
use crate::edocument_types::{EdocumentAbacData, EdocumentAbac, EdocumentDomainParser};
use crate::types::GenericAbacParser;
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    let domain = if args.len() > 1 {
        // コマンドライン引数で指定
        match args[1].as_str() {
            "university" | "uni" => Domain::University,
            "edocument" | "edoc" => Domain::Edocument,
            _ => {
                eprintln!("不明なドメイン: {}. 利用可能: university, edocument", args[1]);
                std::process::exit(1);
            }
        }
    } else {
        // 対話式選択
        select_domain_interactive()?
    };

    println!("=== {} ドメインを使用して実行します ===", domain.name());
    
    run_analysis(domain)?;

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
            improved_simple_loop(parsed_abac);
        },
        Domain::Edocument => {
            let parser = GenericAbacParser::new(EdocumentDomainParser);
            let parsed_abac = parser.parse_file(domain.file_path())?;
            
            println!("=== パース結果をJSONに出力中... ===");
            let parsed_abac_copy = parsed_abac.clone();
            output_to_json(parsed_abac_copy, domain)?;

            println!("=== 詳細分析を実行します ===");
            improved_simple_loop(parsed_abac);
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
