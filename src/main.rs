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

    // 4. 各ルールの個別Z3分析
    println!("\n🔬 Starting Individual Rule Analysis...");
    let z3_analysis_start = Instant::now();
    analyze_all_rules_individually(&ctx, &policy);
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

fn analyze_all_rules_individually(ctx: &Context, policy: &AbacPolicy) {
    println!("🔬 Analyzing each rule individually with Z3 constraints...");
    
    // 各ルールを個別に分析
    for rule in &policy.rules {
        println!("\n{}", "=".repeat(80));
        println!("🎯 Analyzing Rule {}: {}", rule.id, rule.description);
        println!("📝 Rule content: {}", rule.raw_content);
        
        let rule_start = Instant::now();
        let solutions = analyze_single_rule(ctx, policy, rule);
        let rule_duration = rule_start.elapsed();
        
        if solutions.is_empty() {
            println!("  ❌ No valid access combinations found for this rule");
        } else {
            println!("\n📈 Rule {} Summary: Found {} accessible combinations", rule.id, solutions.len());
        }
        
        println!("⏱️  Rule {} analysis time: {:.2?}", rule.id, rule_duration);
    }
}

fn analyze_single_rule(ctx: &Context, policy: &AbacPolicy, rule: &Rule) -> Vec<(i64, i64)> {
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
    
    // 特定のルールに対する制約の追加
    add_rule_constraints(ctx, &solver, policy, rule, &user_id, &resource_id, &can_access);
    
    // アクセス可能な組み合わせを検索（すべての組み合わせ）
    solver.push();
    solver.assert(&can_access);
    
    println!("\n🔍 Finding all accessible combinations for Rule {}...", rule.id);
    let mut solutions = Vec::new();
    let mut count = 0;
    
    // すべての解を見つける（制限なし）
    while solver.check() == SatResult::Sat {
        let model = solver.get_model().unwrap();
        let user_val = model.eval(&user_id, true).unwrap().as_i64().unwrap();
        let resource_val = model.eval(&resource_id, true).unwrap().as_i64().unwrap();
        
        let user_name = &policy.users[user_val as usize].name;
        let resource_name = &policy.resources[resource_val as usize].name;
        
        count += 1;
        println!("  ✅ {}: {} can access {}", count, user_name, resource_name);
        
        // 簡潔な詳細表示
        show_access_summary(&policy.users[user_val as usize], &policy.resources[resource_val as usize]);
        
        solutions.push((user_val, resource_val));
        
        // 同じ解を避けるための制約を追加
        solver.assert(&Bool::not(&Bool::and(ctx, &[
            &user_id._eq(&Int::from_i64(ctx, user_val)),
            &resource_id._eq(&Int::from_i64(ctx, resource_val))
        ])));
        
        // 進行状況の表示（1000個ごと）
        if count % 1000 == 0 {
            println!("    📊 Progress: {} combinations found so far...", count);
        }
        
        // 安全装置：非常に多くの解がある場合の制限
        if count >= 500000 {
            println!("    ⚠️  Reached maximum limit of 50,000 combinations for safety");
            break;
        }
    }
    
    solver.pop(1);
    solutions
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

fn add_rule_constraints(
    ctx: &Context, 
    solver: &Solver, 
    policy: &AbacPolicy, 
    rule: &Rule,
    user_id: &Int, 
    resource_id: &Int, 
    can_access: &Bool
) {
    // ルールの内容を解析して適切な制約を追加
    let rule_content = &rule.raw_content;
    
    // ルール内容に基づいて制約を判定
    if rule_content.contains("role [ {customer}") && rule_content.contains("registered [ {False}") {
        // Rule 1: 未登録顧客ルール
        add_unregistered_customer_constraints(ctx, solver, policy, user_id, resource_id, can_access);
    } else if rule_content.contains("role [ {helpdesk}") && rule_content.contains("{search readMetaInfo}") {
        // Rule 2: ヘルプデスクのメタ情報ルール
        add_helpdesk_meta_constraints(ctx, solver, policy, user_id, resource_id, can_access);
    } else if rule_content.contains("role [ {helpdesk}") && rule_content.contains("isConfidential [ {False}") {
        // Rule 3: ヘルプデスクの非機密文書ルール
        add_helpdesk_nonconfidential_constraints(ctx, solver, policy, user_id, resource_id, can_access);
    } else if rule_content.contains("role [ {admin}") && rule_content.contains("isConfidential [ {False}") {
        // Rule 4: 管理者の非機密文書ルール
        add_admin_nonconfidential_constraints(ctx, solver, policy, user_id, resource_id, can_access);
    } else if rule_content.contains("supervisee ] owner") {
        // Rule 5: 上司による部下の文書アクセス
        add_supervisor_constraints(ctx, solver, policy, user_id, resource_id, can_access);
    } else if rule_content.contains("currentProjects ] projectId") {
        // Rule 6 & 16: プロジェクトメンバーのアクセス
        add_project_member_constraints(ctx, solver, policy, user_id, resource_id, can_access);
    } else if rule_content.contains("securityClearance [") && rule_content.contains("securityLevel [") {
        // Rule 7 & 21: セキュリティクリアランス
        add_security_clearance_constraints(ctx, solver, policy, user_id, resource_id, can_access);
    } else if rule_content.contains("role [ {financialOfficer}") {
        // Rule 8: 財務担当者
        add_financial_officer_constraints(ctx, solver, policy, user_id, resource_id, can_access);
    } else if rule_content.contains("role [ {legalOfficer}") {
        // Rule 9: 法務担当者
        add_legal_officer_constraints(ctx, solver, policy, user_id, resource_id, can_access);
    } else if rule_content.contains("role [ {auditor}") && rule_content.contains("containsPersonalInfo [ {False}") {
        // Rule 10: 監査人
        add_auditor_constraints(ctx, solver, policy, user_id, resource_id, can_access);
    } else if rule_content.contains("uid [ recipients") {
        // Rule 13: コンサルタントや受信者ルール
        add_recipient_constraints(ctx, solver, policy, user_id, resource_id, can_access);
    } else if rule_content.contains("language = language") && rule_content.contains("region = region") {
        // Rule 20: 言語・地域ルール
        add_language_region_constraints(ctx, solver, policy, user_id, resource_id, can_access);
    } else {
        // フォールバック：基本的な制約
        add_basic_constraints(ctx, solver, policy, user_id, resource_id, can_access);
    }
}

fn add_unregistered_customer_constraints(
    ctx: &Context, solver: &Solver, policy: &AbacPolicy, 
    user_id: &Int, resource_id: &Int, can_access: &Bool
) {
    for (u_idx, user) in policy.users.iter().enumerate() {
        let user_condition = user_id._eq(&Int::from_i64(ctx, u_idx as i64));
        
        if let Some(AttributeValue::String(role)) = user.attributes.get("role") {
            if role == "customer" {
                if let Some(AttributeValue::Boolean(false)) = user.attributes.get("registered") {
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

fn add_helpdesk_meta_constraints(
    ctx: &Context, solver: &Solver, policy: &AbacPolicy, 
    user_id: &Int, resource_id: &Int, can_access: &Bool
) {
    for (u_idx, user) in policy.users.iter().enumerate() {
        if let Some(AttributeValue::String(role)) = user.attributes.get("role") {
            if role == "helpdesk" {
                let user_condition = user_id._eq(&Int::from_i64(ctx, u_idx as i64));
                for r_idx in 0..policy.resources.len() {
                    let resource_condition = resource_id._eq(&Int::from_i64(ctx, r_idx as i64));
                    let combined_condition = Bool::and(ctx, &[&user_condition, &resource_condition]);
                    solver.assert(&Bool::implies(&combined_condition, can_access));
                }
            }
        }
    }
}

fn add_helpdesk_nonconfidential_constraints(
    ctx: &Context, solver: &Solver, policy: &AbacPolicy, 
    user_id: &Int, resource_id: &Int, can_access: &Bool
) {
    for (u_idx, user) in policy.users.iter().enumerate() {
        if let Some(AttributeValue::String(role)) = user.attributes.get("role") {
            if role == "helpdesk" {
                let user_condition = user_id._eq(&Int::from_i64(ctx, u_idx as i64));
                let user_tenant = user.attributes.get("tenant");
                
                for (r_idx, resource) in policy.resources.iter().enumerate() {
                    if let Some(AttributeValue::Boolean(false)) = resource.attributes.get("isConfidential") {
                        let resource_condition = resource_id._eq(&Int::from_i64(ctx, r_idx as i64));
                        let combined_condition = Bool::and(ctx, &[&user_condition, &resource_condition]);
                        
                        // テナントチェック
                        if let (Some(AttributeValue::String(u_tenant)), Some(AttributeValue::String(r_tenant))) = (user_tenant, resource.attributes.get("tenant")) {
                            if u_tenant == r_tenant {
                                solver.assert(&Bool::implies(&combined_condition, can_access));
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

fn add_admin_nonconfidential_constraints(
    ctx: &Context, solver: &Solver, policy: &AbacPolicy, 
    user_id: &Int, resource_id: &Int, can_access: &Bool
) {
    for (u_idx, user) in policy.users.iter().enumerate() {
        if let Some(AttributeValue::String(role)) = user.attributes.get("role") {
            if role == "admin" {
                let user_condition = user_id._eq(&Int::from_i64(ctx, u_idx as i64));
                for (r_idx, resource) in policy.resources.iter().enumerate() {
                    if let Some(AttributeValue::Boolean(false)) = resource.attributes.get("isConfidential") {
                        let resource_condition = resource_id._eq(&Int::from_i64(ctx, r_idx as i64));
                        let combined_condition = Bool::and(ctx, &[&user_condition, &resource_condition]);
                        solver.assert(&Bool::implies(&combined_condition, can_access));
                    }
                }
            }
        }
    }
}

fn add_supervisor_constraints(
    ctx: &Context, solver: &Solver, policy: &AbacPolicy, 
    user_id: &Int, resource_id: &Int, can_access: &Bool
) {
    for (u_idx, user) in policy.users.iter().enumerate() {
        let user_condition = user_id._eq(&Int::from_i64(ctx, u_idx as i64));
        
        if let Some(AttributeValue::StringSet(supervisees)) = user.attributes.get("supervisee") {
            for (r_idx, resource) in policy.resources.iter().enumerate() {
                if let Some(AttributeValue::String(owner)) = resource.attributes.get("owner") {
                    if supervisees.contains(owner) {
                        let resource_condition = resource_id._eq(&Int::from_i64(ctx, r_idx as i64));
                        let combined_condition = Bool::and(ctx, &[&user_condition, &resource_condition]);
                        solver.assert(&Bool::implies(&combined_condition, can_access));
                    }
                }
            }
        }
    }
}

fn add_project_member_constraints(
    ctx: &Context, solver: &Solver, policy: &AbacPolicy, 
    user_id: &Int, resource_id: &Int, can_access: &Bool
) {
    for (u_idx, user) in policy.users.iter().enumerate() {
        let user_condition = user_id._eq(&Int::from_i64(ctx, u_idx as i64));
        
        if let Some(AttributeValue::StringSet(user_projects)) = user.attributes.get("currentProjects") {
            for (r_idx, resource) in policy.resources.iter().enumerate() {
                if let Some(AttributeValue::String(project_id)) = resource.attributes.get("projectId") {
                    if user_projects.contains(project_id) {
                        let resource_condition = resource_id._eq(&Int::from_i64(ctx, r_idx as i64));
                        let combined_condition = Bool::and(ctx, &[&user_condition, &resource_condition]);
                        solver.assert(&Bool::implies(&combined_condition, can_access));
                    }
                }
            }
        }
    }
}

fn add_security_clearance_constraints(
    ctx: &Context, solver: &Solver, policy: &AbacPolicy, 
    user_id: &Int, resource_id: &Int, can_access: &Bool
) {
    let clearance_levels = vec!["public", "internal", "confidential", "secret", "topSecret"];
    
    for (u_idx, user) in policy.users.iter().enumerate() {
        let user_condition = user_id._eq(&Int::from_i64(ctx, u_idx as i64));
        
        if let Some(AttributeValue::String(user_clearance)) = user.attributes.get("securityClearance") {
            let user_level = clearance_levels.iter().position(|&x| x == user_clearance).unwrap_or(0);
            
            for (r_idx, resource) in policy.resources.iter().enumerate() {
                if let Some(AttributeValue::String(resource_level)) = resource.attributes.get("securityLevel") {
                    let resource_level_idx = clearance_levels.iter().position(|&x| x == resource_level).unwrap_or(0);
                    
                    let resource_condition = resource_id._eq(&Int::from_i64(ctx, r_idx as i64));
                    let combined_condition = Bool::and(ctx, &[&user_condition, &resource_condition]);
                    
                    if user_level >= resource_level_idx {
                        solver.assert(&Bool::implies(&combined_condition, can_access));
                    } else {
                        solver.assert(&Bool::implies(&combined_condition, &Bool::not(can_access)));
                    }
                }
            }
        }
    }
}

fn add_financial_officer_constraints(
    ctx: &Context, solver: &Solver, policy: &AbacPolicy, 
    user_id: &Int, resource_id: &Int, can_access: &Bool
) {
    for (u_idx, user) in policy.users.iter().enumerate() {
        if let Some(AttributeValue::String(role)) = user.attributes.get("role") {
            if role == "financialOfficer" {
                let user_condition = user_id._eq(&Int::from_i64(ctx, u_idx as i64));
                for (r_idx, resource) in policy.resources.iter().enumerate() {
                    if let Some(AttributeValue::StringSet(tags)) = resource.attributes.get("tags") {
                        if tags.contains("financial") {
                            let resource_condition = resource_id._eq(&Int::from_i64(ctx, r_idx as i64));
                            let combined_condition = Bool::and(ctx, &[&user_condition, &resource_condition]);
                            solver.assert(&Bool::implies(&combined_condition, can_access));
                        }
                    }
                }
            }
        }
    }
}

fn add_legal_officer_constraints(
    ctx: &Context, solver: &Solver, policy: &AbacPolicy, 
    user_id: &Int, resource_id: &Int, can_access: &Bool
) {
    for (u_idx, user) in policy.users.iter().enumerate() {
        if let Some(AttributeValue::String(role)) = user.attributes.get("role") {
            if role == "legalOfficer" {
                let user_condition = user_id._eq(&Int::from_i64(ctx, u_idx as i64));
                for (r_idx, resource) in policy.resources.iter().enumerate() {
                    if let Some(AttributeValue::StringSet(tags)) = resource.attributes.get("tags") {
                        if tags.contains("legal") {
                            let resource_condition = resource_id._eq(&Int::from_i64(ctx, r_idx as i64));
                            let combined_condition = Bool::and(ctx, &[&user_condition, &resource_condition]);
                            solver.assert(&Bool::implies(&combined_condition, can_access));
                        }
                    }
                }
            }
        }
    }
}

fn add_auditor_constraints(
    ctx: &Context, solver: &Solver, policy: &AbacPolicy, 
    user_id: &Int, resource_id: &Int, can_access: &Bool
) {
    for (u_idx, user) in policy.users.iter().enumerate() {
        if let Some(AttributeValue::String(role)) = user.attributes.get("role") {
            if role == "auditor" {
                let user_condition = user_id._eq(&Int::from_i64(ctx, u_idx as i64));
                for (r_idx, resource) in policy.resources.iter().enumerate() {
                    if let Some(AttributeValue::Boolean(false)) = resource.attributes.get("containsPersonalInfo") {
                        let resource_condition = resource_id._eq(&Int::from_i64(ctx, r_idx as i64));
                        let combined_condition = Bool::and(ctx, &[&user_condition, &resource_condition]);
                        solver.assert(&Bool::implies(&combined_condition, can_access));
                    }
                }
            }
        }
    }
}

fn add_recipient_constraints(
    ctx: &Context, solver: &Solver, policy: &AbacPolicy, 
    user_id: &Int, resource_id: &Int, can_access: &Bool
) {
    for (u_idx, user) in policy.users.iter().enumerate() {
        let user_condition = user_id._eq(&Int::from_i64(ctx, u_idx as i64));
        
        for (r_idx, resource) in policy.resources.iter().enumerate() {
            if let Some(AttributeValue::StringSet(recipients)) = resource.attributes.get("recipients") {
                if recipients.contains(&user.name) {
                    let resource_condition = resource_id._eq(&Int::from_i64(ctx, r_idx as i64));
                    let combined_condition = Bool::and(ctx, &[&user_condition, &resource_condition]);
                    solver.assert(&Bool::implies(&combined_condition, can_access));
                }
            }
        }
    }
}

fn add_language_region_constraints(
    ctx: &Context, solver: &Solver, policy: &AbacPolicy, 
    user_id: &Int, resource_id: &Int, can_access: &Bool
) {
    for (u_idx, user) in policy.users.iter().enumerate() {
        let user_condition = user_id._eq(&Int::from_i64(ctx, u_idx as i64));
        
        for (r_idx, resource) in policy.resources.iter().enumerate() {
            let resource_condition = resource_id._eq(&Int::from_i64(ctx, r_idx as i64));
            let combined_condition = Bool::and(ctx, &[&user_condition, &resource_condition]);
            
            let same_language = match (user.attributes.get("language"), resource.attributes.get("language")) {
                (Some(AttributeValue::String(u_lang)), Some(AttributeValue::String(r_lang))) => u_lang == r_lang,
                _ => false
            };
            
            let same_region = match (user.attributes.get("region"), resource.attributes.get("region")) {
                (Some(AttributeValue::String(u_region)), Some(AttributeValue::String(r_region))) => u_region == r_region,
                _ => false
            };
            
            if same_language && same_region {
                solver.assert(&Bool::implies(&combined_condition, can_access));
            } else {
                solver.assert(&Bool::implies(&combined_condition, &Bool::not(can_access)));
            }
        }
    }
}

fn add_basic_constraints(
    ctx: &Context, solver: &Solver, policy: &AbacPolicy, 
    user_id: &Int, resource_id: &Int, can_access: &Bool
) {
    // 基本的な制約：同じテナントのリソースにアクセス可能
    for (u_idx, user) in policy.users.iter().enumerate() {
        let user_condition = user_id._eq(&Int::from_i64(ctx, u_idx as i64));
        
        if let Some(user_tenant) = user.attributes.get("tenant") {
            for (r_idx, resource) in policy.resources.iter().enumerate() {
                let resource_condition = resource_id._eq(&Int::from_i64(ctx, r_idx as i64));
                let combined_condition = Bool::and(ctx, &[&user_condition, &resource_condition]);
                
                if let (AttributeValue::String(u_tenant), Some(AttributeValue::String(r_tenant))) = (user_tenant, resource.attributes.get("tenant")) {
                    if u_tenant == r_tenant {
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

fn show_access_summary(user: &UserAttribute, resource: &ResourceAttribute) {
    let user_role = user.attributes.get("role")
        .map(|v| match v {
            AttributeValue::String(s) => s.as_str(),
            _ => "unknown"
        }).unwrap_or("unknown");
    
    let user_dept = user.attributes.get("department")
        .map(|v| match v {
            AttributeValue::String(s) => s.as_str(),
            _ => "unknown"
        }).unwrap_or("unknown");
    
    let resource_type = resource.attributes.get("type")
        .map(|v| match v {
            AttributeValue::String(s) => s.as_str(),
            _ => "unknown"
        }).unwrap_or("unknown");
    
    println!("      👤 {} ({}, {}) → 📄 {} ({})", 
             user.name, user_role, user_dept, resource.name, resource_type);
}