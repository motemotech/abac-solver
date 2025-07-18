use std::collections::{HashMap, HashSet};
use crate::types::{self, AttributeValueExtractor, Condition, ComparisonOperator};
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

// Import specific types to avoid ambiguity
use crate::university_types::{
    UniversityAbac, UniversityRule, UniversityUserAttribute, UniversityResourceAttribute,
    AttributeName as UniversityAttributeName, AttributeValue as UniversityAttributeValue,
    AttributeExpression as UniversityAttributeExpression
};
use crate::edocument_types::{
    EdocumentAbac, EdocumentRule, EdocumentUserAttribute, EdocumentResourceAttribute,
    AttributeName as EdocAttributeName, AttributeValue as EdocAttributeValue, 
    AttributeExpression as EdocAttributeExpression
};

// Common trait for Rule types
pub trait Rule {
    type AttributeExpression;
    
    fn get_user_conditions(&self) -> &Vec<Condition<Self::AttributeExpression>>;
    fn get_resource_conditions(&self) -> &Vec<Condition<Self::AttributeExpression>>;
    fn get_comparison_conditions(&self) -> &Vec<Condition<Self::AttributeExpression>>;
    fn id(&self) -> usize;
}

// Generic condition value enum
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GenericConditionValue<T> {
    Single(T),
    Set(Vec<T>),
    None,
}

// Type aliases for domain-specific condition values
pub type UniversityConditionValue = GenericConditionValue<UniversityAttributeValue>;
pub type EdocumentConditionValue = GenericConditionValue<EdocAttributeValue>;

// Generic evaluation function
fn evaluate_condition<L, R, Expr, AttrVal>(
    left_source: L,
    right_source: R,
    condition: &Condition<Expr>,
    get_left_value: impl Fn(L, &Expr) -> Result<GenericConditionValue<AttrVal>, Box<dyn std::error::Error + Send + Sync>>,
    get_right_value: impl Fn(R, &Expr) -> Result<GenericConditionValue<AttrVal>, Box<dyn std::error::Error + Send + Sync>>,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>
where
    AttrVal: PartialEq + std::fmt::Debug,
{
    let left_value = get_left_value(left_source, &condition.left)?;
    let right_value = get_right_value(right_source, &condition.right)?;

    match condition.operator {
        ComparisonOperator::ContainedIn => evaluate_contained_in(&left_value, &right_value),
        ComparisonOperator::Contains => evaluate_contains(&left_value, &right_value),
        ComparisonOperator::Equals => evaluate_equals(&left_value, &right_value),
    }
}

// Ruleトレイトを実装するためのマクロ
macro_rules! impl_rule {
    ($rule_type:ty, $attr_expr_type:ty) => {
        impl Rule for $rule_type {
            type AttributeExpression = $attr_expr_type;

            fn get_user_conditions(&self) -> &Vec<Condition<Self::AttributeExpression>> {
                &self.user_conditions
            }

            fn get_resource_conditions(&self) -> &Vec<Condition<Self::AttributeExpression>> {
                &self.resource_conditions
            }

            fn get_comparison_conditions(&self) -> &Vec<Condition<Self::AttributeExpression>> {
                &self.comparison_conditions
            }

            fn id(&self) -> usize {
                self.id
            }
        }
    };
}

// マクロを使用してRuleトレイトを実装
impl_rule!(UniversityRule, UniversityAttributeExpression);
impl_rule!(EdocumentRule, EdocAttributeExpression);

// Generic trait for ABAC analysis
pub trait AbacAnalyzer {
    type UserAttribute: Clone;
    type ResourceAttribute: Clone;
    type Rule: Rule<AttributeExpression = Self::AttributeExpression>;
    type AttributeExpression;
    
    fn get_users(&self) -> &Vec<Self::UserAttribute>;
    fn get_resources(&self) -> &Vec<Self::ResourceAttribute>;
    fn get_rules(&self) -> &Vec<Self::Rule>;
    
    fn evaluate_user_condition(&self, user: &Self::UserAttribute, condition: &Condition<Self::AttributeExpression>) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;
    fn evaluate_resource_condition(&self, resource: &Self::ResourceAttribute, condition: &Condition<Self::AttributeExpression>) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;
    fn evaluate_comparison_condition(&self, user: &Self::UserAttribute, resource: &Self::ResourceAttribute, condition: &Condition<Self::AttributeExpression>) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;
}

// Implementation for University domain
impl AbacAnalyzer for UniversityAbac {
    type UserAttribute = UniversityUserAttribute;
    type ResourceAttribute = UniversityResourceAttribute;
    type Rule = UniversityRule;
    type AttributeExpression = UniversityAttributeExpression;
    
    fn get_users(&self) -> &Vec<Self::UserAttribute> {
        &self.users
    }
    
    fn get_resources(&self) -> &Vec<Self::ResourceAttribute> {
        &self.resources
    }
    
    fn get_rules(&self) -> &Vec<Self::Rule> {
        &self.rules
    }
    
    fn evaluate_user_condition(&self, user: &Self::UserAttribute, condition: &Condition<Self::AttributeExpression>) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        evaluate_condition(
            user,
            &(), // Pass a dummy value for the right source
            condition,
            |u, expr| get_user_attribute_value(u, expr),
            |_, expr| get_condition_value(expr),
        )
    }
    
    fn evaluate_resource_condition(&self, resource: &Self::ResourceAttribute, condition: &Condition<Self::AttributeExpression>) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        evaluate_condition(
            resource,
            &(), // Pass a dummy value for the right source
            condition,
            |r, expr| get_resource_attribute_value(r, expr),
            |_, expr| get_condition_value(expr),
        )
    }
    
    fn evaluate_comparison_condition(&self, user: &Self::UserAttribute, resource: &Self::ResourceAttribute, condition: &Condition<Self::AttributeExpression>) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        evaluate_condition(
            (user, resource),
            (user, resource),
            condition,
            |(u, r), expr| get_comparison_attribute_value(u, r, expr),
            |(u, r), expr| get_comparison_attribute_value(u, r, expr),
        )
    }
}

// Implementation for Edocument domain
impl AbacAnalyzer for EdocumentAbac {
    type UserAttribute = EdocumentUserAttribute;
    type ResourceAttribute = EdocumentResourceAttribute;
    type Rule = EdocumentRule;
    type AttributeExpression = EdocAttributeExpression;
    
    fn get_users(&self) -> &Vec<Self::UserAttribute> {
        &self.users
    }
    
    fn get_resources(&self) -> &Vec<Self::ResourceAttribute> {
        &self.resources
    }
    
    fn get_rules(&self) -> &Vec<Self::Rule> {
        &self.rules
    }
    
    fn evaluate_user_condition(&self, user: &Self::UserAttribute, condition: &Condition<Self::AttributeExpression>) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        evaluate_condition(
            user,
            &(), // Pass a dummy value
            condition,
            |u, expr| get_user_attribute_value(u, expr),
            |_, expr| get_condition_value(expr),
        )
    }
    
    fn evaluate_resource_condition(&self, resource: &Self::ResourceAttribute, condition: &Condition<Self::AttributeExpression>) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        evaluate_condition(
            resource,
            &(), // Pass a dummy value
            condition,
            |r, expr| get_resource_attribute_value(r, expr),
            |_, expr| get_condition_value(expr),
        )
    }
    
    fn evaluate_comparison_condition(&self, user: &Self::UserAttribute, resource: &Self::ResourceAttribute, condition: &Condition<Self::AttributeExpression>) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        evaluate_condition(
            (user, resource),
            (user, resource),
            condition,
            |(u, r), expr| get_comparison_attribute_value(u, r, expr),
            |(u, r), expr| get_comparison_attribute_value(u, r, expr),
        )
    }
}

pub fn simple_loop<T: AbacAnalyzer>(
    abac_data: T,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    T::UserAttribute: std::fmt::Debug,
    T::ResourceAttribute: std::fmt::Debug,
    T::Rule: std::fmt::Debug,
{
    let users = abac_data.get_users();
    let resources = abac_data.get_resources();
    let rules = abac_data.get_rules();

    let rule_start_time = std::time::Instant::now();
    for rule in rules {
        let user_conditions = rule.get_user_conditions();
        let resource_conditions = rule.get_resource_conditions();
        let comparison_conditions = rule.get_comparison_conditions();

        let mut validated_users = Vec::new();
        
        // ユーザ条件の処理
        if user_conditions.is_empty() {
            // ユーザ条件がない場合は、全ユーザを対象とする
            validated_users = (*users).clone();
        } else {
            // 複数の条件は AND 条件として処理
            for user in users {
                let mut all_conditions_met = true;
                
                for user_condition in user_conditions.iter() {
                    if !abac_data.evaluate_user_condition(user, user_condition)? {
                        all_conditions_met = false;
                        break;
                    }
                }
                
                if all_conditions_met {
                    validated_users.push(user.clone());
                }
            }
        }

        let mut validated_resources = Vec::new();
        
        // リソース条件の処理
        if resource_conditions.is_empty() {
            // リソース条件がない場合は、全リソースを対象とする
            validated_resources = (*resources).clone();
        } else {
            // 複数の条件は AND 条件として処理
            for resource in resources {
                let mut all_conditions_met = true;
                
                for resource_condition in resource_conditions.iter() {
                    if !abac_data.evaluate_resource_condition(resource, resource_condition)? {
                        all_conditions_met = false;
                        break;
                    }
                }
                
                if all_conditions_met {
                    validated_resources.push(resource.clone());
                }
            }
        }

        // 比較条件の処理：ユーザとリソースの組み合わせを評価
        let mut valid_combinations = Vec::new();
        
        for user in &validated_users {
            for resource in &validated_resources {
                let mut all_conditions_met = true;
                
                // 比較条件がない場合は、全ての組み合わせが有効
                if comparison_conditions.is_empty() {
                    valid_combinations.push((user.clone(), resource.clone()));
                    continue;
                }
                
                // 全ての比較条件をチェック
                for comparison_condition in comparison_conditions.iter() {
                    if !abac_data.evaluate_comparison_condition(user, resource, comparison_condition)? {
                        all_conditions_met = false;
                        break;
                    }
                }
                
                if all_conditions_met {
                    valid_combinations.push((user.clone(), resource.clone()));
                }
            }
        }

        println!("Rule {}: Valid (user, resource) combinations count: {}", rule.id(), valid_combinations.len());
    }
    let rule_duration = rule_start_time.elapsed();
    println!("Rule processing time: {:.2?}", rule_duration);
    
    Ok(())
}

pub fn improved_simple_loop<T, N, V>(
    abac_data: T,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    T: AbacAnalyzer<AttributeExpression = crate::types::AttributeExpression<N, V>>,
    T::UserAttribute: std::fmt::Debug + AttributeValueExtractor<AttributeName = N, AttributeValue = V>,
    T::ResourceAttribute: std::fmt::Debug + AttributeValueExtractor<AttributeName = N, AttributeValue = V>,
    T::Rule: std::fmt::Debug,
    N: Eq + std::hash::Hash + Clone,
    V: Eq + std::hash::Hash + Clone,
{
    let users = abac_data.get_users();
    let resources = abac_data.get_resources();
    let rules = abac_data.get_rules();

    let rule_start_time = std::time::Instant::now();
    for rule in rules {
        let user_conditions = rule.get_user_conditions();
        let resource_conditions = rule.get_resource_conditions();
        let comparison_conditions = rule.get_comparison_conditions();

        let mut validated_users = Vec::new();
        if user_conditions.is_empty() {
            validated_users = (*users).clone();
        } else {
            for user in users {
                let mut all_conditions_met = true;
                for user_condition in user_conditions.iter() {
                    if !abac_data.evaluate_user_condition(user, user_condition)? {
                        all_conditions_met = false;
                        break;
                    }
                }
                if all_conditions_met {
                    validated_users.push(user.clone());
                }
            }
        }

        println!("Validated users count: {}", validated_users.len());

        let mut validated_resources = Vec::new();
        if resource_conditions.is_empty() {
            validated_resources = (*resources).clone();
        } else {
            for resource in resources {
                let mut all_conditions_met = true;
                for resource_condition in resource_conditions.iter() {
                    if !abac_data.evaluate_resource_condition(resource, resource_condition)? {
                        all_conditions_met = false;
                        break;
                    }
                }
                if all_conditions_met {
                    validated_resources.push(resource.clone());
                }
            }
        }

        println!("Validated resources count: {}", validated_resources.len());

        let mut valid_combinations_count = 0;

        if comparison_conditions.is_empty() {
            valid_combinations_count = validated_users.len() * validated_resources.len();
        } else {
            // Separate comparison conditions into Equals and others
            let (equals_conditions, other_conditions): (Vec<&Condition<T::AttributeExpression>>, Vec<&Condition<T::AttributeExpression>>) =
                comparison_conditions.iter().partition(|cond| cond.operator == ComparisonOperator::Equals);

            if !equals_conditions.is_empty() {
                // Phase 1: Indexed matching for Equals conditions
                let mut resource_map: HashMap<Vec<V>, Vec<&T::ResourceAttribute>> = HashMap::new();
                let mut resource_key_extractors: Vec<Box<dyn Fn(&T::ResourceAttribute) -> Option<V>>> = Vec::new();
                let mut user_key_extractors: Vec<Box<dyn Fn(&T::UserAttribute) -> Option<V>>> = Vec::new();

                for cond in &equals_conditions {
                    if let (Some(key_attr_name), Some(value_attr_name)) = (get_attribute_name(&cond.left), get_attribute_name(&cond.right)) {
                        resource_key_extractors.push(Box::new(move |res: &T::ResourceAttribute| res.get_attribute_value(&key_attr_name)));
                        user_key_extractors.push(Box::new(move |usr: &T::UserAttribute| usr.get_attribute_value(&value_attr_name)));
                    }
                }

                // Build composite key for resources
                for resource in &validated_resources {
                    let mut composite_key_parts = Vec::new();
                    for extractor in &resource_key_extractors {
                        if let Some(val) = extractor(resource) {
                            composite_key_parts.push(val);
                        } else {
                            composite_key_parts.clear();
                            break;
                        }
                    }
                    if !composite_key_parts.is_empty() {
                        resource_map.entry(composite_key_parts).or_insert_with(Vec::new).push(resource);
                    }
                }

                for user in &validated_users {
                    let mut composite_user_key_parts = Vec::new();
                    for extractor in &user_key_extractors {
                        if let Some(val) = extractor(user) {
                            composite_user_key_parts.push(val);
                        } else {
                            composite_user_key_parts.clear();
                            break;
                        }
                    }

                    if !composite_user_key_parts.is_empty() {
                        if let Some(matched_resources) = resource_map.get(&composite_user_key_parts) {
                            // Phase 2: Evaluate remaining conditions for matched pairs
                            if other_conditions.is_empty() {
                                valid_combinations_count += matched_resources.len();
                            } else {
                                for res in matched_resources {
                                    let mut all_other_conditions_met = true;
                                    for other_cond in &other_conditions {
                                        if !abac_data.evaluate_comparison_condition(user, res, other_cond)? {
                                            all_other_conditions_met = false;
                                            break;
                                        }
                                    }
                                    if all_other_conditions_met {
                                        valid_combinations_count += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                // No Equals conditions, fallback to simple loop for all comparison conditions
                for user in &validated_users {
                    for resource in &validated_resources {
                        let mut all_conditions_met = true;
                        for comp_condition in comparison_conditions {
                            if !abac_data.evaluate_comparison_condition(user, resource, comp_condition)? {
                                all_conditions_met = false;
                                break;
                            }
                        }
                        if all_conditions_met {
                            valid_combinations_count += 1;
                        }
                    }
                }
            }
        }

        println!("Rule {}: Valid (user, resource) combinations count: {}", rule.id(), valid_combinations_count);
    }
    let rule_duration = rule_start_time.elapsed();
    println!("Rule processing time: {:.2?}", rule_duration);

    Ok(())
}

pub fn parallel_indexed_loop<T, N, V>(
    abac_data: T,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: AbacAnalyzer<AttributeExpression = crate::types::AttributeExpression<N, V>> + Send + Sync,
    T::UserAttribute: std::fmt::Debug + AttributeValueExtractor<AttributeName = N, AttributeValue = V> + Send + Sync,
    T::ResourceAttribute: std::fmt::Debug + AttributeValueExtractor<AttributeName = N, AttributeValue = V> + Send + Sync,
    T::Rule: std::fmt::Debug + Send + Sync,
    N: Eq + std::hash::Hash + Clone + Send + Sync,
    V: Eq + std::hash::Hash + Clone + Send + Sync,
    <T as AbacAnalyzer>::UserAttribute: Send + Sync,
    <T as AbacAnalyzer>::ResourceAttribute: Send + Sync,
{
    let users = abac_data.get_users();
    let resources = abac_data.get_resources();
    let rules = abac_data.get_rules();

    let rule_start_time = std::time::Instant::now();

    // Collect results from parallel processing
    let results: Vec<Result<(usize, usize), Box<dyn std::error::Error + Send + Sync>>> = rules.par_iter().map(|rule| {
        let user_conditions = rule.get_user_conditions();
        let resource_conditions = rule.get_resource_conditions();
        let comparison_conditions = rule.get_comparison_conditions();

        let validated_users: Vec<T::UserAttribute> = if user_conditions.is_empty() {
            users.clone()
        } else {
            users.par_iter().filter_map(|user| {
                let mut all_conditions_met = true;
                for user_condition in user_conditions.iter() {
                    match abac_data.evaluate_user_condition(user, user_condition) {
                        Ok(true) => {},
                        Ok(false) => {
                            all_conditions_met = false;
                            break;
                        },
                        Err(_) => {
                            all_conditions_met = false; // Treat error as failed condition
                            break;
                        }
                    }
                }
                if all_conditions_met { Some(user.clone()) } else { None }
            }).collect()
        };
        println!("Validated users count: {}", validated_users.len());

        let validated_resources: Vec<T::ResourceAttribute> = if resource_conditions.is_empty() {
            resources.clone()
        } else {
            resources.par_iter().filter_map(|resource| {
                let mut all_conditions_met = true;
                for resource_condition in resource_conditions.iter() {
                    match abac_data.evaluate_resource_condition(resource, resource_condition) {
                        Ok(true) => {},
                        Ok(false) => {
                            all_conditions_met = false;
                            break;
                        },
                        Err(_) => {
                            all_conditions_met = false; // Treat error as failed condition
                            break;
                        }
                    }
                }
                if all_conditions_met { Some(resource.clone()) } else { None }
            }).collect()
        };
        println!("Validated resources count: {}", validated_resources.len());

        let mut valid_combinations_count = 0;

        if comparison_conditions.is_empty() {
            valid_combinations_count = validated_users.len() * validated_resources.len();
        } else {
            // Separate comparison conditions into Equals and others
            let (equals_conditions, other_conditions): (Vec<&Condition<T::AttributeExpression>>, Vec<&Condition<T::AttributeExpression>>) =
                comparison_conditions.iter().partition(|cond| cond.operator == ComparisonOperator::Equals);

            if !equals_conditions.is_empty() {
                // Phase 1: Indexed matching for Equals conditions
                let mut resource_map: HashMap<Vec<V>, Vec<&T::ResourceAttribute>> = HashMap::new();
                // Extractors need to be Send + Sync
                let mut resource_key_extractors: Vec<Box<dyn Fn(&T::ResourceAttribute) -> Option<V> + Send + Sync>> = Vec::new();
                let mut user_key_extractors: Vec<Box<dyn Fn(&T::UserAttribute) -> Option<V> + Send + Sync>> = Vec::new();

                for cond in &equals_conditions {
                    if let (Some(key_attr_name), Some(value_attr_name)) = (get_attribute_name(&cond.left), get_attribute_name(&cond.right)) {
                        // Assuming left is resource attribute and right is user attribute for indexing
                        resource_key_extractors.push(Box::new(move |res: &T::ResourceAttribute| res.get_attribute_value(&key_attr_name)));
                        user_key_extractors.push(Box::new(move |usr: &T::UserAttribute| usr.get_attribute_value(&value_attr_name)));
                    }
                }

                // Build composite key for resources
                for resource in &validated_resources {
                    let mut composite_key_parts = Vec::new();
                    for extractor in &resource_key_extractors {
                        if let Some(val) = extractor(resource) {
                            composite_key_parts.push(val);
                        } else {
                            composite_key_parts.clear();
                            break;
                        }
                    }
                    if !composite_key_parts.is_empty() {
                        resource_map.entry(composite_key_parts).or_insert_with(Vec::new).push(resource);
                    }
                }

                for user in &validated_users {
                    let mut composite_user_key_parts = Vec::new();
                    for extractor in &user_key_extractors {
                        if let Some(val) = extractor(user) {
                            composite_user_key_parts.push(val);
                        } else {
                            composite_user_key_parts.clear();
                            break;
                        }
                    }

                    if !composite_user_key_parts.is_empty() {
                        if let Some(matched_resources) = resource_map.get(&composite_user_key_parts) {
                            // Phase 2: Evaluate remaining conditions for matched pairs
                            if other_conditions.is_empty() {
                                valid_combinations_count += matched_resources.len();
                            } else {
                                for res in matched_resources {
                                    let mut all_other_conditions_met = true;
                                    for other_cond in &other_conditions {
                                        match abac_data.evaluate_comparison_condition(user, res, other_cond) {
                                            Ok(true) => {},
                                            Ok(false) => {
                                                all_other_conditions_met = false;
                                                break;
                                            },
                                            Err(_) => {
                                                all_other_conditions_met = false; // Treat error as failed condition
                                                break;
                                            }
                                        }
                                    }
                                    if all_other_conditions_met {
                                        valid_combinations_count += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                // No Equals conditions, fallback to simple loop for all comparison conditions
                for user in &validated_users {
                    for resource in &validated_resources {
                        let mut all_conditions_met = true;
                        for comp_condition in comparison_conditions {
                            match abac_data.evaluate_comparison_condition(user, resource, comp_condition) {
                                Ok(true) => {},
                                Ok(false) => {
                                    all_conditions_met = false;
                                    break;
                                },
                                Err(_) => {
                                    all_conditions_met = false; // Treat error as failed condition
                                    break;
                                }
                            }
                        }
                        if all_conditions_met {
                            valid_combinations_count += 1;
                        }
                    }
                }
            }
        }
        println!("Rule {}: Valid (user, resource) combinations count: {}", rule.id(), valid_combinations_count);

        Ok((rule.id(), valid_combinations_count))
    }).collect(); // Collect results from parallel processing

    // Print results and handle errors
    for result in results {
        match result {
            Ok((rule_id, count)) => println!("Rule {}: Valid (user, resource) combinations count: {}", rule_id, count),
            Err(e) => eprintln!("Error during rule processing: {}", e),
        }
    }

    let rule_duration = rule_start_time.elapsed();
    println!("Rule processing time: {:.2?}", rule_duration);

    Ok(())
}

fn get_attribute_name<N, V>(attr_expr: &crate::types::AttributeExpression<N, V>) -> Option<N>
where
    N: Clone,
{
    if let crate::types::AttributeExpression::AttributeName(name) = attr_expr {
        Some(name.clone())
    } else {
        None
    }
}

fn get_resource_attribute_value<R>(
    resource: &R,
    attr_expr: &crate::types::AttributeExpression<R::AttributeName, R::AttributeValue>,
) -> Result<GenericConditionValue<R::AttributeValue>, Box<dyn std::error::Error + Send + Sync>>
where
    R: AttributeValueExtractor,
    R::AttributeValue: Clone,
{
    match attr_expr {
        crate::types::AttributeExpression::AttributeName(attr_name) => {
            if let Some(value) = resource.get_attribute_value(attr_name) {
                Ok(GenericConditionValue::Single(value))
            } else if let Some(values) = resource.get_attribute_set(attr_name) {
                Ok(GenericConditionValue::Set(values))
            } else {
                Ok(GenericConditionValue::None)
            }
        },
        crate::types::AttributeExpression::AttributeValue(value) => {
            Ok(GenericConditionValue::Single(value.clone()))
        },
        crate::types::AttributeExpression::ValueSet(values) => {
            Ok(GenericConditionValue::Set(values.clone()))
        },
    }
}

fn get_user_attribute_value<U>(
    user: &U,
    attr_expr: &crate::types::AttributeExpression<U::AttributeName, U::AttributeValue>,
) -> Result<GenericConditionValue<U::AttributeValue>, Box<dyn std::error::Error + Send + Sync>>
where
    U: AttributeValueExtractor,
    U::AttributeValue: Clone,
{
    match attr_expr {
        crate::types::AttributeExpression::AttributeName(attr_name) => {
            if let Some(value) = user.get_attribute_value(attr_name) {
                Ok(GenericConditionValue::Single(value))
            } else if let Some(values) = user.get_attribute_set(attr_name) {
                Ok(GenericConditionValue::Set(values))
            } else {
                Ok(GenericConditionValue::None)
            }
        },
        crate::types::AttributeExpression::AttributeValue(value) => {
            Ok(GenericConditionValue::Single(value.clone()))
        },
        crate::types::AttributeExpression::ValueSet(values) => {
            Ok(GenericConditionValue::Set(values.clone()))
        },
    }
}

fn get_condition_value<N, V>(
    attr_expr: &crate::types::AttributeExpression<N, V>,
) -> Result<GenericConditionValue<V>, Box<dyn std::error::Error + Send + Sync>>
where
    V: Clone,
{
    match attr_expr {
        crate::types::AttributeExpression::AttributeName(_) => {
            Err("Attribute name in condition value not supported in user conditions".into())
        },
        crate::types::AttributeExpression::AttributeValue(value) => {
            Ok(GenericConditionValue::Single(value.clone()))
        },
        crate::types::AttributeExpression::ValueSet(values) => {
            Ok(GenericConditionValue::Set(values.clone()))
        },
    }
}

/// 比較条件でユーザまたはリソースの属性値を取得
fn get_comparison_attribute_value<U, R>(
    user: &U,
    resource: &R,
    attr_expr: &crate::types::AttributeExpression<U::AttributeName, U::AttributeValue>,
) -> Result<GenericConditionValue<U::AttributeValue>, Box<dyn std::error::Error + Send + Sync>>
where
    U: AttributeValueExtractor,
    U::AttributeValue: Clone,
    R: AttributeValueExtractor<AttributeName = U::AttributeName, AttributeValue = U::AttributeValue>,
{
    match attr_expr {
        crate::types::AttributeExpression::AttributeName(attr_name) => {
            if let Some(value) = user.get_attribute_value(attr_name) {
                Ok(GenericConditionValue::Single(value))
            } else if let Some(values) = user.get_attribute_set(attr_name) {
                Ok(GenericConditionValue::Set(values))
            } else if let Some(value) = resource.get_attribute_value(attr_name) {
                Ok(GenericConditionValue::Single(value))
            } else if let Some(values) = resource.get_attribute_set(attr_name) {
                Ok(GenericConditionValue::Set(values))
            } else {
                Ok(GenericConditionValue::None)
            }
        },
        crate::types::AttributeExpression::AttributeValue(value) => {
            Ok(GenericConditionValue::Single(value.clone()))
        },
        crate::types::AttributeExpression::ValueSet(values) => {
            Ok(GenericConditionValue::Set(values.clone()))
        },
    }
}

/// 集合の所属関係を評価する共通関数: 値が集合に含まれるかチェック
fn evaluate_set_membership<T: PartialEq + std::fmt::Debug>(value: &GenericConditionValue<T>, set: &GenericConditionValue<T>) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    match (value, set) {
        (GenericConditionValue::Single(val), GenericConditionValue::Set(set_vals)) => {
            Ok(set_vals.contains(val))
        },
        (GenericConditionValue::None, _) => Ok(false),
        // This is a special case to handle single value containment as equality check
        (GenericConditionValue::Single(left_val), GenericConditionValue::Single(right_val)) => {
            Ok(left_val == right_val)
        },
        _ => Err(format!("Invalid set membership operation: value={:?}, set={:?}", value, set).into()),
    }
}

/// ContainedIn演算子の評価: 左の値が右の集合に含まれる
fn evaluate_contained_in<T: PartialEq + std::fmt::Debug>(left: &GenericConditionValue<T>, right: &GenericConditionValue<T>) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    evaluate_set_membership(left, right)
        .map_err(|e| format!("Invalid ContainedIn operation: {:?} [ {:?} ({})", left, right, e).into())
}

/// Contains演算子の評価: 左の集合が右の値を含む
fn evaluate_contains<T: PartialEq + std::fmt::Debug>(left: &GenericConditionValue<T>, right: &GenericConditionValue<T>) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    evaluate_set_membership(right, left)
        .map_err(|e| format!("Invalid Contains operation: {:?} ] {:?} ({})", left, right, e).into())
}

/// Equals演算子の評価: 左の値と右の値が等しい
fn evaluate_equals<T: PartialEq + std::fmt::Debug>(left: &GenericConditionValue<T>, right: &GenericConditionValue<T>) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    match (left, right) {
        (GenericConditionValue::Single(left_val), GenericConditionValue::Single(right_val)) => {
            Ok(left_val == right_val)
        },
        (GenericConditionValue::None, GenericConditionValue::None) => Ok(true),
        (GenericConditionValue::None, _) | (_, GenericConditionValue::None) => Ok(false),
        _ => Err(format!("Invalid Equals operation: {:?} = {:?}", left, right).into()),
    }
}