use crate::types::{
    AbacPolicy, ParsedRule, AttributeCondition, AttributeComparison, 
    ConditionOperator, ComparisonOperator, AttributeValue
};
use std::time::Instant;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

pub fn solve_abac_loop(policy: &AbacPolicy, max_users_to_check: usize) -> (Vec<(String, String, usize)>, f64) {
    let mut matches = Vec::new();
    let users_to_check = std::cmp::min(max_users_to_check, policy.users.len());
    let total_combinations = users_to_check * policy.resources.len() * policy.rules.len();
    
    let start_time = Instant::now();
    
    // Create progress bar
    let pb = ProgressBar::new(total_combinations as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} 🔄 Loop [{elapsed_precise}] [{bar:40.yellow/orange}] {pos}/{len} combinations ({eta})")
            .unwrap()
            .progress_chars("#>-")
    );
    
    for (u_idx, user) in policy.users.iter().enumerate().take(users_to_check) {
        pb.set_message(format!("Processing user: {}", user.name));
        
        // Check this user against all resources and rules
        for (r_idx, resource) in policy.resources.iter().enumerate() {
            for (rule_idx, rule) in policy.rules.iter().enumerate() {
                if rule_matches_user_resource(policy, rule, u_idx, r_idx, &user.name) {
                    matches.push((user.name.clone(), resource.name.clone(), rule_idx));
                }
                pb.inc(1);
            }
        }
    }
    
    let elapsed_time = start_time.elapsed().as_secs_f64() * 1000.0; // Convert to milliseconds
    pb.finish_with_message("Loop enumeration completed!");
    
    (matches, elapsed_time)
}

pub fn rule_matches_user_resource(
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

pub fn check_user_condition(policy: &AbacPolicy, user_idx: usize, condition: &AttributeCondition) -> bool {
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

pub fn check_resource_condition(policy: &AbacPolicy, resource_idx: usize, condition: &AttributeCondition) -> bool {
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

pub fn check_comparison(
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

/// Parallel loop-based solver using Rayon for multi-threading
pub fn solve_abac_loop_parallel(policy: &AbacPolicy, max_users_to_check: usize) -> (Vec<(String, String, usize)>, f64) {
    let users_to_check = std::cmp::min(max_users_to_check, policy.users.len());
    let total_combinations = users_to_check * policy.resources.len() * policy.rules.len();
    
    let start_time = Instant::now();
    
    // Create progress bar
    let pb = ProgressBar::new(total_combinations as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} ⚡ Parallel [{elapsed_precise}] [{bar:40.magenta/pink}] {pos}/{len} combinations ({eta})")
            .unwrap()
            .progress_chars("#>-")
    );
    
    // Process user-resource-rule combinations in parallel
    let all_combinations: Vec<(usize, usize, usize)> = (0..users_to_check)
        .flat_map(|u_idx| {
            (0..policy.resources.len()).flat_map(move |r_idx| {
                (0..policy.rules.len()).map(move |rule_idx| (u_idx, r_idx, rule_idx))
            })
        })
        .collect();
    
    let matches: Vec<(String, String, usize)> = all_combinations
        .par_iter()
        .filter_map(|(u_idx, r_idx, rule_idx)| {
            let user = &policy.users[*u_idx];
            let resource = &policy.resources[*r_idx];
            let rule = &policy.rules[*rule_idx];
            
            if rule_matches_user_resource(policy, rule, *u_idx, *r_idx, &user.name) {
                pb.inc(1);
                Some((user.name.clone(), resource.name.clone(), *rule_idx))
            } else {
                pb.inc(1);
                None
            }
        })
        .collect();
    
    let elapsed_time = start_time.elapsed().as_secs_f64() * 1000.0; // Convert to milliseconds
    pb.finish_with_message("Parallel enumeration completed!");
    
    (matches, elapsed_time)
}

/// Optimized loop solver with pre-computed indices - enumerates all matches
pub fn solve_abac_loop_optimized(policy: &AbacPolicy, max_users_to_check: usize) -> (Vec<(String, String, usize)>, f64) {
    let mut matches = Vec::new();
    let users_to_check = std::cmp::min(max_users_to_check, policy.users.len());
    let total_combinations = users_to_check * policy.resources.len() * policy.rules.len();
    
    let start_time = Instant::now();
    
    // Pre-compute rule statistics for optimization
    let rule_stats = compute_rule_statistics(policy);
    let sorted_rules = sort_rules_by_efficiency(&rule_stats);
    
    // Create progress bar
    let pb = ProgressBar::new(total_combinations as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} 🚀 Optimized [{elapsed_precise}] [{bar:40.green/yellow}] {pos}/{len} combinations ({eta})")
            .unwrap()
            .progress_chars("#>-")
    );
    
    for (u_idx, user) in policy.users.iter().enumerate().take(users_to_check) {
        pb.set_message(format!("Processing user: {}", user.name));
        
        // Use optimized rule order
        for &rule_idx in &sorted_rules {
            let rule = &policy.rules[rule_idx];
            
            // Quick user pre-filter
            if !quick_user_match(user, rule) {
                // Still need to increment progress bar for skipped combinations
                for _ in 0..policy.resources.len() {
                    pb.inc(1);
                }
                continue;
            }
            
            // Check all resources for this rule
            for (r_idx, resource) in policy.resources.iter().enumerate() {
                if rule_matches_user_resource(policy, rule, u_idx, r_idx, &user.name) {
                    matches.push((user.name.clone(), resource.name.clone(), rule_idx));
                }
                pb.inc(1);
            }
        }
    }
    
    let elapsed_time = start_time.elapsed().as_secs_f64() * 1000.0; // Convert to milliseconds
    pb.finish_with_message("Optimized enumeration completed!");
    
    (matches, elapsed_time)
}

#[derive(Debug)]
struct RuleStatistics {
    match_rate: f64,
    avg_check_time: f64,
    user_conditions_count: usize,
    resource_conditions_count: usize,
}

fn compute_rule_statistics(policy: &AbacPolicy) -> Vec<RuleStatistics> {
    policy.rules.iter().map(|rule| {
        // Simple heuristics for rule efficiency
        let user_conditions_count = rule.user_conditions.len();
        let resource_conditions_count = rule.resource_conditions.len();
        let comparison_count = rule.comparisons.len();
        
        // Estimate complexity based on condition counts
        let complexity = user_conditions_count + resource_conditions_count + comparison_count * 2;
        
        RuleStatistics {
            match_rate: 1.0 / (complexity as f64 + 1.0), // Inverse complexity as rough match rate
            avg_check_time: complexity as f64 * 0.1,
            user_conditions_count,
            resource_conditions_count,
        }
    }).collect()
}

fn sort_rules_by_efficiency(rule_stats: &[RuleStatistics]) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..rule_stats.len()).collect();
    
    // Sort rules by estimated efficiency (high match rate, low check time)
    indices.sort_by(|&a, &b| {
        let efficiency_a = rule_stats[a].match_rate / rule_stats[a].avg_check_time;
        let efficiency_b = rule_stats[b].match_rate / rule_stats[b].avg_check_time;
        efficiency_b.partial_cmp(&efficiency_a).unwrap()
    });
    
    indices
}

fn quick_user_match(user: &crate::types::UserAttribute, rule: &ParsedRule) -> bool {
    // Quick pre-filter: check if user has any chance of matching this rule
    for condition in &rule.user_conditions {
        if let Some(user_value) = user.attributes.get(&condition.attribute) {
            match (&condition.operator, user_value) {
                (ConditionOperator::In, AttributeValue::String(s)) => {
                    if !condition.values.contains(s) {
                        return false;
                    }
                },
                (ConditionOperator::In, AttributeValue::Boolean(b)) => {
                    let bool_str = if *b { "True" } else { "False" };
                    if !condition.values.contains(bool_str) {
                        return false;
                    }
                },
                _ => return false,
            }
        } else {
            return false; // Missing required attribute
        }
    }
    true
} 

/// Memory-optimized solver with pre-computed lookup tables
pub fn solve_abac_loop_lookup(policy: &AbacPolicy, max_users_to_check: usize) -> (Vec<(String, String, usize)>, f64) {
    let mut matches = Vec::new();
    let users_to_check = std::cmp::min(max_users_to_check, policy.users.len());
    let total_combinations = users_to_check * policy.resources.len() * policy.rules.len();
    
    let start_time = Instant::now();
    
    // Pre-compute lookup tables
    let rule_lookup = build_rule_lookup_tables(policy);
    
    // Create progress bar
    let pb = ProgressBar::new(total_combinations as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} 🔍 Lookup [{elapsed_precise}] [{bar:40.blue/cyan}] {pos}/{len} combinations ({eta})")
            .unwrap()
            .progress_chars("#>-")
    );
    
    for (u_idx, user) in policy.users.iter().enumerate().take(users_to_check) {
        pb.set_message(format!("Processing user: {}", user.name));
        
        // Use pre-computed lookup tables for faster matching
        let user_matches = enumerate_user_matches_with_lookup(user, u_idx, &rule_lookup, policy, &pb);
        matches.extend(user_matches);
    }
    
    let elapsed_time = start_time.elapsed().as_secs_f64() * 1000.0; // Convert to milliseconds
    pb.finish_with_message("Lookup-based enumeration completed!");
    
    (matches, elapsed_time)
}

#[derive(Debug, Clone)]
struct RuleLookupTable {
    // Index rules by user attributes for fast filtering
    by_user_position: HashMap<String, Vec<usize>>,
    by_user_department: HashMap<String, Vec<usize>>,
    by_user_boolean_attrs: HashMap<String, Vec<usize>>,
    // Index rules by resource attributes
    by_resource_type: HashMap<String, Vec<usize>>,
    // Pre-computed rule complexity scores
    rule_complexities: Vec<f64>,
}

fn build_rule_lookup_tables(policy: &AbacPolicy) -> RuleLookupTable {
    let mut by_user_position = HashMap::new();
    let mut by_user_department = HashMap::new();
    let mut by_user_boolean_attrs = HashMap::new();
    let mut by_resource_type = HashMap::new();
    let mut rule_complexities = Vec::new();
    
    for (rule_idx, rule) in policy.rules.iter().enumerate() {
        // Index by user conditions
        for condition in &rule.user_conditions {
            match condition.attribute.as_str() {
                "position" => {
                    for value in &condition.values {
                        by_user_position.entry(value.clone()).or_insert_with(Vec::new).push(rule_idx);
                    }
                },
                "department" => {
                    for value in &condition.values {
                        by_user_department.entry(value.clone()).or_insert_with(Vec::new).push(rule_idx);
                    }
                },
                "isChair" => {
                    for value in &condition.values {
                        by_user_boolean_attrs.entry(value.clone()).or_insert_with(Vec::new).push(rule_idx);
                    }
                },
                _ => {} // Add more attributes as needed
            }
        }
        
        // Index by resource conditions
        for condition in &rule.resource_conditions {
            if condition.attribute == "type" {
                for value in &condition.values {
                    by_resource_type.entry(value.clone()).or_insert_with(Vec::new).push(rule_idx);
                }
            }
        }
        
        // Compute rule complexity
        let complexity = rule.user_conditions.len() as f64 + 
                        rule.resource_conditions.len() as f64 + 
                        rule.comparisons.len() as f64 * 1.5;
        rule_complexities.push(complexity);
    }
    
    RuleLookupTable {
        by_user_position,
        by_user_department,
        by_user_boolean_attrs,
        by_resource_type,
        rule_complexities,
    }
}

fn check_user_with_lookup(
    user: &crate::types::UserAttribute,
    user_idx: usize,
    lookup: &RuleLookupTable,
    policy: &AbacPolicy
) -> bool {
    // Get candidate rules based on user attributes
    let mut candidate_rules = HashSet::new();
    
    // Filter by position
    if let Some(AttributeValue::String(position)) = user.attributes.get("position") {
        if let Some(rules) = lookup.by_user_position.get(position) {
            candidate_rules.extend(rules);
        }
    }
    
    // Filter by department
    if let Some(AttributeValue::String(department)) = user.attributes.get("department") {
        if let Some(rules) = lookup.by_user_department.get(department) {
            if candidate_rules.is_empty() {
                candidate_rules.extend(rules);
            } else {
                candidate_rules.retain(|&rule_idx| rules.contains(&rule_idx));
            }
        }
    }
    
    // If no candidates found through indexed attributes, fall back to all rules
    if candidate_rules.is_empty() {
        candidate_rules.extend(0..policy.rules.len());
    }
    
    // Sort candidates by complexity (simpler rules first)
    let mut sorted_candidates: Vec<_> = candidate_rules.into_iter().collect();
    sorted_candidates.sort_by(|&a, &b| {
        lookup.rule_complexities[a].partial_cmp(&lookup.rule_complexities[b]).unwrap()
    });
    
    // Check candidate rules
    for &rule_idx in &sorted_candidates {
        let rule = &policy.rules[rule_idx];
        
        // Quick user match first
        if !quick_user_match(user, rule) {
            continue;
        }
        
        // Check if any resource matches
        for (r_idx, _resource) in policy.resources.iter().enumerate() {
            if rule_matches_user_resource(policy, rule, user_idx, r_idx, &user.name) {
                return true; // Found access
            }
        }
    }
    
    false
}

fn enumerate_user_matches_with_lookup(
    user: &crate::types::UserAttribute,
    user_idx: usize,
    lookup: &RuleLookupTable,
    policy: &AbacPolicy,
    pb: &ProgressBar
) -> Vec<(String, String, usize)> {
    let mut matches = Vec::new();
    
    // Get candidate rules based on user attributes
    let mut candidate_rules = HashSet::new();
    
    // Filter by position
    if let Some(AttributeValue::String(position)) = user.attributes.get("position") {
        if let Some(rules) = lookup.by_user_position.get(position) {
            candidate_rules.extend(rules);
        }
    }
    
    // Filter by department
    if let Some(AttributeValue::String(department)) = user.attributes.get("department") {
        if let Some(rules) = lookup.by_user_department.get(department) {
            if candidate_rules.is_empty() {
                candidate_rules.extend(rules);
            } else {
                candidate_rules.retain(|&rule_idx| rules.contains(&rule_idx));
            }
        }
    }
    
    // If no candidates found through indexed attributes, fall back to all rules
    if candidate_rules.is_empty() {
        candidate_rules.extend(0..policy.rules.len());
    }
    
    // Sort candidates by complexity (simpler rules first)
    let mut sorted_candidates: Vec<_> = candidate_rules.into_iter().collect();
    sorted_candidates.sort_by(|&a, &b| {
        lookup.rule_complexities[a].partial_cmp(&lookup.rule_complexities[b]).unwrap()
    });
    
    // Check all candidate rules against all resources
    for &rule_idx in &sorted_candidates {
        let rule = &policy.rules[rule_idx];
        
        // Quick user match first
        if !quick_user_match(user, rule) {
            // Still need to increment progress bar for skipped combinations
            for _ in 0..policy.resources.len() {
                pb.inc(1);
            }
            continue;
        }
        
        // Check all resources for this rule
        for (r_idx, resource) in policy.resources.iter().enumerate() {
            if rule_matches_user_resource(policy, rule, user_idx, r_idx, &user.name) {
                matches.push((user.name.clone(), resource.name.clone(), rule_idx));
            }
            pb.inc(1);
        }
    }
    
    matches
} 

/// Bitmask-based solver using bitwise operations for ultra-fast filtering
pub fn solve_abac_loop_bitmask(policy: &AbacPolicy, max_users_to_check: usize) -> (Vec<(String, String, usize)>, f64) {
    let mut matches = Vec::new();
    let users_to_check = std::cmp::min(max_users_to_check, policy.users.len());
    let total_combinations = users_to_check * policy.resources.len() * policy.rules.len();
    
    let start_time = Instant::now();
    
    // Pre-compute bitmasks for fast filtering
    let bitmasks = build_bitmask_tables(policy);
    
    // Create progress bar
    let pb = ProgressBar::new(total_combinations as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} 🎯 Bitmask [{elapsed_precise}] [{bar:40.red/orange}] {pos}/{len} combinations ({eta})")
            .unwrap()
            .progress_chars("#>-")
    );
    
    for (u_idx, user) in policy.users.iter().enumerate().take(users_to_check) {
        pb.set_message(format!("Processing user: {}", user.name));
        
        // Use bitmask operations for ultra-fast rule filtering
        let user_matches = enumerate_user_matches_with_bitmask(user, u_idx, &bitmasks, policy, &pb);
        matches.extend(user_matches);
    }
    
    let elapsed_time = start_time.elapsed().as_secs_f64() * 1000.0; // Convert to milliseconds
    pb.finish_with_message("Bitmask-based enumeration completed!");
    
    (matches, elapsed_time)
}

/// Cache-aware solver that reuses computation results
pub fn solve_abac_loop_cached(policy: &AbacPolicy, max_users_to_check: usize) -> (Vec<(String, String, usize)>, f64) {
    let mut matches = Vec::new();
    let users_to_check = std::cmp::min(max_users_to_check, policy.users.len());
    let total_combinations = users_to_check * policy.resources.len() * policy.rules.len();
    
    let start_time = Instant::now();
    
    // Initialize caches
    let mut rule_cache = HashMap::new();
    let mut user_resource_cache = HashMap::new();
    
    // Create progress bar
    let pb = ProgressBar::new(total_combinations as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} 💾 Cached [{elapsed_precise}] [{bar:40.purple/pink}] {pos}/{len} combinations ({eta})")
            .unwrap()
            .progress_chars("#>-")
    );
    
    for (u_idx, user) in policy.users.iter().enumerate().take(users_to_check) {
        pb.set_message(format!("Processing user: {}", user.name));
        
        // Use cached computations when possible
        let user_matches = enumerate_user_matches_with_cache(
            user, u_idx, policy, 
            &mut rule_cache, 
            &mut user_resource_cache,
            &pb
        );
        matches.extend(user_matches);
    }
    
    let elapsed_time = start_time.elapsed().as_secs_f64() * 1000.0; // Convert to milliseconds
    pb.finish_with_message("Cache-based enumeration completed!");
    
    (matches, elapsed_time)
}

#[derive(Debug)]
struct BitMaskTables {
    // Each rule gets a bitmask indicating which users/resources it applies to
    rule_user_masks: Vec<u64>,
    rule_resource_masks: Vec<u64>,
    user_attribute_masks: Vec<u64>,
    resource_attribute_masks: Vec<u64>,
}

fn build_bitmask_tables(policy: &AbacPolicy) -> BitMaskTables {
    let num_rules = policy.rules.len();
    let mut rule_user_masks = vec![0u64; num_rules];
    let mut rule_resource_masks = vec![0u64; num_rules];
    let mut user_attribute_masks = vec![0u64; policy.users.len()];
    let mut resource_attribute_masks = vec![0u64; policy.resources.len()];
    
    // Build user attribute masks (simplified for demonstration)
    for (u_idx, user) in policy.users.iter().enumerate() {
        let mut mask = 0u64;
        
        // Set bits based on user attributes
        if user.attributes.contains_key("position") { mask |= 1 << 0; }
        if user.attributes.contains_key("department") { mask |= 1 << 1; }
        if user.attributes.contains_key("isChair") { mask |= 1 << 2; }
        if user.attributes.contains_key("crsTaken") { mask |= 1 << 3; }
        if user.attributes.contains_key("crsTaught") { mask |= 1 << 4; }
        
        user_attribute_masks[u_idx] = mask;
    }
    
    // Build resource attribute masks
    for (r_idx, resource) in policy.resources.iter().enumerate() {
        let mut mask = 0u64;
        
        if resource.attributes.contains_key("type") { mask |= 1 << 0; }
        if resource.attributes.contains_key("crs") { mask |= 1 << 1; }
        if resource.attributes.contains_key("student") { mask |= 1 << 2; }
        if resource.attributes.contains_key("departments") { mask |= 1 << 3; }
        
        resource_attribute_masks[r_idx] = mask;
    }
    
    // Build rule masks (simplified - would need more sophisticated logic in practice)
    for (rule_idx, rule) in policy.rules.iter().enumerate() {
        let mut user_mask = 0u64;
        let mut resource_mask = 0u64;
        
        // Simple heuristic based on rule conditions
        if !rule.user_conditions.is_empty() { user_mask |= 1 << 0; }
        if !rule.resource_conditions.is_empty() { resource_mask |= 1 << 0; }
        if !rule.comparisons.is_empty() { 
            user_mask |= 1 << 1; 
            resource_mask |= 1 << 1;
        }
        
        rule_user_masks[rule_idx] = user_mask;
        rule_resource_masks[rule_idx] = resource_mask;
    }
    
    BitMaskTables {
        rule_user_masks,
        rule_resource_masks,
        user_attribute_masks,
        resource_attribute_masks,
    }
}

fn check_user_with_bitmask(
    user: &crate::types::UserAttribute,
    user_idx: usize,
    bitmasks: &BitMaskTables,
    policy: &AbacPolicy
) -> bool {
    let user_mask = bitmasks.user_attribute_masks[user_idx];
    
    // Fast rule filtering using bitwise operations
    for (rule_idx, &rule_mask) in bitmasks.rule_user_masks.iter().enumerate() {
        // Quick mask check - if no overlap, skip this rule
        if (user_mask & rule_mask) == 0 {
            continue;
        }
        
        let rule = &policy.rules[rule_idx];
        
        // Quick user match
        if !quick_user_match(user, rule) {
            continue;
        }
        
        // Check resources with bitmask acceleration
        for (r_idx, _resource) in policy.resources.iter().enumerate() {
            let resource_mask = bitmasks.resource_attribute_masks[r_idx];
            let rule_resource_mask = bitmasks.rule_resource_masks[rule_idx];
            
            // Quick resource mask check
            if (resource_mask & rule_resource_mask) == 0 {
                continue;
            }
            
            if rule_matches_user_resource(policy, rule, user_idx, r_idx, &user.name) {
                return true;
            }
        }
    }
    
    false
}

fn enumerate_user_matches_with_bitmask(
    user: &crate::types::UserAttribute,
    user_idx: usize,
    bitmasks: &BitMaskTables,
    policy: &AbacPolicy,
    pb: &ProgressBar
) -> Vec<(String, String, usize)> {
    let mut matches = Vec::new();
    let user_mask = bitmasks.user_attribute_masks[user_idx];
    
    // Fast rule filtering using bitwise operations
    for (rule_idx, &rule_mask) in bitmasks.rule_user_masks.iter().enumerate() {
        // Quick mask check - if no overlap, skip this rule
        if (user_mask & rule_mask) == 0 {
            // Still need to increment progress bar for skipped combinations
            for _ in 0..policy.resources.len() {
                pb.inc(1);
            }
            continue;
        }
        
        let rule = &policy.rules[rule_idx];
        
        // Quick user match
        if !quick_user_match(user, rule) {
            // Still need to increment progress bar for skipped combinations
            for _ in 0..policy.resources.len() {
                pb.inc(1);
            }
            continue;
        }
        
        // Check all resources with bitmask acceleration
        for (r_idx, resource) in policy.resources.iter().enumerate() {
            let resource_mask = bitmasks.resource_attribute_masks[r_idx];
            let rule_resource_mask = bitmasks.rule_resource_masks[rule_idx];
            
            // Quick resource mask check
            if (resource_mask & rule_resource_mask) != 0 {
                if rule_matches_user_resource(policy, rule, user_idx, r_idx, &user.name) {
                    matches.push((user.name.clone(), resource.name.clone(), rule_idx));
                }
            }
            pb.inc(1);
        }
    }
    
    matches
}

fn check_user_with_cache(
    user: &crate::types::UserAttribute,
    user_idx: usize,
    policy: &AbacPolicy,
    rule_cache: &mut HashMap<String, bool>,
    user_resource_cache: &mut HashMap<(usize, usize, usize), bool>,
) -> bool {
    // Try to find cached result for this user's attribute signature
    let user_signature = create_user_signature(user);
    
    if let Some(&cached_result) = rule_cache.get(&user_signature) {
        return cached_result;
    }
    
    // Compute and cache the result
    for (rule_idx, rule) in policy.rules.iter().enumerate() {
        for (r_idx, _resource) in policy.resources.iter().enumerate() {
            let cache_key = (user_idx, rule_idx, r_idx);
            
            let result = user_resource_cache
                .entry(cache_key)
                .or_insert_with(|| {
                    rule_matches_user_resource(policy, rule, user_idx, r_idx, &user.name)
                });
            
            if *result {
                rule_cache.insert(user_signature, true);
                return true;
            }
        }
    }
    
    rule_cache.insert(user_signature, false);
    false
}

fn enumerate_user_matches_with_cache(
    user: &crate::types::UserAttribute,
    user_idx: usize,
    policy: &AbacPolicy,
    _rule_cache: &mut HashMap<String, bool>,
    user_resource_cache: &mut HashMap<(usize, usize, usize), bool>,
    pb: &ProgressBar
) -> Vec<(String, String, usize)> {
    let mut matches = Vec::new();
    
    // Enumerate all combinations and use cache when possible
    for (rule_idx, rule) in policy.rules.iter().enumerate() {
        for (r_idx, resource) in policy.resources.iter().enumerate() {
            let cache_key = (user_idx, rule_idx, r_idx);
            
            let result = user_resource_cache
                .entry(cache_key)
                .or_insert_with(|| {
                    rule_matches_user_resource(policy, rule, user_idx, r_idx, &user.name)
                });
            
            if *result {
                matches.push((user.name.clone(), resource.name.clone(), rule_idx));
            }
            pb.inc(1);
        }
    }
    
    matches
}

fn create_user_signature(user: &crate::types::UserAttribute) -> String {
    // Create a simple signature based on key attributes
    let mut signature = String::new();
    
    if let Some(AttributeValue::String(pos)) = user.attributes.get("position") {
        signature.push_str(pos);
    }
    signature.push('|');
    
    if let Some(AttributeValue::String(dept)) = user.attributes.get("department") {
        signature.push_str(dept);
    }
    signature.push('|');
    
    if let Some(AttributeValue::Boolean(chair)) = user.attributes.get("isChair") {
        signature.push_str(&chair.to_string());
    }
    
    signature
} 