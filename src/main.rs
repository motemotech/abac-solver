use z3::{Config, Context, Solver, Sort, FuncDecl, Symbol, DatatypeBuilder, ast::{Ast, Dynamic}};
use std::collections::HashMap;
use std::fs;

mod university_types;
mod parser;

use crate::university_types::UniversityAbacData;
use parser::{UniversityAbacParser, ParseError};

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
