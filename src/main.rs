use z3::{Config, Context, Solver, Sort, FuncDecl, Symbol, DatatypeBuilder, ast::{Ast, Dynamic}};
use std::collections::HashMap;
use std::fs;
use indicatif::{ProgressBar, ProgressStyle, MultiProgress, ProgressState, ProgressFinish};
use console::Term;
use std::time::Duration;
use std::thread;

// 新しく追加されたモジュールをインポート
mod universityTypes;
mod parser;
mod abac_solver;
mod edocumentTypes;
mod edocumentParser;
mod edocument_abac_solver;

use universityTypes::{
    UniversityAbacData, UniversityUserAttribute, UniversityResourceAttribute, UniversityRule,
    Position as UniversityPosition, Department as UniversityDepartment, Course, ResourceType, Action as UniversityAction
};
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
    // マルチプログレスバーを設定
    let mp = MultiProgress::new();
    
    // 全体の進捗を管理するメインプログレスバー
    let main_pb = mp.add(create_progress_bar(5, "Overall Progress"));
    
    // 従来のZ3の例
    main_pb.set_message("Running Z3 Simple Example");
    println!("=== Z3 Simple Example ===");
    z3_simple_example();
    main_pb.inc(1);
    
    // 新しいABACパーサーの例
    main_pb.set_message("Running University ABAC Parser Example");
    println!("\n=== University ABAC Parser Example ===");
    university_abac_example()?;
    main_pb.inc(1);
    
    // ABACソルバーの例
    main_pb.set_message("Running University ABAC Solver Example");
    println!("\n=== University ABAC Solver Example ===");
    university_abac_solver_example()?;
    main_pb.inc(1);
    
    // 実際のuniversity.abacファイルのテスト
    main_pb.set_message("Running Real University ABAC File Test");
    println!("\n=== Real University ABAC File Test ===");
    real_university_abac_test(&mp)?;
    main_pb.inc(1);
    
    // edocument用のテスト
    main_pb.set_message("Running Edocument ABAC Test");
    println!("\n=== Edocument ABAC Test ===");
    edocument_abac_test(&mp)?;
    main_pb.inc(1);
    
    main_pb.finish_with_message("All tests completed!");
    println!("\n🎉 All processes completed successfully!");
    
    Ok(())
}

fn real_university_abac_test(mp: &MultiProgress) -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing with actual university.abac file...");
    
    // ファイル読み込み用のプログレスバー
    let file_pb = mp.add(create_spinner("Reading university.abac file"));
    
    // ファイルを読み込む
    let file_content = match fs::read_to_string("data/university_fixed.abac") {
        Ok(content) => content,
        Err(e) => {
            file_pb.finish_with_message("❌ Error reading university_fixed.abac file");
            println!("Error reading university_fixed.abac file: {}", e);
            println!("Make sure the file exists in data/university_fixed.abac");
            return Ok(());
        }
    };
    
    file_pb.finish_with_message("✅ File loaded successfully");
    println!("File loaded successfully. Size: {} bytes", file_content.len());
    
    // パーサーでデータを解析
    let parse_pb = mp.add(create_spinner("Parsing ABAC data"));
    let parser = UniversityAbacParser::new();
    match parser.parse(&file_content) {
        Ok(abac_data) => {
            parse_pb.finish_with_message("✅ Parsing completed successfully");
            println!("\n=== Parsing Results ===");
            println!("Users: {}", abac_data.users.len());
            println!("Resources: {}", abac_data.resources.len());
            println!("Rules: {}", abac_data.rules.len());
            
            // ユーザー一覧の表示
            println!("\n--- User Summary ---");
            let mut applicant_count = 0;
            let mut student_count = 0;
            let mut faculty_count = 0;
            let mut staff_count = 0;
            let mut chair_count = 0;
            
            for user in &abac_data.users {
                match user.position {
                    Some(UniversityPosition::Applicant) => applicant_count += 1,
                    Some(UniversityPosition::Student) => student_count += 1,
                    Some(UniversityPosition::Faculty) => faculty_count += 1,
                    Some(UniversityPosition::Staff) => staff_count += 1,
                    _ => {}
                }
                if user.is_chair == Some(true) {
                    chair_count += 1;
                }
            }
            
            println!("  Applicants: {}", applicant_count);
            println!("  Students: {}", student_count);
            println!("  Faculty: {}", faculty_count);
            println!("  Staff: {}", staff_count);
            println!("  Chairs: {}", chair_count);
            
            // リソース一覧の表示
            println!("\n--- Resource Summary ---");
            let mut application_count = 0;
            let mut gradebook_count = 0;
            let mut roster_count = 0;
            let mut transcript_count = 0;
            
            for resource in &abac_data.resources {
                match resource.resource_type {
                    ResourceType::Application => application_count += 1,
                    ResourceType::Gradebook => gradebook_count += 1,
                    ResourceType::Roster => roster_count += 1,
                    ResourceType::Transcript => transcript_count += 1,
                }
            }
            
            println!("  Applications: {}", application_count);
            println!("  Gradebooks: {}", gradebook_count);
            println!("  Rosters: {}", roster_count);
            println!("  Transcripts: {}", transcript_count);
            
            // ルールの詳細
            println!("\n--- Rule Details ---");
            for rule in &abac_data.rules {
                println!("Rule {}: {}", rule.id + 1, rule.description);
                println!("  User conditions: {}", rule.user_conditions.len());
                println!("  Resource conditions: {}", rule.resource_conditions.len());
                println!("  Actions: {:?}", rule.actions);
                println!("  Comparison conditions: {}", rule.comparison_conditions.len());
                println!();
            }
            
            // Z3ソルバーでの制約生成テスト
            println!("=== Z3 Constraint Generation Test ===");
            let ctx_pb = mp.add(create_spinner("Creating Z3 context"));
            let ctx = create_optimized_z3_context();
            ctx_pb.finish_with_message("✅ Z3 context created");
            
            let solver_pb = mp.add(create_spinner("Creating ABAC solver"));
            let mut solver = UniversityAbacSolver::new(&ctx, &abac_data);
            solver_pb.finish_with_message("✅ ABAC solver created");
            
            let constraints_pb = mp.add(create_spinner("Generating constraints"));
            match solver.generate_constraints(&abac_data) {
                Ok(()) => {
                    constraints_pb.finish_with_message("✅ Constraint generation successful!");
                    println!("✅ Constraint generation successful!");
                    println!("All {} rules processed successfully.", abac_data.rules.len());
                    
                    // 制約の詳細情報
                    println!("\n=== Constraint Generation Summary ===");
                    println!("- {} user attribute constraints generated", abac_data.users.len());
                    println!("- {} resource attribute constraints generated", abac_data.resources.len());
                    println!("- {} rule constraints generated", abac_data.rules.len());
                    
                    let total_constraints = abac_data.users.len() + abac_data.resources.len() + abac_data.rules.len();
                    println!("- Total constraints in Z3 solver: ~{}", total_constraints);
                    
                    // アクセス制御の解析を実行
                    println!("\n=== Access Control Analysis ===");
                    println!("Analyzing possible user-resource-action combinations...");
                    
                    let analysis_pb = mp.add(create_spinner("Analyzing access control"));
                    match solver.solve_access_control() {
                        Ok(results) => {
                            analysis_pb.finish_with_message("✅ Access control analysis completed");
                            if results.is_empty() {
                                println!("No valid access combinations found.");
                            } else {
                                println!("Found {} valid access combinations:", results.len());
                                println!();
                                
                                // 結果を表示
                                for (i, result) in results.iter().enumerate() {
                                    println!("{}. {}", i + 1, result.display());
                                }
                                
                                // 統計情報
                                println!("\n--- Access Statistics ---");
                                let mut user_access_count = std::collections::HashMap::new();
                                let mut resource_access_count = std::collections::HashMap::new();
                                let mut action_access_count = std::collections::HashMap::new();
                                
                                for result in &results {
                                    *user_access_count.entry(result.user.clone()).or_insert(0) += 1;
                                    *resource_access_count.entry(result.resource.clone()).or_insert(0) += 1;
                                    *action_access_count.entry(result.action.clone()).or_insert(0) += 1;
                                }
                                
                                println!("Users with access rights:");
                                for (user, count) in user_access_count {
                                    println!("  {}: {} permissions", user, count);
                                }
                                
                                println!("\nResources being accessed:");
                                for (resource, count) in resource_access_count {
                                    println!("  {}: {} access patterns", resource, count);
                                }
                                
                                println!("\nActions being performed:");
                                for (action, count) in action_access_count {
                                    println!("  {}: {} times", action, count);
                                }
                            }
                        }
                        Err(e) => {
                            analysis_pb.finish_with_message("❌ Error in access control analysis");
                            println!("❌ Error in access control analysis: {}", e);
                        }
                    }
                    
                    println!("\n🎉 Real university.abac file processing completed successfully!");
                }
                Err(e) => {
                    constraints_pb.finish_with_message("❌ Error generating constraints");
                    println!("❌ Error generating constraints: {}", e);
                }
            }
        }
        Err(e) => {
            parse_pb.finish_with_message("❌ Parse error");
            println!("❌ Parse error: {}", e);
        }
    }
    
    Ok(())
}

fn z3_simple_example() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    // 1. 有限ドメインを列挙型（Datatype）として定義
    // ユーザー型: alice, bob, charlie のみ
    let user_sort = DatatypeBuilder::new(&ctx, "User")
        .variant("alice", vec![])
        .variant("bob", vec![])
        .variant("charlie", vec![])
        .finish();

    // リソース型: secret_data, dev_server のみ
    let resource_sort = DatatypeBuilder::new(&ctx, "Resource")
        .variant("secret_data", vec![])
        .variant("dev_server", vec![])
        .finish();

    // グループ型: admin, dev, guest のみ
    let group_sort = DatatypeBuilder::new(&ctx, "Group")
        .variant("admin", vec![])
        .variant("dev", vec![])
        .variant("guest", vec![])
        .finish();

    // 2. 定数を取得
    let alice = user_sort.variants[0].constructor.apply(&[]);
    let bob = user_sort.variants[1].constructor.apply(&[]);
    let charlie = user_sort.variants[2].constructor.apply(&[]);

    let secret_data = resource_sort.variants[0].constructor.apply(&[]);
    let dev_server = resource_sort.variants[1].constructor.apply(&[]);

    let admin_group = group_sort.variants[0].constructor.apply(&[]);
    let dev_group = group_sort.variants[1].constructor.apply(&[]);
    let guest_group = group_sort.variants[2].constructor.apply(&[]);

    // 3. 属性を関数として定義
    let user_in_group = FuncDecl::new(&ctx, "user_in_group", &[&user_sort.sort, &group_sort.sort], &Sort::bool(&ctx));
    let required_group_func = FuncDecl::new(&ctx, "required_group", &[&resource_sort.sort], &group_sort.sort);

    // 4. データセットの事実（Fact）をアサート
    // Alice の所属グループ: admin, dev
    solver.assert(&z3::ast::Bool::try_from(user_in_group.apply(&[&alice, &admin_group])).unwrap());
    solver.assert(&z3::ast::Bool::try_from(user_in_group.apply(&[&alice, &dev_group])).unwrap());
    solver.assert(&z3::ast::Bool::try_from(user_in_group.apply(&[&alice, &guest_group])).unwrap().not());

    // Bob の所属グループ: dev
    solver.assert(&z3::ast::Bool::try_from(user_in_group.apply(&[&bob, &admin_group])).unwrap().not());
    solver.assert(&z3::ast::Bool::try_from(user_in_group.apply(&[&bob, &dev_group])).unwrap());
    solver.assert(&z3::ast::Bool::try_from(user_in_group.apply(&[&bob, &guest_group])).unwrap().not());

    // Charlie の所属グループ: guest
    solver.assert(&z3::ast::Bool::try_from(user_in_group.apply(&[&charlie, &admin_group])).unwrap().not());
    solver.assert(&z3::ast::Bool::try_from(user_in_group.apply(&[&charlie, &dev_group])).unwrap().not());
    solver.assert(&z3::ast::Bool::try_from(user_in_group.apply(&[&charlie, &guest_group])).unwrap());

    // リソースが必要とするグループ
    solver.assert(&required_group_func.apply(&[&secret_data])._eq(&admin_group));
    solver.assert(&required_group_func.apply(&[&dev_server])._eq(&dev_group));

    // 5. 解を求めるための変数を準備
    let u_var = z3::ast::Datatype::fresh_const(&ctx, "u", &user_sort.sort);
    let r_var = z3::ast::Datatype::fresh_const(&ctx, "r", &resource_sort.sort);

    // 6. アクセス制御ルール：ユーザーがリソースの必要グループに所属している
    let required_group = required_group_func.apply(&[&r_var]);
    let has_access = user_in_group.apply(&[&u_var, &required_group]);
    solver.assert(&z3::ast::Bool::try_from(has_access).unwrap());

    // 7. ループですべての解を列挙
    println!("--- アクセス可能な (ユーザー, リソース) の組み合わせ ---");
    let mut solution_count = 0;
    loop {
        match solver.check() {
            z3::SatResult::Sat => {
                solution_count += 1;
                let model = solver.get_model().unwrap();

                // モデルから u_var と r_var の値を取得
                let found_user = model.eval(&u_var, true).unwrap();
                let found_resource = model.eval(&r_var, true).unwrap();

                // 人間が読める形式で表示
                println!("ペア {}: ({}, {})",
                    solution_count,
                    found_user.to_string(),
                    found_resource.to_string()
                );

                // 見つかった解を禁止する制約を追加して、次の解を探す
                let block_this_solution = u_var._eq(&found_user) & r_var._eq(&found_resource);
                solver.assert(&block_this_solution.not());
            }
            z3::SatResult::Unsat => {
                println!("\n--- これ以上見つかりません ---");
                break;
            }
            z3::SatResult::Unknown => {
                println!("\n--- ソルバーが不明な結果を返しました ---");
                break;
            }
        }
    }
}

fn university_abac_example() -> Result<(), Box<dyn std::error::Error>> {
    let parser = UniversityAbacParser::new();
    
    // サンプルABACデータ
    let sample_abac = r#"
        # University ABAC Sample Data
        
        # User attributes
        userAttrib(applicant1, position=applicant)
        userAttrib(csStu1, position=student, department=cs, crsTaken={cs101})
        userAttrib(csStu2, position=student, department=cs, crsTaken={cs601}, crsTaught={cs101 cs602})
        userAttrib(csFac1, position=faculty, department=cs, crsTaught={cs101})
        userAttrib(csChair, isChair=True, department=cs)
        userAttrib(registrar1, position=staff, department=registrar)
        
        # Resource attributes
        resourceAttrib(application1, type=application, student=applicant1)
        resourceAttrib(csStu1application, type=application, student=csStu1)
        resourceAttrib(cs101gradebook, departments={cs}, crs=cs101, type=gradebook)
        resourceAttrib(cs101roster, departments={cs}, crs=cs101, type=roster)
        resourceAttrib(csStu1trans, student=csStu1, departments={cs}, type=transcript)
        
        # Rules (simplified syntax)
        rule(; type [ {gradebook}; {readMyScores}; crsTaken ] crs)
        rule(; type [ {gradebook}; {addScore readScore}; crsTaught ] crs)
        rule(position [ {faculty}; type [ {gradebook}; {changeScore assignGrade}; crsTaught ] crs)
        rule(department [ {registrar}; type [ {roster}; {read write}; )
        rule(; type [ {transcript}; {read}; uid=student)
    "#;
    
    // ABACデータをパース
    match parser.parse(sample_abac) {
        Ok(abac_data) => {
            println!("=== Parsed ABAC Data ===");
            println!("Users: {}", abac_data.users.len());
            println!("Resources: {}", abac_data.resources.len());
            println!("Rules: {}", abac_data.rules.len());
            
            println!("\n--- Users ---");
            for user in &abac_data.users {
                println!("User: {}", user.user_id);
                if let Some(position) = &user.position {
                    println!("  Position: {:?}", position);
                }
                if let Some(department) = &user.department {
                    println!("  Department: {:?}", department);
                }
                if !user.crs_taken.is_empty() {
                    println!("  Courses Taken: {:?}", user.crs_taken);
                }
                if !user.crs_taught.is_empty() {
                    println!("  Courses Taught: {:?}", user.crs_taught);
                }
                if let Some(is_chair) = user.is_chair {
                    println!("  Is Chair: {}", is_chair);
                }
                println!();
            }
            
            println!("--- Resources ---");
            for resource in &abac_data.resources {
                println!("Resource: {} (Type: {:?})", resource.resource_id, resource.resource_type);
                if let Some(student) = &resource.student {
                    println!("  Student: {}", student);
                }
                if let Some(course) = &resource.crs {
                    println!("  Course: {:?}", course);
                }
                if !resource.departments.is_empty() {
                    println!("  Departments: {:?}", resource.departments);
                }
                println!();
            }
            
            println!("--- Rules ---");
            for rule in &abac_data.rules {
                println!("Rule {}: {}", rule.id + 1, rule.description);
                println!("  User Conditions: {} conditions", rule.user_conditions.len());
                println!("  Resource Conditions: {} conditions", rule.resource_conditions.len());
                println!("  Actions: {:?}", rule.actions);
                println!("  Comparison Conditions: {} conditions", rule.comparison_conditions.len());
                println!();
            }
        }
        Err(e) => {
            println!("Parse error: {}", e);
        }
    }
    
    Ok(())
}

fn university_abac_solver_example() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = create_optimized_z3_context();
    let parser = UniversityAbacParser::new();
    
    // より詳細なABACデータサンプル
    let sample_abac = r#"
        # University ABAC Sample Data for Solver
        
        # User attributes
        userAttrib(alice, position=student, department=cs, crsTaken={cs101})
        userAttrib(bob, position=faculty, department=cs, crsTaught={cs101})
        userAttrib(charlie, position=staff, department=registrar)
        userAttrib(diana, position=student, department=cs, crsTaken={cs601})
        userAttrib(eve, isChair=True, department=cs, position=faculty)
        
        # Resource attributes  
        resourceAttrib(alice_application, type=application, student=alice)
        resourceAttrib(cs101_gradebook, type=gradebook, departments={cs}, crs=cs101)
        resourceAttrib(cs101_roster, type=roster, departments={cs}, crs=cs101)
        resourceAttrib(alice_transcript, type=transcript, student=alice, departments={cs})
        resourceAttrib(diana_transcript, type=transcript, student=diana, departments={cs})
        
        # Rules (simplified for demo)
        rule(; type [ {gradebook}; {readMyScores}; crsTaken ] crs)
        rule(; type [ {gradebook}; {addScore readScore}; crsTaught ] crs)
        rule(position [ {faculty}; type [ {gradebook}; {changeScore assignGrade}; crsTaught ] crs)
        rule(department [ {registrar}; type [ {roster}; {read write}; )
        rule(; type [ {transcript}; {read}; uid=student)
        rule(; type [ {application}; {checkStatus}; uid=student)
    "#;
    
    // ABACデータをパース
    match parser.parse(sample_abac) {
        Ok(abac_data) => {
            println!("=== Creating ABAC Solver ===");
            println!("Users: {}", abac_data.users.len());
            println!("Resources: {}", abac_data.resources.len());
            println!("Rules: {}", abac_data.rules.len());
            
            // ABACソルバーを作成
            let mut solver = UniversityAbacSolver::new(&ctx, &abac_data);
            
            println!("\n=== Generating Constraints ===");
            // 制約を生成
            match solver.generate_constraints(&abac_data) {
                Ok(()) => {
                    println!("Constraints generated successfully!");
                    
                    // 現在は基本的な属性制約のみ生成
                    // TODO: より複雑なルール制約の実装
                    println!("Basic attribute constraints have been added to the solver.");
                    println!("Advanced rule processing will be implemented in the next phase.");
                }
                Err(e) => {
                    println!("Error generating constraints: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Parse error: {}", e);
        }
    }
    
    Ok(())
}

fn edocument_abac_test(mp: &MultiProgress) -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing with edocument.abac file...");
    
    // ファイル読み込み用のプログレスバー
    let file_pb = mp.add(create_spinner("Reading edocument.abac file"));
    
    // ファイルを読み込む
    let file_content = match fs::read_to_string("data/edocument_5000.abac") {
        Ok(content) => content,
        Err(e) => {
            file_pb.finish_with_message("❌ Error reading edocument.abac file");
            println!("Error reading edocument.abac file: {}", e);
            println!("Make sure the file exists in data/edocument.abac");
            return Ok(());
        }
    };
    
    file_pb.finish_with_message("✅ File loaded successfully");
    println!("File loaded successfully. Size: {} bytes", file_content.len());
    
    // パーサーでデータを解析
    let parse_pb = mp.add(create_spinner("Parsing edocument ABAC data"));
    let parser = EdocumentAbacParser::new();
    match parser.parse(&file_content) {
        Ok(abac_data) => {
            parse_pb.finish_with_message("✅ Parsing completed successfully");
            println!("\n=== Parsing Results ===");
            println!("Users: {}", abac_data.users.len());
            println!("Resources: {}", abac_data.resources.len());
            println!("Rules: {}", abac_data.rules.len());
            
            // ユーザー一覧の表示
            println!("\n--- User Summary ---");
            let mut employee_count = 0;
            let mut helpdesk_count = 0;
            let mut admin_count = 0;
            let mut customer_count = 0;
            
            for user in &abac_data.users {
                match &user.role {
                    Some(Role::Employee) => employee_count += 1,
                    Some(Role::Helpdesk) => helpdesk_count += 1,
                    Some(Role::Admin) => admin_count += 1,
                    Some(Role::Customer) => customer_count += 1,
                    _ => {}
                }
            }
            
            println!("  Employees: {}", employee_count);
            println!("  Helpdesk: {}", helpdesk_count);
            println!("  Admins: {}", admin_count);
            println!("  Customers: {}", customer_count);
            
            // リソース一覧の表示
            println!("\n--- Resource Summary ---");
            let mut banking_note_count = 0;
            let mut traffic_fine_count = 0;
            let mut sales_offer_count = 0;
            let mut contract_count = 0;
            let mut invoice_count = 0;
            let mut paycheck_count = 0;
            
            for resource in &abac_data.resources {
                match resource.document_type {
                    DocumentType::BankingNote => banking_note_count += 1,
                    DocumentType::TrafficFine => traffic_fine_count += 1,
                    DocumentType::SalesOffer => sales_offer_count += 1,
                    DocumentType::Contract => contract_count += 1,
                    DocumentType::Invoice => invoice_count += 1,
                    DocumentType::Paycheck => paycheck_count += 1,
                    // Handle all other document types without separate counters
                    _ => {}
                }
            }
            
            println!("  Banking Notes: {}", banking_note_count);
            println!("  Traffic Fines: {}", traffic_fine_count);
            println!("  Sales Offers: {}", sales_offer_count);
            println!("  Contracts: {}", contract_count);
            println!("  Invoices: {}", invoice_count);
            println!("  Paychecks: {}", paycheck_count);
            
            // ルールの詳細
            println!("\n--- Rule Details ---");
            for rule in &abac_data.rules {
                println!("Rule {}: {}", rule.id, rule.description);
                println!("  User conditions: {}", rule.user_conditions.len());
                println!("  Resource conditions: {}", rule.resource_conditions.len());
                println!("  Actions: {:?}", rule.actions);
                println!("  Comparison conditions: {}", rule.comparison_conditions.len());
                println!();
            }
            
            // Z3ソルバーでの制約生成テスト
            println!("=== Z3 Constraint Generation Test ===");
            let ctx_pb = mp.add(create_spinner("Creating Z3 context"));
            let ctx = create_optimized_z3_context();
            ctx_pb.finish_with_message("✅ Z3 context created");
            
            let solver_pb = mp.add(create_spinner("Creating edocument ABAC solver"));
            let mut solver = EdocumentAbacSolver::new(&ctx, &abac_data);
            solver_pb.finish_with_message("✅ Edocument ABAC solver created");
            
            let constraints_pb = mp.add(create_spinner("Generating constraints"));
            match solver.generate_constraints(&abac_data) {
                Ok(()) => {
                    constraints_pb.finish_with_message("✅ Constraint generation successful!");
                    println!("✅ Constraint generation successful!");
                    println!("All {} rules processed successfully.", abac_data.rules.len());
                    
                    // 制約の詳細情報
                    println!("\n=== Constraint Generation Summary ===");
                    println!("- {} user attribute constraints generated", abac_data.users.len());
                    println!("- {} resource attribute constraints generated", abac_data.resources.len());
                    println!("- {} rule constraints generated", abac_data.rules.len());
                    
                    let total_constraints = abac_data.users.len() + abac_data.resources.len() + abac_data.rules.len();
                    println!("- Total constraints in Z3 solver: ~{}", total_constraints);
                    
                    // アクセス制御の解析を実行
                    println!("\n=== Access Control Analysis ===");
                    println!("Analyzing possible user-resource-action combinations...");
                    
                    // プログレスバーを作成（動的に長さを調整）
                    let analysis_pb = mp.add(ProgressBar::new(1000));
                    analysis_pb.set_style(
                        ProgressStyle::default_bar()
                            .template("{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                            .unwrap()
                            .progress_chars("█▉▊▋▌▍▎▏  ")
                    );
                    analysis_pb.set_message("Starting access control analysis...");
                    
                    match solver.solve_access_control_with_progress(&analysis_pb) {
                        Ok(results) => {
                            analysis_pb.finish_with_message(format!("✅ Access control analysis completed - {} combinations found", results.len()));
                            if results.is_empty() {
                                println!("No valid access combinations found.");
                            } else {
                                println!("Found {} valid access combinations:", results.len());
                                println!();
                                
                                // 結果を表示（最初の10件のみ）
                                for (i, result) in results.iter().take(10).enumerate() {
                                    println!("{}. {}", i + 1, result.display());
                                }
                                
                                if results.len() > 10 {
                                    println!("... and {} more results.", results.len() - 10);
                                }
                                
                                // 統計情報
                                println!("\n--- Access Statistics ---");
                                let mut user_access_count = std::collections::HashMap::new();
                                let mut resource_access_count = std::collections::HashMap::new();
                                let mut action_access_count = std::collections::HashMap::new();
                                
                                for result in &results {
                                    *user_access_count.entry(result.user.clone()).or_insert(0) += 1;
                                    *resource_access_count.entry(result.resource.clone()).or_insert(0) += 1;
                                    *action_access_count.entry(result.action.clone()).or_insert(0) += 1;
                                }
                                
                                println!("Users with access rights: {}", user_access_count.len());
                                println!("Resources being accessed: {}", resource_access_count.len());
                                
                                println!("\nActions being performed:");
                                for (action, count) in action_access_count {
                                    println!("  {}: {} times", action, count);
                                }
                            }
                        }
                        Err(e) => {
                            analysis_pb.finish_with_message("❌ Error in access control analysis");
                            println!("❌ Error in access control analysis: {}", e);
                        }
                    }
                    
                    println!("\n🎉 edocument.abac file processing completed successfully!");
                }
                Err(e) => {
                    constraints_pb.finish_with_message("❌ Error generating constraints");
                    println!("❌ Error generating constraints: {}", e);
                }
            }
        }
        Err(e) => {
            parse_pb.finish_with_message("❌ Parse error");
            println!("❌ Parse error: {}", e);
        }
    }
    
    Ok(())
}