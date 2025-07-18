use z3::ast::{Ast, Bool, Int};
use z3::{Context, Solver, SatResult};
use std::time::Instant;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use crate::types::{
    AbacPolicy, ParsedRule, 
    ConditionOperator, ComparisonOperator, AttributeValue
};

/// True SMT solver approach: Only loop over rules, Z3 handles user×resource combinations
/// This is the genuine SMT solver implementation that leverages Z3's symbolic reasoning
pub fn solve_abac_z3(ctx: &Context, policy: &AbacPolicy, max_users_to_check: usize) -> (Vec<(String, String, usize)>, f64) {
    let mut matches = Vec::new();
    let users_to_check = std::cmp::min(max_users_to_check, policy.users.len());
    
    let start_time = Instant::now();
    
    // Create progress bar - only for rules
    let pb = ProgressBar::new(policy.rules.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} 🔬 True SMT [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} rules ({eta})")
            .unwrap()
            .progress_chars("#>-")
    );
    
    // Pre-encode all user and resource attributes as Z3 functions (done once)
    let user_attr_functions = create_user_attribute_functions(ctx, policy);
    let resource_attr_functions = create_resource_attribute_functions(ctx, policy);
    
    // ONLY loop over rules - Z3 handles user×resource combinations
    for (rule_idx, rule) in policy.rules.iter().enumerate() {
        pb.set_message(format!("SMT solving rule {}: {}", rule_idx, rule.description));
        
        // Let Z3 find ALL user×resource combinations that satisfy this rule
        let rule_matches = find_all_combinations_with_z3(
            ctx, 
            rule, 
            rule_idx, 
            users_to_check, 
            policy.resources.len(),
            &user_attr_functions,
            &resource_attr_functions,
            policy
        );
        matches.extend(rule_matches);
        
        pb.inc(1);
    }
    
    let elapsed_time = start_time.elapsed().as_secs_f64() * 1000.0;
    pb.finish_with_message("True SMT solving completed!");
    
    (matches, elapsed_time)
}

/// Create Z3 functions that represent user attributes symbolically
/// Function signature: user_attr_name(user_id) -> attribute_value_index
fn create_user_attribute_functions<'ctx>(ctx: &'ctx Context, policy: &AbacPolicy) -> HashMap<String, z3::FuncDecl<'ctx>> {
    let mut functions = HashMap::new();
    
    // Get all unique user attribute names
    let mut attr_names = std::collections::HashSet::new();
    for user in &policy.users {
        attr_names.extend(user.attributes.keys().cloned());
    }
    
    // Create Z3 function for each attribute
    for attr_name in attr_names {
        // Function: Int -> Int (user_id -> attribute_value_index)
        let func_decl = z3::FuncDecl::new(
            ctx,
            format!("user_attr_{}", attr_name).as_str(),
            &[&z3::Sort::int(ctx)],
            &z3::Sort::int(ctx)
        );
        functions.insert(attr_name, func_decl);
    }
    
    functions
}

/// Create Z3 functions that represent resource attributes symbolically
fn create_resource_attribute_functions<'ctx>(ctx: &'ctx Context, policy: &AbacPolicy) -> HashMap<String, z3::FuncDecl<'ctx>> {
    let mut functions = HashMap::new();
    
    let mut attr_names = std::collections::HashSet::new();
    for resource in &policy.resources {
        attr_names.extend(resource.attributes.keys().cloned());
    }
    
    for attr_name in attr_names {
        let func_decl = z3::FuncDecl::new(
            ctx,
            format!("resource_attr_{}", attr_name).as_str(),
            &[&z3::Sort::int(ctx)],
            &z3::Sort::int(ctx)
        );
        functions.insert(attr_name, func_decl);
    }
    
    functions
}

/// Use Z3 to find ALL user×resource combinations that satisfy a rule (NO LOOPS!)
/// This is pure symbolic reasoning - Z3 explores the space
fn find_all_combinations_with_z3<'ctx>(
    ctx: &'ctx Context,
    rule: &ParsedRule,
    rule_idx: usize,
    user_count: usize,
    resource_count: usize,
    user_attr_functions: &HashMap<String, z3::FuncDecl<'ctx>>,
    resource_attr_functions: &HashMap<String, z3::FuncDecl<'ctx>>,
    policy: &AbacPolicy
) -> Vec<(String, String, usize)> {
    let mut matches = Vec::new();
    
    let solver = Solver::new(ctx);
    
    // Create symbolic variables
    let user_id = Int::new_const(ctx, "user_id");
    let resource_id = Int::new_const(ctx, "resource_id");
    
    // Domain constraints
    solver.assert(&user_id.ge(&Int::from_i64(ctx, 0)));
    solver.assert(&user_id.lt(&Int::from_i64(ctx, user_count as i64)));
    solver.assert(&resource_id.ge(&Int::from_i64(ctx, 0)));
    solver.assert(&resource_id.lt(&Int::from_i64(ctx, resource_count as i64)));
    
    // Add attribute function definitions (encode concrete data as Z3 axioms)
    add_user_attribute_axioms(ctx, &solver, policy, user_attr_functions);
    add_resource_attribute_axioms(ctx, &solver, policy, resource_attr_functions);
    
    // Add rule constraints symbolically (NO USER/RESOURCE LOOPS!)
    add_pure_symbolic_rule_constraints(
        ctx, 
        &solver, 
        rule, 
        &user_id, 
        &resource_id, 
        user_attr_functions,
        resource_attr_functions,
        policy
    );
    
    // Let Z3 find all solutions
    let mut iteration = 0;
    let max_iterations = std::cmp::min(1000, user_count * resource_count);
    
    while solver.check() == SatResult::Sat && iteration < max_iterations {
        if let Some(model) = solver.get_model() {
            if let (Some(user_val), Some(resource_val)) = (
                model.eval(&user_id, true).and_then(|v| v.as_i64()),
                model.eval(&resource_id, true).and_then(|v| v.as_i64())
            ) {
                let user_idx = user_val as usize;
                let resource_idx = resource_val as usize;
                
                if user_idx < user_count && resource_idx < resource_count {
                    let user = &policy.users[user_idx];
                    let resource = &policy.resources[resource_idx];
                    matches.push((user.name.clone(), resource.name.clone(), rule_idx));
                    
                    // Exclude this solution
                    solver.assert(&Bool::not(&Bool::and(ctx, &[
                        &user_id._eq(&Int::from_i64(ctx, user_val)),
                        &resource_id._eq(&Int::from_i64(ctx, resource_val))
                    ])));
                }
            }
        }
        iteration += 1;
    }
    
    matches
}

/// Add axioms that define user attribute functions based on concrete data
/// This encodes the concrete user data as Z3 logical axioms
fn add_user_attribute_axioms<'ctx>(
    ctx: &'ctx Context,
    solver: &Solver,
    policy: &AbacPolicy,
    user_attr_functions: &HashMap<String, z3::FuncDecl<'ctx>>
) {
    for (user_idx, user) in policy.users.iter().enumerate() {
        let user_id_val = Int::from_i64(ctx, user_idx as i64);
        
        for (attr_name, attr_value) in &user.attributes {
            if let Some(func_decl) = user_attr_functions.get(attr_name) {
                // Create axiom: user_attr_function(user_idx) = encoded_value
                let func_app = func_decl.apply(&[&user_id_val]);
                let encoded_value = encode_attribute_value_to_int(attr_value, policy, attr_name, true);
                let value_int = Int::from_i64(ctx, encoded_value);
                
                // Assert the axiom - convert Dynamic to Int first
                if let Some(func_app_int) = func_app.as_int() {
                    solver.assert(&func_app_int._eq(&value_int));
                }
            }
        }
    }
}

/// Add axioms that define resource attribute functions
fn add_resource_attribute_axioms<'ctx>(
    ctx: &'ctx Context,
    solver: &Solver,
    policy: &AbacPolicy,
    resource_attr_functions: &HashMap<String, z3::FuncDecl<'ctx>>
) {
    for (resource_idx, resource) in policy.resources.iter().enumerate() {
        let resource_id_val = Int::from_i64(ctx, resource_idx as i64);
        
        for (attr_name, attr_value) in &resource.attributes {
            if let Some(func_decl) = resource_attr_functions.get(attr_name) {
                let func_app = func_decl.apply(&[&resource_id_val]);
                let encoded_value = encode_attribute_value_to_int(attr_value, policy, attr_name, false);
                let value_int = Int::from_i64(ctx, encoded_value);
                
                // Convert Dynamic to Int before equality
                if let Some(func_app_int) = func_app.as_int() {
                    solver.assert(&func_app_int._eq(&value_int));
                }
            }
        }
    }
}

/// Add rule constraints using pure symbolic reasoning (NO LOOPS OVER USERS/RESOURCES!)
/// This is where the true SMT power is leveraged
fn add_pure_symbolic_rule_constraints<'ctx>(
    ctx: &'ctx Context,
    solver: &Solver,
    rule: &ParsedRule,
    user_id: &Int,
    resource_id: &Int,
    user_attr_functions: &HashMap<String, z3::FuncDecl<'ctx>>,
    resource_attr_functions: &HashMap<String, z3::FuncDecl<'ctx>>,
    policy: &AbacPolicy
) {
    let mut rule_constraints = Vec::new();
    
    // Add user condition constraints symbolically
    for condition in &rule.user_conditions {
        if let Some(func_decl) = user_attr_functions.get(&condition.attribute) {
            // Get symbolic value: user_attr_function(user_id)
            let user_attr_value = func_decl.apply(&[user_id]);
            
            // Create constraint: user_attr_value ∈ allowed_values
            let domain = get_attribute_domain(policy, &condition.attribute, true);
            let mut value_constraints = Vec::new();
            
            for allowed_value in &condition.values {
                if let Some(value_idx) = domain.iter().position(|v| v == allowed_value) {
                    if let Some(user_attr_int) = user_attr_value.as_int() {
                        value_constraints.push(
                            user_attr_int._eq(&Int::from_i64(ctx, value_idx as i64))
                        );
                    }
                }
            }
            
            if !value_constraints.is_empty() {
                let user_condition = if value_constraints.len() == 1 {
                    value_constraints[0].clone()
                } else {
                    Bool::or(ctx, &value_constraints.iter().collect::<Vec<_>>())
                };
                rule_constraints.push(user_condition);
            } else {
                // No valid values - unsatisfiable
                solver.assert(&Bool::from_bool(ctx, false));
                return;
            }
        }
    }
    
    // Add resource condition constraints symbolically
    for condition in &rule.resource_conditions {
        if let Some(func_decl) = resource_attr_functions.get(&condition.attribute) {
            let resource_attr_value = func_decl.apply(&[resource_id]);
            
            let domain = get_attribute_domain(policy, &condition.attribute, false);
            let mut value_constraints = Vec::new();
            
            for allowed_value in &condition.values {
                if let Some(value_idx) = domain.iter().position(|v| v == allowed_value) {
                    if let Some(resource_attr_int) = resource_attr_value.as_int() {
                        value_constraints.push(
                            resource_attr_int._eq(&Int::from_i64(ctx, value_idx as i64))
                        );
                    }
                }
            }
            
            if !value_constraints.is_empty() {
                let resource_condition = if value_constraints.len() == 1 {
                    value_constraints[0].clone()
                } else {
                    Bool::or(ctx, &value_constraints.iter().collect::<Vec<_>>())
                };
                rule_constraints.push(resource_condition);
            } else {
                solver.assert(&Bool::from_bool(ctx, false));
                return;
            }
        }
    }
    
    // Add comparison constraints symbolically
    for comparison in &rule.comparisons {
        match &comparison.operator {
            ComparisonOperator::Equal => {
                if let (Some(user_func), Some(resource_func)) = (
                    user_attr_functions.get(&comparison.user_attribute),
                    resource_attr_functions.get(&comparison.resource_attribute)
                ) {
                    let user_value = user_func.apply(&[user_id]);
                    let resource_value = resource_func.apply(&[resource_id]);
                    
                    // Convert both to Int for comparison
                    if let (Some(user_int), Some(resource_int)) = (user_value.as_int(), resource_value.as_int()) {
                        rule_constraints.push(user_int._eq(&resource_int));
                    }
                }
            },
            // For Contains, ContainedIn, UserIdEqual - these need special encoding
            _ => {
                // TODO: Implement symbolic encoding for complex comparison operators
                // This requires encoding StringSet operations and user name lookups
                // For now, we'll skip these complex cases in pure symbolic mode
            }
        }
    }
    
    // Assert all rule constraints
    if !rule_constraints.is_empty() {
        let overall_constraint = if rule_constraints.len() == 1 {
            rule_constraints[0].clone()
        } else {
            Bool::and(ctx, &rule_constraints.iter().collect::<Vec<_>>())
        };
        solver.assert(&overall_constraint);
    }
}

/// Encode attribute value to integer for Z3 reasoning
fn encode_attribute_value_to_int(
    attr_value: &AttributeValue,
    policy: &AbacPolicy,
    attr_name: &str,
    is_user: bool
) -> i64 {
    match attr_value {
        AttributeValue::String(s) => {
            let domain = get_attribute_domain(policy, attr_name, is_user);
            domain.iter().position(|v| v == s).unwrap_or(0) as i64
        },
        AttributeValue::Boolean(b) => if *b { 1 } else { 0 },
        AttributeValue::StringSet(_) => {
            // StringSet encoding is complex for pure symbolic reasoning
            // Would need special handling or approximation
            0
        }
    }
}

/// Get domain values for attribute encoding
fn get_attribute_domain(policy: &AbacPolicy, attr_name: &str, is_user: bool) -> Vec<String> {
    let mut values = std::collections::HashSet::new();
    
    if is_user {
        for user in &policy.users {
            if let Some(attr_value) = user.attributes.get(attr_name) {
                match attr_value {
                    AttributeValue::String(s) => { values.insert(s.clone()); },
                    AttributeValue::StringSet(set) => {
                        for val in set { values.insert(val.clone()); }
                    },
                    _ => {}
                }
            }
        }
    } else {
        for resource in &policy.resources {
            if let Some(attr_value) = resource.attributes.get(attr_name) {
                match attr_value {
                    AttributeValue::String(s) => { values.insert(s.clone()); },
                    AttributeValue::StringSet(set) => {
                        for val in set { values.insert(val.clone()); }
                    },
                    _ => {}
                }
            }
        }
    }
    
    values.into_iter().collect()
}