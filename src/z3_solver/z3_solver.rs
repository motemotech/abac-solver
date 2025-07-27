use z3::ast::{Ast, Bool, Dynamic, Int};
use z3::{Config, Context, DatatypeAccessor, DatatypeBuilder, FuncDecl, SatResult, Solver, Sort, Symbol};
use std::collections::{HashMap, HashSet};
use std::fs;

use crate::types::edocument_types::{EdocumentAbac, EdocumentRule, EdocumentUserAttribute, EdocumentResourceAttribute, AttributeName, AttributeValue, AttributeExpression};
use crate::types::types::{Condition, AttributeValueExtractor};

// This is the original example function.
pub fn how_to_use_z3_example() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");

    let zero = Int::from_i64(&ctx, 0);
    let two = Int::from_i64(&ctx, 2);
    let seven = Int::from_i64(&ctx, 7);

    let solver = Solver::new(&ctx);
    solver.assert(&x.gt(&y));
    solver.assert(&x.gt(&zero));
    solver.assert(&y.gt(&zero));
    solver.assert(&y.rem(&seven)._eq(&two));
    solver.assert(&((&x + &two).gt(&seven)));

    println!("solver: {}", solver);

    match solver.check() {
        SatResult::Sat => {
            println!("check: Sat");
            let model = solver.get_model().unwrap();
            let x_value = model.eval(&x, true).unwrap().as_i64().unwrap();
            let y_value = model.eval(&y, true).unwrap().as_i64().unwrap();

            println!("x: {}", x_value);
            println!("y: {}", y_value);
        }
        SatResult::Unsat => {
            println!("check: Unsat");
        }
        SatResult::Unknown => {
            println!("check: Unknown");
        }
    }

    Ok(())
}

/// 全ユーザー・リソースのデータを一度に読み込み、
/// ルールを満たすすべてのペアを列挙するZ3の例 (DatatypeによるClosed World版)
pub fn solve_with_z3() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("
--- Running find_all_matches_symbolic (Datatype version) ---");
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    // --- データセットのシミュレーション ---
    let users_data: HashMap<&str, HashMap<&str, i64>> = [
        ("user1", [("role", 1), ("department", 101)].iter().cloned().collect()),
        ("user2", [("role", 2), ("department", 102)].iter().cloned().collect()),
        ("user3", [("role", 1), ("department", 103)].iter().cloned().collect()),
    ].iter().cloned().collect();

    let resources_data: HashMap<&str, HashMap<&str, i64>> = [
        ("resource1", [("sensitivity", 2), ("owner_department", 101)].iter().cloned().collect()),
        ("resource2", [("sensitivity", 4), ("owner_department", 101)].iter().cloned().collect()),
        ("resource3", [("sensitivity", 1), ("owner_department", 103)].iter().cloned().collect()),
    ].iter().cloned().collect();

    // 1. Datatype を使ってUserとResourceの型を定義 (Closed World)
    let user_dt = {
        let mut builder = DatatypeBuilder::new(&ctx, Symbol::String("User".to_string()));
        for name in users_data.keys() {
            builder = builder.variant(name, vec![]);
        }
        builder.finish()
    };
    let resource_dt = {
        let mut builder = DatatypeBuilder::new(&ctx, Symbol::String("Resource".to_string()));
        for name in resources_data.keys() {
            builder = builder.variant(name, vec![]);
        }
        builder.finish()
    };

    let user_sort = user_dt.sort;
    let resource_sort = resource_dt.sort;
    let int_sort = Sort::int(&ctx);

    // 2. 属性を関数としてモデル化
    let user_role = FuncDecl::new(&ctx, "user_role", &[&user_sort], &int_sort);
    let user_department = FuncDecl::new(&ctx, "user_department", &[&user_sort], &int_sort);
    let resource_sensitivity = FuncDecl::new(&ctx, "resource_sensitivity", &[&resource_sort], &int_sort);
    let resource_owner_department = FuncDecl::new(&ctx, "resource_owner_department", &[&resource_sort], &int_sort);

    // 3. データセットを公理として読み込ませる
    // Datatypeの各ヴァリアント（user1, user2...）に対して属性値を制約として設定
    for (i, (_name, attrs)) in users_data.iter().enumerate() {
        let u_const = user_dt.variants[i].constructor.apply(&[]); // Datatypeのコンストラクタを取得
        solver.assert(&user_role.apply(&[&u_const]).as_int().unwrap()._eq(&Int::from_i64(&ctx, attrs["role"])));
        solver.assert(&user_department.apply(&[&u_const]).as_int().unwrap()._eq(&Int::from_i64(&ctx, attrs["department"])));
    }
    for (i, (_name, attrs)) in resources_data.iter().enumerate() {
        let r_const = resource_dt.variants[i].constructor.apply(&[]); // Datatypeのコンストラクタを取得
        solver.assert(&resource_sensitivity.apply(&[&r_const]).as_int().unwrap()._eq(&Int::from_i64(&ctx, attrs["sensitivity"])));
        solver.assert(&resource_owner_department.apply(&[&r_const]).as_int().unwrap()._eq(&Int::from_i64(&ctx, attrs["owner_department"])));
    }

    // 4. ルールを記号変数に対する制約として課す
    let u = Dynamic::new_const(&ctx, Symbol::String("u".to_string()), &user_sort);
    let r = Dynamic::new_const(&ctx, Symbol::String("r".to_string()), &resource_sort);

    let rule = Bool::and(&ctx, &[
        &user_role.apply(&[&u]).as_int().unwrap()._eq(&Int::from_i64(&ctx, 1)),
        &resource_sensitivity.apply(&[&r]).as_int().unwrap().lt(&Int::from_i64(&ctx, 3)),
        &user_department.apply(&[&u]).as_int().unwrap()._eq(&resource_owner_department.apply(&[&r]).as_int().unwrap())
    ]);
    solver.assert(&rule);

    println!("Solver with Datatype constraints (Closed World) setup.");
    println!("Finding all valid (user, resource) pairs...");

    // 5. すべての解を発見する (モデル列挙)
    let mut solutions = Vec::new();
    while solver.check() == SatResult::Sat {
        let model = solver.get_model().unwrap();
        let found_u = model.eval(&u, true).unwrap();
        let found_r = model.eval(&r, true).unwrap();

        // Datatypeの解はそのまま文字列として表示できる
        solutions.push((format!("{}", found_u), format!("{}", found_r)));

        // 見つかった解を禁止する制約を追加して、次の解を探す
        let new_constraint = Bool::and(&ctx, &[
            &u._eq(&found_u),
            &r._eq(&found_r)
        ]).not();
        solver.assert(&new_constraint);
    }

    if solutions.is_empty() {
        println!("No matching pairs found.");
    } else {
        println!("
Found {} matching pairs:", solutions.len());
        for (u, r) in solutions {
            println!("  - User: {}, Resource: {}", u, r);
        }
    }

    Ok(())
}

/// Solves a real-world ABAC scenario loaded from a JSON file.
/// This function initializes the Z3 solver with all users, resources, and their attributes once.
/// It then iterates through each rule, using Z3's push/pop mechanism for efficient, scoped rule evaluation.
pub fn solve_real_world_scenario(json_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Running Real-World ABAC Solver from '{}' ---", json_path);
    let mut cfg = Config::new();
    // cfg.set_bool_param_value("parallel.enable", true);
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    // 1. Load and parse the entire dataset from JSON
    let json_content = fs::read_to_string(json_path)?;
    let abac_data: EdocumentAbac = serde_json::from_str(&json_content)?;
    println!("Successfully parsed {} users, {} resources, and {} rules.",
             abac_data.users.len(), abac_data.resources.len(), abac_data.rules.len());

    // --- Base Context Setup (Done Once) ---

    // 2. Create a unified map for all attribute values (String to Int mapping)
    let (value_to_int, _int_to_value) = create_value_mappings(&abac_data);
    let get_int = |val: &AttributeValue| -> i64 { *value_to_int.get(val).unwrap_or(&-1) };

    // 3. Define User and Resource types as Datatypes (Closed World)
    let user_dt = {
        let mut builder = DatatypeBuilder::new(&ctx, Symbol::String("User".to_string()));
        for user in &abac_data.users {
            builder = builder.variant(user.user_id.as_str(), vec![]);
        }
        builder.finish()
    };
    let resource_dt = {
        let mut builder = DatatypeBuilder::new(&ctx, Symbol::String("Resource".to_string()));
        for resource in &abac_data.resources {
            builder = builder.variant(resource.resource_id.as_str(), vec![]);
        }
        builder.finish()
    };

    let user_sort = user_dt.sort;
    let resource_sort = resource_dt.sort;
    let int_sort = Sort::int(&ctx);
    let bool_sort = Sort::bool(&ctx);

    // 4. Model all unique attributes as Z3 functions
    let set_attributes: HashSet<AttributeName> = [
        AttributeName::Projects,
        AttributeName::Supervisee,
        AttributeName::PayrollingPermissions,
        AttributeName::Recipients,
    ].iter().cloned().collect();

    let mut attr_funcs: HashMap<AttributeName, (Option<Z3Func>, Option<Z3Func>)> = HashMap::new();

    for attr_name in get_all_attribute_names_enum_variants() {
        let is_set_attr = set_attributes.contains(&attr_name);
        let mut user_func = None;
        let mut resource_func = None;

        if abac_data.users.iter().any(|u| u.get_attribute_value(&attr_name).is_some() || u.get_attribute_set(&attr_name).is_some()) {
            user_func = Some(if is_set_attr {
                Z3Func::Set(FuncDecl::new(&ctx, format!("user_{}", attr_name), &[&user_sort, &int_sort], &bool_sort))
            } else {
                Z3Func::Single(FuncDecl::new(&ctx, format!("user_{}", attr_name), &[&user_sort], &int_sort))
            });
        }
        if abac_data.resources.iter().any(|r| r.get_attribute_value(&attr_name).is_some() || r.get_attribute_set(&attr_name).is_some()) {
            resource_func = Some(if is_set_attr {
                Z3Func::Set(FuncDecl::new(&ctx, format!("resource_has_{}", attr_name), &[&resource_sort, &int_sort], &bool_sort))
            } else {
                Z3Func::Single(FuncDecl::new(&ctx, format!("resource_{}", attr_name), &[&resource_sort], &int_sort))
            });
        }
        attr_funcs.insert(attr_name, (user_func, resource_func));
    }

    // 5. Assert all user and resource attributes as axioms
    for (i, user) in abac_data.users.iter().enumerate() {
        let u_const = user_dt.variants[i].constructor.apply(&[]);
        for (attr_name, (user_func_opt, _)) in &attr_funcs {
            if let Some(z3_func) = user_func_opt {
                match z3_func {
                    Z3Func::Single(func) => {
                        if let Some(val) = user.get_attribute_value(attr_name) {
                            solver.assert(&func.apply(&[&u_const]).as_int().unwrap()._eq(&Int::from_i64(&ctx, get_int(&val))));
                        }
                    },
                    Z3Func::Set(func) => {
                        let user_values: HashSet<i64> = user.get_attribute_set(attr_name)
                            .map(|s| s.iter().map(|v| get_int(v)).collect())
                            .unwrap_or_default();
                        
                        for (_, val_int) in &value_to_int {
                            let z3_val = Int::from_i64(&ctx, *val_int);
                            let has_val = func.apply(&[&u_const, &z3_val]).as_bool().unwrap();
                            if user_values.contains(val_int) {
                                solver.assert(&has_val);
                            } else {
                                solver.assert(&has_val.not());
                            }
                        }
                    }
                }
            }
        }
    }
    for (i, resource) in abac_data.resources.iter().enumerate() {
        let r_const = resource_dt.variants[i].constructor.apply(&[]);
        for (attr_name, (_, resource_func_opt)) in &attr_funcs {
            if let Some(z3_func) = resource_func_opt {
                match z3_func {
                    Z3Func::Single(func) => {
                        if let Some(val) = resource.get_attribute_value(attr_name) {
                            solver.assert(&func.apply(&[&r_const]).as_int().unwrap()._eq(&Int::from_i64(&ctx, get_int(&val))));
                        }
                    },
                    Z3Func::Set(func) => {
                        let resource_values: HashSet<i64> = resource.get_attribute_set(attr_name)
                            .map(|s| s.iter().map(|v| get_int(v)).collect())
                            .unwrap_or_default();

                        for (_, val_int) in &value_to_int {
                            let z3_val = Int::from_i64(&ctx, *val_int);
                            let has_val = func.apply(&[&r_const, &z3_val]).as_bool().unwrap();
                            if resource_values.contains(val_int) {
                                solver.assert(&has_val);
                            } else {
                                solver.assert(&has_val.not());
                            }
                        }
                    }
                }
            }
        }
    }
    println!("Base context with all data axioms is set up.");

    // --- Rule Evaluation Loop ---
    let u_var = Dynamic::new_const(&ctx, Symbol::String("u".to_string()), &user_sort);
    let r_var = Dynamic::new_const(&ctx, Symbol::String("r".to_string()), &resource_sort);

    for rule in &abac_data.rules {
        println!("--- Evaluating Rule {} ---", rule.id);
        solver.push();

        println!("translating rule to z3 constraints");
        let rule_constraint = translate_rule_to_z3(&ctx, rule, &attr_funcs, &u_var, &r_var, &get_int);
        solver.assert(&rule_constraint);
        println!("translated");

        let mut solutions = Vec::new();
        println!("start checking");
        while solver.check() == SatResult::Sat {
            let model = solver.get_model().unwrap();
            let found_u = model.eval(&u_var, true).unwrap();
            let found_r = model.eval(&r_var, true).unwrap();
            solutions.push((format!("{}", found_u), format!("{}", found_r)));
            println!("found a solution");
            let exclusion_constraint = Bool::and(&ctx, &[&u_var._eq(&found_u), &r_var._eq(&found_r)]).not();
            solver.assert(&exclusion_constraint);
        }
        println!("checked");

        if solutions.is_empty() {
            println!("Result: No matching pairs found.");
        } else {
            println!("Result: Found {} matching pairs.", solutions.len());
        }
        solver.pop(1);
    }
    Ok(())
}

// --- Helper Functions ---

pub enum Z3Func<'a> {
    Single(FuncDecl<'a>), // Entity -> Int
    Set(FuncDecl<'a>),    // (Entity, Int) -> Bool
}

enum AttributeContext {
    User,
    Resource,
    Comparison,
}

fn create_value_mappings(data: &EdocumentAbac) -> (HashMap<AttributeValue, i64>, HashMap<i64, AttributeValue>) {
    let mut value_to_int = HashMap::new();
    let mut int_to_value = HashMap::new();
    let mut counter = 0;

    let mut insert_val = |val: &AttributeValue| {
        if !value_to_int.contains_key(val) {
            value_to_int.insert(val.clone(), counter);
            int_to_value.insert(counter, val.clone());
            counter += 1;
        }
    };

    for user in &data.users {
        for attr_name in get_all_attribute_names_enum_variants() {
            if let Some(val) = user.get_attribute_value(&attr_name) { insert_val(&val); }
            if let Some(set) = user.get_attribute_set(&attr_name) { set.iter().for_each(|v| insert_val(v)); }
        }
    }
    for resource in &data.resources {
        for attr_name in get_all_attribute_names_enum_variants() {
            if let Some(val) = resource.get_attribute_value(&attr_name) { insert_val(&val); }
            if let Some(set) = resource.get_attribute_set(&attr_name) { set.iter().for_each(|v| insert_val(v)); }
        }
    }
    for rule in &data.rules {
        for cond in rule.user_conditions.iter().chain(rule.resource_conditions.iter()).chain(rule.comparison_conditions.iter()) {
            if let AttributeExpression::AttributeValue(val) = &cond.right { insert_val(val); }
            if let AttributeExpression::ValueSet(vals) = &cond.right { vals.iter().for_each(|v| insert_val(v)); }
        }
    }
    (value_to_int, int_to_value)
}

fn get_all_attribute_names_enum_variants() -> HashSet<AttributeName> {
    use crate::types::edocument_types::AttributeName::*;
    [
        Role, Position, Tenant, Department, Office, Registered, Projects,
        Supervisor, Supervisee, PayrollingPermissions, ClearanceLevel,
        SecurityLevel, Type, Owner, Recipients, IsConfidential,
        ContainsPersonalInfo, Uid, Rid,
    ].iter().cloned().collect()
}

fn translate_rule_to_z3<'a>(
    ctx: &'a Context,
    rule: &EdocumentRule,
    attr_funcs: &HashMap<AttributeName, (Option<Z3Func<'a>>, Option<Z3Func<'a>>)>,
    u_var: &Dynamic<'a>,
    r_var: &Dynamic<'a>,
    get_int: &impl Fn(&AttributeValue) -> i64,
) -> Bool<'a> {
    let mut all_conditions = Vec::new();
    for cond in &rule.user_conditions {
        all_conditions.push(translate_condition(ctx, cond, attr_funcs, u_var, r_var, get_int, &AttributeContext::User));
    }
    for cond in &rule.resource_conditions {
        all_conditions.push(translate_condition(ctx, cond, attr_funcs, u_var, r_var, get_int, &AttributeContext::Resource));
    }
    for cond in &rule.comparison_conditions {
        all_conditions.push(translate_condition(ctx, cond, attr_funcs, u_var, r_var, get_int, &AttributeContext::Comparison));
    }
    Bool::and(ctx, &all_conditions.iter().collect::<Vec<_>>())
}

fn translate_condition<'a>(
    ctx: &'a Context,
    condition: &Condition<AttributeExpression>,
    attr_funcs: &HashMap<AttributeName, (Option<Z3Func<'a>>, Option<Z3Func<'a>>)>,
    u_var: &Dynamic<'a>,
    r_var: &Dynamic<'a>,
    get_int: &impl Fn(&AttributeValue) -> i64,
    context: &AttributeContext,
) -> Bool<'a> {
    use crate::types::types::ComparisonOperator::*;

    match condition.operator {
        Contains | ContainedIn => {
            let (scalar_expr, set_expr) = if condition.operator == ContainedIn {
                (&condition.left, &condition.right)
            } else {
                (&condition.right, &condition.left)
            };

            let scalar_z3 = match translate_expr_to_int(ctx, scalar_expr, attr_funcs, u_var, r_var, get_int, context) {
                Some(s) => s,
                None => return Bool::from_bool(ctx, false),
            };

            match set_expr {
                AttributeExpression::AttributeName(name) => {
                    let (user_func_opt, resource_func_opt) = attr_funcs.get(name).unwrap();
                    let z3_func_opt = match context {
                        AttributeContext::User => user_func_opt.as_ref(),
                        AttributeContext::Resource => resource_func_opt.as_ref(),
                        AttributeContext::Comparison => user_func_opt.as_ref().or(resource_func_opt.as_ref()),
                    };
                    if let Some(Z3Func::Set(set_func)) = z3_func_opt {
                        let entity_var = match context {
                            AttributeContext::User => u_var,
                            AttributeContext::Resource => r_var,
                            AttributeContext::Comparison => if user_func_opt.is_some() { u_var } else { r_var },
                        };
                        set_func.apply(&[entity_var, &scalar_z3]).as_bool().unwrap()
                    } else {
                        Bool::from_bool(ctx, false)
                    }
                },
                AttributeExpression::ValueSet(values) => {
                    let or_clauses: Vec<Bool> = values.iter()
                        .map(|v| scalar_z3._eq(&Int::from_i64(ctx, get_int(v))))
                        .collect();
                    Bool::or(ctx, &or_clauses.iter().collect::<Vec<_>>())
                },
                _ => Bool::from_bool(ctx, false),
            }
        },
        _ => {
            let left = translate_expr_to_int(ctx, &condition.left, attr_funcs, u_var, r_var, get_int, context);
            let right = translate_expr_to_int(ctx, &condition.right, attr_funcs, u_var, r_var, get_int, context);

            if let (Some(left), Some(right)) = (left, right) {
                match condition.operator {
                    Equals => left._eq(&right),
                    GreaterThan => left.gt(&right),
                    LessThan => left.lt(&right),
                    GreaterThanOrEqual => left.ge(&right),
                    LessThanOrEqual => left.le(&right),
                    _ => Bool::from_bool(ctx, false),
                }
            } else {
                Bool::from_bool(ctx, false)
            }
        }
    }
}

fn translate_expr_to_int<'a>(
    ctx: &'a Context,
    expr: &AttributeExpression,
    attr_funcs: &HashMap<AttributeName, (Option<Z3Func<'a>>, Option<Z3Func<'a>>)>,
    u_var: &Dynamic<'a>,
    r_var: &Dynamic<'a>,
    get_int: &impl Fn(&AttributeValue) -> i64,
    context: &AttributeContext,
) -> Option<Int<'a>> {
    match expr {
        AttributeExpression::AttributeName(name) => {
            let (user_func_opt, resource_func_opt) = attr_funcs.get(name)?;
            let z3_func_opt = match context {
                AttributeContext::User => user_func_opt.as_ref(),
                AttributeContext::Resource => resource_func_opt.as_ref(),
                AttributeContext::Comparison => user_func_opt.as_ref().or(resource_func_opt.as_ref()),
            };
            if let Some(Z3Func::Single(func)) = z3_func_opt {
                let entity_var = match context {
                    AttributeContext::User => u_var,
                    AttributeContext::Resource => r_var,
                    AttributeContext::Comparison => if user_func_opt.is_some() { u_var } else { r_var },
                };
                Some(func.apply(&[entity_var]).as_int().unwrap())
            } else {
                None
            }
        }
        AttributeExpression::AttributeValue(val) => Some(Int::from_i64(ctx, get_int(val))),
        AttributeExpression::ValueSet(_) => None,
    }
}