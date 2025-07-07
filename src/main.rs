use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;
use regex::Regex;
use z3::ast::{Ast, Bool, Int};
use z3::{Config, Context, Solver, SatResult};
use clap::Parser;

#[derive(Parser)]
#[command(name = "abac-solver")]
#[command(about = "ABAC Policy Analyzer with Z3")]
struct Args {
    /// Path to the ABAC file to analyze
    #[arg(short, long, default_value = "data/university.abac")]
    file: String,
    
    /// Number of solutions to find per rule
    #[arg(short, long, default_value = "5")]
    max_solutions: usize,
    
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Debug, Clone)]
struct UserAttribute {
    name: String,
    attributes: HashMap<String, AttributeValue>,
}

#[derive(Debug, Clone)]
struct ResourceAttribute {
    name: String,
    attributes: HashMap<String, AttributeValue>,
}

#[derive(Debug, Clone)]
enum AttributeValue {
    String(String),
    StringSet(HashSet<String>),
    Boolean(bool),
}

#[derive(Debug, Clone)]
struct Rule {
    id: usize,
    raw_content: String,
    description: String,
}

#[derive(Debug)]
struct AbacPolicy {
    users: Vec<UserAttribute>,
    resources: Vec<ResourceAttribute>,
    rules: Vec<Rule>,
    filename: String,
}

impl AbacPolicy {
    fn new(filename: String) -> Self {
        AbacPolicy {
            users: Vec::new(),
            resources: Vec::new(),
            rules: Vec::new(),
            filename,
        }
    }

    fn parse_from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(filename)?;
        let mut policy = AbacPolicy::new(filename.to_string());

        // 改良された正規表現パターン
        let user_attrib_re = Regex::new(r"userAttrib\(([^,]+),\s*(.+)\)")?;
        let resource_attrib_re = Regex::new(r"resourceAttrib\(([^,]+),\s*(.+)\)")?;
        let rule_re = Regex::new(r"rule\((.+)\)")?;

        let mut rule_counter = 1;
        let mut current_rule_description = String::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // コメント行の処理（ルールの説明を抽出）
            if line.starts_with('#') {
                if line.len() > 2 && !line.contains("----") {
                    current_rule_description = line[1..].trim().to_string();
                }
                continue;
            }

            // userAttrib のパース
            if let Some(caps) = user_attrib_re.captures(line) {
                let name = caps[1].to_string();
                let attributes = Self::parse_attributes(&caps[2])?;
                policy.users.push(UserAttribute { name, attributes });
            }
            // resourceAttrib のパース
            else if let Some(caps) = resource_attrib_re.captures(line) {
                let name = caps[1].to_string();
                let attributes = Self::parse_attributes(&caps[2])?;
                policy.resources.push(ResourceAttribute { name, attributes });
            }
            // rule のパース
            else if let Some(caps) = rule_re.captures(line) {
                let rule_content = caps[1].to_string();
                policy.rules.push(Rule {
                    id: rule_counter,
                    raw_content: rule_content,
                    description: if current_rule_description.is_empty() {
                        format!("Rule {}", rule_counter)
                    } else {
                        current_rule_description.clone()
                    },
                });
                rule_counter += 1;
                current_rule_description.clear();
            }
        }

        Ok(policy)
    }

    fn parse_attributes(attr_str: &str) -> Result<HashMap<String, AttributeValue>, Box<dyn std::error::Error>> {
        let mut attributes = HashMap::new();
        
        // より単純で確実な属性パーシング
        // まず、カンマで分割してから各部分を処理
        let parts: Vec<&str> = attr_str.split(',').collect();
        
        for part in parts {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }
            
            // = で分割
            if let Some(eq_pos) = part.find('=') {
                let key = part[..eq_pos].trim().to_string();
                let value_str = part[eq_pos + 1..].trim();
                
                if value_str.starts_with('{') && value_str.ends_with('}') {
                    // セットの場合
                    let set_content = &value_str[1..value_str.len()-1];
                    let set_values: HashSet<String> = set_content
                        .split_whitespace()
                        .filter(|s| !s.is_empty())
                        .map(|s| s.to_string())
                        .collect();
                    attributes.insert(key, AttributeValue::StringSet(set_values));
                } else if value_str == "True" || value_str == "False" {
                    // ブールの場合
                    let bool_val = value_str == "True";
                    attributes.insert(key, AttributeValue::Boolean(bool_val));
                } else {
                    // 文字列の場合
                    attributes.insert(key, AttributeValue::String(value_str.to_string()));
                }
            }
        }
        
        Ok(attributes)
    }

    fn get_all_attribute_names(&self) -> HashSet<String> {
        let mut attr_names = HashSet::new();
        
        for user in &self.users {
            attr_names.extend(user.attributes.keys().cloned());
        }
        
        for resource in &self.resources {
            attr_names.extend(resource.attributes.keys().cloned());
        }
        
        attr_names
    }

    fn get_all_resource_types(&self) -> HashSet<String> {
        let mut resource_types = HashSet::new();
        
        for resource in &self.resources {
            if let Some(AttributeValue::String(r_type)) = resource.attributes.get("type") {
                resource_types.insert(r_type.clone());
            }
        }
        
        resource_types
    }
}

fn main() {
    let start_time = Instant::now();
    let args = Args::parse();
    
    println!("🔍 ABAC Policy Analyzer with Z3");
    println!("Analyzing file: {}", args.file);
    println!("⏱️  Start time: {:?}", start_time);
    
    // 1. ABACファイルのパース
    println!("\n📖 Parsing ABAC file...");
    let parse_start = Instant::now();
    let (policy, parse_duration) = match AbacPolicy::parse_from_file(&args.file) {
        Ok(p) => {
            let parse_duration = parse_start.elapsed();
            println!("✅ Successfully parsed ABAC file");
            println!("⏱️  Parse time: {:.2?}", parse_duration);
            (p, parse_duration)
        },
        Err(e) => {
            eprintln!("❌ Error parsing ABAC file: {}", e);
            return;
        }
    };

    println!("📊 Parsed {} users, {} resources, {} rules", 
             policy.users.len(), policy.resources.len(), policy.rules.len());

    // 2. ファイル別の詳細分析
    println!("\n📋 Analyzing policy structure...");
    let analysis_start = Instant::now();
    analyze_policy_structure(&policy, args.verbose);
    let analysis_duration = analysis_start.elapsed();
    println!("⏱️  Structure analysis time: {:.2?}", analysis_duration);

    // 3. Z3のセットアップ
    println!("\n🧮 Setting up Z3...");
    let z3_setup_start = Instant::now();
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let z3_setup_duration = z3_setup_start.elapsed();
    println!("⏱️  Z3 setup time: {:.2?}", z3_setup_duration);

    // 4. 汎用的なZ3分析
    println!("\n🔬 Starting Z3 Analysis...");
    let z3_analysis_start = Instant::now();
    analyze_with_z3_generic(&ctx, &policy, args.max_solutions);
    let z3_analysis_duration = z3_analysis_start.elapsed();
    println!("⏱️  Z3 analysis time: {:.2?}", z3_analysis_duration);
    
    let total_duration = start_time.elapsed();
    println!("\n✨ Analysis Complete!");
    println!("⏱️  Total execution time: {:.2?}", total_duration);
    println!("📊 Performance Summary:");
    println!("   - Parsing: {:.2?} ({:.1}%)", parse_duration, 
             parse_duration.as_secs_f64() / total_duration.as_secs_f64() * 100.0);
    println!("   - Structure analysis: {:.2?} ({:.1}%)", analysis_duration,
             analysis_duration.as_secs_f64() / total_duration.as_secs_f64() * 100.0);
    println!("   - Z3 setup: {:.2?} ({:.1}%)", z3_setup_duration,
             z3_setup_duration.as_secs_f64() / total_duration.as_secs_f64() * 100.0);
    println!("   - Z3 analysis: {:.2?} ({:.1}%)", z3_analysis_duration,
             z3_analysis_duration.as_secs_f64() / total_duration.as_secs_f64() * 100.0);
}

fn analyze_policy_structure(policy: &AbacPolicy, verbose: bool) {
    println!("\n📋 Policy Structure Analysis");
    
    // ユーザーの種類別統計
    let mut user_types = HashMap::new();
    let mut user_positions = HashMap::new();
    let mut user_departments = HashMap::new();
    
    for user in &policy.users {
        // role/position の統計
        if let Some(AttributeValue::String(pos)) = user.attributes.get("position") {
            *user_positions.entry(pos.clone()).or_insert(0) += 1;
        }
        if let Some(AttributeValue::String(role)) = user.attributes.get("role") {
            *user_types.entry(role.clone()).or_insert(0) += 1;
        }
        if let Some(AttributeValue::String(dept)) = user.attributes.get("department") {
            *user_departments.entry(dept.clone()).or_insert(0) += 1;
        }
    }
    
    // リソースの種類別統計
    let mut resource_types = HashMap::new();
    for resource in &policy.resources {
        if let Some(AttributeValue::String(r_type)) = resource.attributes.get("type") {
            *resource_types.entry(r_type.clone()).or_insert(0) += 1;
        }
    }
    
    println!("User Roles/Positions:");
    for (pos, count) in user_positions {
        println!("  {}: {}", pos, count);
    }
    
    if !user_types.is_empty() {
        println!("\nUser Types:");
        for (role, count) in user_types {
            println!("  {}: {}", role, count);
        }
    }
    
    if !user_departments.is_empty() {
        println!("\nDepartments:");
        for (dept, count) in user_departments {
            println!("  {}: {}", dept, count);
        }
    }
    
    println!("\nResource Types:");
    for (r_type, count) in resource_types {
        println!("  {}: {}", r_type, count);
    }
    
    // ルールの一覧
    println!("\nRules:");
    for rule in &policy.rules {
        println!("  Rule {}: {}", rule.id, rule.description);
        if verbose {
            println!("    Content: {}", rule.raw_content);
        }
    }

    // 全属性の一覧
    if verbose {
        let all_attrs = policy.get_all_attribute_names();
        println!("\nAll Attributes: {:?}", all_attrs);
    }
}

fn analyze_with_z3_generic(ctx: &Context, policy: &AbacPolicy, max_solutions: usize) {
    println!("🔬 Analyzing ABAC policies with Z3 constraints...");
    
    let solver = Solver::new(ctx);
    
    // 変数の作成
    let user_id = Int::new_const(ctx, "user_id");
    let resource_id = Int::new_const(ctx, "resource_id");
    let can_access = Bool::new_const(ctx, "can_access");
    
    // ドメインの制約
    let num_users = policy.users.len() as i64;
    let num_resources = policy.resources.len() as i64;
    
    solver.assert(&user_id.ge(&Int::from_i64(ctx, 0)));
    solver.assert(&user_id.lt(&Int::from_i64(ctx, num_users)));
    solver.assert(&resource_id.ge(&Int::from_i64(ctx, 0)));
    solver.assert(&resource_id.lt(&Int::from_i64(ctx, num_resources)));
    
    // 汎用的な制約の追加
    add_generic_constraints(ctx, &solver, policy, &user_id, &resource_id, &can_access);
    
    // アクセス可能な組み合わせを検索
    solver.push();
    solver.assert(&can_access);
    
    println!("\n🎯 Finding accessible combinations (max {}):", max_solutions);
    let mut count = 0;
    let mut solutions = Vec::new();
    
    while solver.check() == SatResult::Sat && count < max_solutions {
        let model = solver.get_model().unwrap();
        let user_val = model.eval(&user_id, true).unwrap().as_i64().unwrap();
        let resource_val = model.eval(&resource_id, true).unwrap().as_i64().unwrap();
        
        let user_name = &policy.users[user_val as usize].name;
        let resource_name = &policy.resources[resource_val as usize].name;
        
        println!("  ✅ {}: {} can access {}", count + 1, user_name, resource_name);
        
        // 解の詳細表示
        let user = &policy.users[user_val as usize];
        let resource = &policy.resources[resource_val as usize];
        show_access_details(user, resource);
        
        solutions.push((user_val, resource_val));
        count += 1;
        
        // 同じ解を避けるための制約を追加
        solver.assert(&Bool::not(&Bool::and(ctx, &[
            &user_id._eq(&Int::from_i64(ctx, user_val)),
            &resource_id._eq(&Int::from_i64(ctx, resource_val))
        ])));
    }
    
    solver.pop(1);
    
    if count == 0 {
        println!("  ❌ No valid access combinations found");
    } else {
        println!("\n📈 Analysis Summary: Found {} accessible combinations", count);
    }
}

fn add_generic_constraints(
    ctx: &Context, 
    solver: &Solver, 
    policy: &AbacPolicy, 
    user_id: &Int, 
    resource_id: &Int, 
    can_access: &Bool
) {
    // ファイル固有の制約を追加
    if policy.filename.contains("university") {
        add_university_constraints(ctx, solver, policy, user_id, resource_id, can_access);
    } else if policy.filename.contains("edocument") {
        add_edocument_constraints(ctx, solver, policy, user_id, resource_id, can_access);
    } else if policy.filename.contains("workforce") {
        add_workforce_constraints(ctx, solver, policy, user_id, resource_id, can_access);
    } else {
        // 汎用的な制約
        add_fallback_constraints(ctx, solver, policy, user_id, resource_id, can_access);
    }
}

fn add_university_constraints(
    ctx: &Context, 
    solver: &Solver, 
    policy: &AbacPolicy, 
    user_id: &Int, 
    resource_id: &Int, 
    can_access: &Bool
) {
    // Rule 1: 学生が受講した科目のgradebookにアクセス可能
    for (u_idx, user) in policy.users.iter().enumerate() {
        let user_condition = user_id._eq(&Int::from_i64(ctx, u_idx as i64));
        
        if let Some(AttributeValue::String(pos)) = user.attributes.get("position") {
            if pos == "student" {
                if let Some(AttributeValue::StringSet(user_courses)) = user.attributes.get("crsTaken") {
                    for (r_idx, resource) in policy.resources.iter().enumerate() {
                        if let Some(AttributeValue::String(r_type)) = resource.attributes.get("type") {
                            if r_type == "gradebook" {
                                let resource_condition = resource_id._eq(&Int::from_i64(ctx, r_idx as i64));
                                let combined_condition = Bool::and(ctx, &[&user_condition, &resource_condition]);
                                
                                if let Some(AttributeValue::String(r_course)) = resource.attributes.get("crs") {
                                    if user_courses.contains(r_course) {
                                        solver.assert(&Bool::implies(&combined_condition, can_access));
                                    } else {
                                        solver.assert(&Bool::implies(&combined_condition, &Bool::not(can_access)));
                                    }
                                }
                            }
                        }
                    }
                } else {
                    // 学生だが受講科目がない場合
                    for (r_idx, resource) in policy.resources.iter().enumerate() {
                        if let Some(AttributeValue::String(r_type)) = resource.attributes.get("type") {
                            if r_type == "gradebook" {
                                let resource_condition = resource_id._eq(&Int::from_i64(ctx, r_idx as i64));
                                let combined_condition = Bool::and(ctx, &[&user_condition, &resource_condition]);
                                solver.assert(&Bool::implies(&combined_condition, &Bool::not(can_access)));
                            }
                        }
                    }
                }
            }
        }
    }
}

fn add_edocument_constraints(
    ctx: &Context, 
    solver: &Solver, 
    policy: &AbacPolicy, 
    user_id: &Int, 
    resource_id: &Int, 
    can_access: &Bool
) {
    // Rule 1: 未登録の顧客は送られたドキュメントのみ閲覧可能
    for (u_idx, user) in policy.users.iter().enumerate() {
        let user_condition = user_id._eq(&Int::from_i64(ctx, u_idx as i64));
        
        if let Some(AttributeValue::String(role)) = user.attributes.get("role") {
            if role == "customer" {
                if let Some(AttributeValue::Boolean(false)) = user.attributes.get("registered") {
                    // 未登録顧客の場合、recipientsに含まれるドキュメントのみアクセス可能
                    for (r_idx, resource) in policy.resources.iter().enumerate() {
                        let resource_condition = resource_id._eq(&Int::from_i64(ctx, r_idx as i64));
                        let combined_condition = Bool::and(ctx, &[&user_condition, &resource_condition]);
                        
                        if let Some(AttributeValue::StringSet(recipients)) = resource.attributes.get("recipients") {
                            if recipients.contains(&user.name) {
                                solver.assert(&Bool::implies(&combined_condition, can_access));
                            } else {
                                solver.assert(&Bool::implies(&combined_condition, &Bool::not(can_access)));
                            }
                        } else {
                            solver.assert(&Bool::implies(&combined_condition, &Bool::not(can_access)));
                        }
                    }
                }
            }
        }
    }
}

fn add_workforce_constraints(
    ctx: &Context, 
    solver: &Solver, 
    policy: &AbacPolicy, 
    user_id: &Int, 
    resource_id: &Int, 
    can_access: &Bool
) {
    // Rule 7: 技術者は自分に割り当てられたタスクにアクセス可能
    for (u_idx, user) in policy.users.iter().enumerate() {
        let user_condition = user_id._eq(&Int::from_i64(ctx, u_idx as i64));
        
        if let Some(AttributeValue::String(pos)) = user.attributes.get("position") {
            if pos == "technician" {
                for (r_idx, resource) in policy.resources.iter().enumerate() {
                    if let Some(AttributeValue::String(r_type)) = resource.attributes.get("type") {
                        if r_type == "task" {
                            let resource_condition = resource_id._eq(&Int::from_i64(ctx, r_idx as i64));
                            let combined_condition = Bool::and(ctx, &[&user_condition, &resource_condition]);
                            
                            if let Some(AttributeValue::String(assigned_tech)) = resource.attributes.get("assignedTechnician") {
                                if assigned_tech == &user.name {
                                    solver.assert(&Bool::implies(&combined_condition, can_access));
                                } else {
                                    solver.assert(&Bool::implies(&combined_condition, &Bool::not(can_access)));
                                }
                            } else {
                                solver.assert(&Bool::implies(&combined_condition, &Bool::not(can_access)));
                            }
                        }
                    }
                }
            }
        }
    }
}

fn add_fallback_constraints(
    ctx: &Context, 
    solver: &Solver, 
    policy: &AbacPolicy, 
    user_id: &Int, 
    resource_id: &Int, 
    can_access: &Bool
) {
    // 汎用的な制約：同じdepartmentのリソースにアクセス可能
    for (u_idx, user) in policy.users.iter().enumerate() {
        let user_condition = user_id._eq(&Int::from_i64(ctx, u_idx as i64));
        
        if let Some(AttributeValue::String(user_dept)) = user.attributes.get("department") {
            for (r_idx, resource) in policy.resources.iter().enumerate() {
                let resource_condition = resource_id._eq(&Int::from_i64(ctx, r_idx as i64));
                let combined_condition = Bool::and(ctx, &[&user_condition, &resource_condition]);
                
                if let Some(AttributeValue::String(resource_dept)) = resource.attributes.get("department") {
                    if user_dept == resource_dept {
                        solver.assert(&Bool::implies(&combined_condition, can_access));
                    } else {
                        solver.assert(&Bool::implies(&combined_condition, &Bool::not(can_access)));
                    }
                } else {
                    // departmentが指定されていないリソースは基本的にアクセス不可
                    solver.assert(&Bool::implies(&combined_condition, &Bool::not(can_access)));
                }
            }
        }
    }
}

fn show_access_details(user: &UserAttribute, resource: &ResourceAttribute) {
    println!("    👤 User: {}", user.name);
    for (key, value) in &user.attributes {
        match value {
            AttributeValue::String(s) => println!("      {}: {}", key, s),
            AttributeValue::StringSet(set) => {
                if !set.is_empty() {
                    println!("      {}: {:?}", key, set);
                }
            },
            AttributeValue::Boolean(b) => println!("      {}: {}", key, b),
        }
    }
    
    println!("    📄 Resource: {}", resource.name);
    for (key, value) in &resource.attributes {
        match value {
            AttributeValue::String(s) => println!("      {}: {}", key, s),
            AttributeValue::StringSet(set) => {
                if !set.is_empty() {
                    println!("      {}: {:?}", key, set);
                }
            },
            AttributeValue::Boolean(b) => println!("      {}: {}", key, b),
        }
    }
    println!();
}