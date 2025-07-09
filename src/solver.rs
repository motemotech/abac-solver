use z3::ast::{Ast, Bool, Int};
use z3::{Context, Solver, SatResult};
use std::time::Instant;
use indicatif::{ProgressBar, ProgressStyle};
use crate::types::{
    AbacPolicy, ParsedRule, AttributeCondition, AttributeComparison, 
    ConditionOperator, ComparisonOperator, AttributeValue
};

pub fn solve_abac_z3(ctx: &Context, policy: &AbacPolicy, max_users_to_check: usize) -> Vec<f64> {
    let mut user_times = Vec::new();
    let users_to_check = std::cmp::min(max_users_to_check, policy.users.len());
    
    // Create progress bar
    let pb = ProgressBar::new(users_to_check as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} 🔬 Z3 [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} users ({eta})")
            .unwrap()
            .progress_chars("#>-")
    );
    
    for (u_idx, user) in policy.users.iter().enumerate().take(users_to_check) {
        let start_time = Instant::now();
        
        // Update progress bar with current user
        pb.set_message(format!("Processing user: {}", user.name));
        
        // Create a fresh Z3 solver for this user
        let solver = Solver::new(ctx);
        let resource_id = Int::new_const(ctx, "resource_id");
        let can_access = Bool::new_const(ctx, "can_access");
        
        // Domain constraints for resources
        let num_resources = policy.resources.len() as i64;
        solver.assert(&resource_id.ge(&Int::from_i64(ctx, 0)));
        solver.assert(&resource_id.lt(&Int::from_i64(ctx, num_resources)));
        
        // For this specific user, check if they can access ANY resource
        let mut accessible_resources = Vec::new();
        
        for (r_idx, _resource) in policy.resources.iter().enumerate() {
            let resource_condition = resource_id._eq(&Int::from_i64(ctx, r_idx as i64));
            
            // Check if this user-resource pair matches ANY rule
            let mut rule_matches = Vec::new();
            
            for rule in &policy.rules {
                if rule_matches_user_resource(policy, rule, u_idx, r_idx, &user.name) {
                    rule_matches.push(Bool::from_bool(ctx, true));
                } else {
                    rule_matches.push(Bool::from_bool(ctx, false));
                }
            }
            
            // Create OR condition: access allowed if ANY rule matches
            let any_rule_matches = if rule_matches.is_empty() {
                Bool::from_bool(ctx, false)
            } else if rule_matches.len() == 1 {
                rule_matches[0].clone()
            } else {
                Bool::or(ctx, &rule_matches.iter().collect::<Vec<_>>())
            };
            
            solver.assert(&Bool::implies(&resource_condition, &Bool::iff(&can_access, &any_rule_matches)));
        }
        
        // Check if this user can access any resource
        solver.push();
        solver.assert(&can_access);
        
        let mut resource_count = 0;
        while solver.check() == SatResult::Sat && resource_count < 10 { // Limit to first 10 accessible resources
            let model = solver.get_model().unwrap();
            let resource_val = model.eval(&resource_id, true).unwrap().as_i64().unwrap();
            let resource_name = &policy.resources[resource_val as usize].name;
            
            accessible_resources.push(resource_name.clone());
            resource_count += 1;
            
            // Exclude this resource for next iteration
            solver.assert(&Bool::not(&resource_id._eq(&Int::from_i64(ctx, resource_val))));
        }
        
        solver.pop(1);
        
        let elapsed = start_time.elapsed();
        user_times.push(elapsed.as_secs_f64() * 1000.0); // Convert to milliseconds
        
        // Update progress bar
        pb.inc(1);
    }
    
    pb.finish_with_message("Z3 solving completed!");
    user_times
}

fn rule_matches_user_resource(
    policy: &AbacPolicy,
    rule: &ParsedRule,
    user_idx: usize,
    resource_idx: usize,
    user_name: &str
) -> bool {
    // Check user conditions
    let user_matches = rule.user_conditions.iter().all(|condition| {
        check_user_condition(policy, user_idx, condition)
    });
    
    if !user_matches {
        return false;
    }
    
    // Check resource conditions
    let resource_matches = rule.resource_conditions.iter().all(|condition| {
        check_resource_condition(policy, resource_idx, condition)
    });
    
    if !resource_matches {
        return false;
    }
    
    // Check comparisons
    let comparison_matches = rule.comparisons.iter().all(|comparison| {
        check_comparison(policy, user_idx, resource_idx, comparison, user_name)
    });
    
    comparison_matches
}

fn check_user_condition(policy: &AbacPolicy, user_idx: usize, condition: &AttributeCondition) -> bool {
    if let Some(user_value) = policy.users[user_idx].attributes.get(&condition.attribute) {
        match (&condition.operator, user_value) {
            (ConditionOperator::In, AttributeValue::String(s)) => condition.values.contains(s),
            (ConditionOperator::In, AttributeValue::Boolean(b)) => {
                let bool_str = if *b { "True" } else { "False" };
                condition.values.contains(bool_str)
            },
            _ => false,
        }
    } else {
        false
    }
}

fn check_resource_condition(policy: &AbacPolicy, resource_idx: usize, condition: &AttributeCondition) -> bool {
    if let Some(resource_value) = policy.resources[resource_idx].attributes.get(&condition.attribute) {
        match (&condition.operator, resource_value) {
            (ConditionOperator::In, AttributeValue::String(s)) => condition.values.contains(s),
            (ConditionOperator::In, AttributeValue::Boolean(b)) => {
                let bool_str = if *b { "True" } else { "False" };
                condition.values.contains(bool_str)
            },
            _ => false,
        }
    } else {
        false
    }
}

fn check_comparison(
    policy: &AbacPolicy, 
    user_idx: usize, 
    resource_idx: usize, 
    comparison: &AttributeComparison,
    user_name: &str
) -> bool {
    match comparison.operator {
        ComparisonOperator::Contains => {
            if let Some(AttributeValue::StringSet(user_set)) = policy.users[user_idx].attributes.get(&comparison.user_attribute) {
                if let Some(AttributeValue::String(resource_val)) = policy.resources[resource_idx].attributes.get(&comparison.resource_attribute) {
                    return user_set.contains(resource_val);
                }
            }
            false
        },
        ComparisonOperator::ContainedIn => {
            match comparison.user_attribute.as_str() {
                "uid" => {
                    if let Some(AttributeValue::StringSet(resource_set)) = policy.resources[resource_idx].attributes.get(&comparison.resource_attribute) {
                        return resource_set.contains(user_name);
                    }
                    false
                },
                _ => {
                    if let Some(AttributeValue::String(user_val)) = policy.users[user_idx].attributes.get(&comparison.user_attribute) {
                        if let Some(AttributeValue::StringSet(resource_set)) = policy.resources[resource_idx].attributes.get(&comparison.resource_attribute) {
                            return resource_set.contains(user_val);
                        }
                    }
                    false
                }
            }
        },
        ComparisonOperator::Equal => {
            if let Some(user_val) = policy.users[user_idx].attributes.get(&comparison.user_attribute) {
                if let Some(resource_val) = policy.resources[resource_idx].attributes.get(&comparison.resource_attribute) {
                    match (user_val, resource_val) {
                        (AttributeValue::String(u), AttributeValue::String(r)) => return u == r,
                        _ => {}
                    }
                }
            }
            false
        },
        ComparisonOperator::UserIdEqual => {
            if let Some(AttributeValue::String(resource_val)) = policy.resources[resource_idx].attributes.get(&comparison.resource_attribute) {
                return user_name == resource_val;
            }
            false
        },
    }
}