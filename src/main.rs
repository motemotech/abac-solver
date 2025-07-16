use z3::{Config, Context, Solver, Sort, FuncDecl, Symbol, DatatypeBuilder, ast::{Ast, Dynamic}};
use std::collections::HashMap;
use std::fs;
use indicatif::{ProgressBar, ProgressStyle, MultiProgress, ProgressState, ProgressFinish};
use console::Term;
use std::time::Duration;
use std::thread;

mod university_types;
mod parser;
mod abac_solver;
mod edocumentTypes;
mod edocumentParser;
mod edocument_abac_solver;

use crate::university_types::UniversityAbacData;
use parser::{UniversityAbacParser, ParseError};
use abac_solver::{UniversityAbacSolver, AccessResult, RuleAccessResults};
use edocumentTypes::{
    EdocumentAbacData, EdocumentUserAttribute, EdocumentResourceAttribute, EdocumentRule,
    Role, Position as EdocumentPosition, Tenant, Department as EdocumentDepartment, Office, DocumentType, Action as EdocumentAction
};
use edocumentParser::{EdocumentAbacParser, ParseError as EdocumentParseError};
use edocument_abac_solver::{EdocumentAbacSolver, EdocumentAccessResult};

/// プログレスバーを作成するヘルパー関数
fn create_progress_bar(len: u64, message: &str) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏  ")
    );
    pb.set_message(message.to_string());
    pb
}

/// スピナーを作成するヘルパー関数
fn create_spinner(message: &str) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} [{elapsed_precise}] {msg}")
            .unwrap()
    );
    spinner.set_message(message.to_string());
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner
}

/// Z3設定を最適化して作成する
fn create_optimized_z3_context() -> Context {
    let mut cfg = Config::new();
    
    // 環境変数でメモリ制限を設定可能にする
    let memory_limit = std::env::var("Z3_MEMORY_LIMIT")
        .unwrap_or_else(|_| "16384".to_string());
    let memory_high_watermark = std::env::var("Z3_MEMORY_HIGH_WATERMARK")
        .unwrap_or_else(|_| "12288".to_string());
    let timeout = std::env::var("Z3_TIMEOUT")
        .unwrap_or_else(|_| "1800000".to_string());
    
    // メモリ制限を設定
    cfg.set_param_value("memory_max_size", &memory_limit);
    cfg.set_param_value("memory_high_watermark_mb", &memory_high_watermark);
    cfg.set_param_value("sat.max_memory", &memory_limit);
    
    // タイムアウトを設定
    cfg.set_param_value("timeout", &timeout);
    cfg.set_param_value("sat.timeout", &timeout);
    
    // より多くのメモリ割り当てを許可
    cfg.set_param_value("memory_max_alloc_count", "0"); // 無制限
    
    // SAT solver の設定を最適化
    cfg.set_param_value("sat.restart", "ema");
    cfg.set_param_value("sat.variable_decay", "120");
    cfg.set_param_value("sat.random_freq", "0.02");
    
    // その他の最適化
    cfg.set_param_value("auto_config", "false");
    cfg.set_param_value("smt.arith.auto_config_simplex", "true");
    cfg.set_param_value("smt.case_split", "3");
    
    println!("Z3 optimized configuration set:");
    println!("- Memory max size: {}MB", memory_limit);
    println!("- Memory high watermark: {}MB", memory_high_watermark);
    println!("- SAT solver max memory: {}MB", memory_limit);
    println!("- Timeout: {}ms", timeout);
    println!("- SAT solver restart strategy: ema");
    println!("- Variable decay: 120");
    println!();
    println!("You can customize these settings with environment variables:");
    println!("- Z3_MEMORY_LIMIT: Memory limit in MB (default: 16384)");
    println!("- Z3_MEMORY_HIGH_WATERMARK: Memory high watermark in MB (default: 12288)");
    println!("- Z3_TIMEOUT: Timeout in milliseconds (default: 1800000)");
    println!();
    
    Context::new(&cfg)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    println!("=== .abacファイルをパースします ===");
    let parser = UniversityAbacParser::new();
    let parsed_abac = parser.parse_file("data/university.abac")?;
    
    println!("=== パース結果をJSONに出力中... ===");
    output_to_json(parsed_abac)?;

    println!("=== パース結果からloopを使用して検証する ===");
    

    println!("=== パース結果をもとにZ3におけるconstraintsを生成 ===");

    println!("=== Z3による検証を実行 ===");
    // TODO: 実行時間を測定する

    Ok(())
}

fn output_to_json(parsed_abac: UniversityAbacData) -> Result<(), Box<dyn std::error::Error>> {
    let json_string = serde_json::to_string_pretty(&parsed_abac)?;
    let output_file = "output/parsed_abac.json";
    std::fs::write(output_file, json_string)?;
    println!("=== 出力完了: {} ===", output_file);
    Ok(())
}
