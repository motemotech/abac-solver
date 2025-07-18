use crate::types::*;
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

// Original detailed evaluation functions for University domain (preserved for backward compatibility)
pub fn detailed_university_analysis(abac_data: UniversityAbac) -> Result<(), Box<dyn std::error::Error>>
{
    let users = abac_data.users;
    let resources = abac_data.resources;
    let rules = abac_data.rules;

    let rule_start_time = std::time::Instant::now();
    for rule in rules {
        // println!("=== Processing Rule {} ===", rule.id);
        let user_conditions = &rule.user_conditions;
        let resource_conditions = &rule.resource_conditions;
        let comparison_conditions = &rule.comparison_conditions;

        let mut validated_users = Vec::new();
        
        // ユーザ条件の処理
        if user_conditions.is_empty() {
            // ユーザ条件がない場合は、全ユーザを対象とする
            validated_users = users.clone();
        } else {
            // 複数の条件は AND 条件として処理
            for user in &users {
                let mut all_conditions_met = true;
                
                for user_condition in user_conditions.iter() {
                    if !evaluate_user_condition(user, user_condition)? {
                        all_conditions_met = false;
                        break;
                    }
                }
                
                if all_conditions_met {
                    validated_users.push(user.clone());
                }
            }
        }

        // println!("Validated users count: {}", validated_users.len());

        let mut validated_resources = Vec::new();
        
        // リソース条件の処理
        if resource_conditions.is_empty() {
            // リソース条件がない場合は、全リソースを対象とする
            validated_resources = resources.clone();
        } else {
            // 複数の条件は AND 条件として処理
            for resource in &resources {
                let mut all_conditions_met = true;
                
                for resource_condition in resource_conditions.iter() {
                    if !evaluate_resource_condition(resource, resource_condition)? {
                        all_conditions_met = false;
                        break;
                    }
                }
                
                if all_conditions_met {
                    validated_resources.push(resource.clone());
                }
            }
        }

        // println!("Validated resources count: {}", validated_resources.len());

        // 比較条件の処理：ユーザとリソースの組み合わせを評価
        let mut valid_combinations = Vec::new();
        
        for user in &validated_users {
            for resource in &validated_resources {
                let mut all_comparison_conditions_met = true;
                
                // 比較条件がない場合は、全ての組み合わせが有効
                if comparison_conditions.is_empty() {
                    valid_combinations.push((user.clone(), resource.clone()));
                    continue;
                }
                
                // 全ての比較条件をチェック
                for comparison_condition in comparison_conditions.iter() {
                    if !evaluate_comparison_condition(user, resource, comparison_condition)? {
                        all_comparison_conditions_met = false;
                        break;
                    }
                }
                
                if all_comparison_conditions_met {
                    valid_combinations.push((user.clone(), resource.clone()));
                }
            }
        }

        // println!("Valid (user, resource) combinations count: {}", valid_combinations.len());
        // for (user, resource) in &valid_combinations {
        //     let actions_str = if rule.actions.is_empty() {
        //         "access".to_string()
        //     } else {
        //         rule.actions.iter()
        //             .map(|action| format!("{:?}", action))
        //             .collect::<Vec<String>>()
        //             .join(", ")
        //     };
        //     println!("  - User: {} can {} Resource: {}", user.get_user_id(), actions_str, resource.get_resource_id());
        // }
        
    }
    let rule_duration = rule_start_time.elapsed();
    println!("Rule processing time: {:.2?}", rule_duration);
    
    Ok(())
}

/// Edocument domain detailed analysis function
pub fn detailed_edocument_analysis(abac_data: EdocumentAbac) -> Result<(), Box<dyn std::error::Error>>
{
    let users = abac_data.users;
    let resources = abac_data.resources;
    let rules = abac_data.rules;

    let mut total_duration = std::time::Instant::now();
    for rule in rules {
        // println!("=== Processing Edocument Rule {} ===", rule.id);
        let user_conditions = &rule.user_conditions;
        let resource_conditions = &rule.resource_conditions;
        let comparison_conditions = &rule.comparison_conditions;

        let mut validated_users = Vec::new();
        
        // ユーザ条件の処理
        if user_conditions.is_empty() {
            // ユーザ条件がない場合は、全ユーザを対象とする
            validated_users = users.clone();
        } else {
            // 複数の条件は AND 条件として処理
            for user in &users {
                let mut all_conditions_met = true;
                
                for user_condition in user_conditions.iter() {
                    if !evaluate_edocument_user_condition_simple(user, user_condition)? {
                        all_conditions_met = false;
                        break;
                    }
                }
                
                if all_conditions_met {
                    validated_users.push(user.clone());
                }
            }
        }

        // println!("Validated users count: {}", validated_users.len());

        let mut validated_resources = Vec::new();
        
        // リソース条件の処理
        if resource_conditions.is_empty() {
            // リソース条件がない場合は、全リソースを対象とする
            validated_resources = resources.clone();
        } else {
            // 複数の条件は AND 条件として処理
            for resource in &resources {
                let mut all_conditions_met = true;
                
                for resource_condition in resource_conditions.iter() {
                    if !evaluate_edocument_resource_condition_simple(resource, resource_condition)? {
                        all_conditions_met = false;
                        break;
                    }
                }
                
                if all_conditions_met {
                    validated_resources.push(resource.clone());
                }
            }
        }

        // println!("Validated resources count: {}", validated_resources.len());

        // 比較条件の処理：ユーザとリソースの組み合わせを評価
        let mut valid_combinations = Vec::new();
        
        for user in &validated_users {
            for resource in &validated_resources {
                let mut all_comparison_conditions_met = true;
                
                // 比較条件がない場合は、全ての組み合わせが有効
                if comparison_conditions.is_empty() {
                    valid_combinations.push((user.clone(), resource.clone()));
                    continue;
                }
                
                // 全ての比較条件をチェック
                for comparison_condition in comparison_conditions.iter() {
                    if !evaluate_edocument_comparison_condition(user, resource, comparison_condition)? {
                        all_comparison_conditions_met = false;
                        break;
                    }
                }
                
                if all_comparison_conditions_met {
                    valid_combinations.push((user.clone(), resource.clone()));
                }
            }
        }

        println!("Valid (user, resource) combinations count: {}", valid_combinations.len());
        // for (user, resource) in &valid_combinations {
        //     let actions_str = if rule.actions.is_empty() {
        //         "access".to_string()
        //     } else {
        //         rule.actions.iter()
        //             .map(|action| format!("{:?}", action))
        //             .collect::<Vec<String>>()
        //             .join(", ")
        //     };
        //     println!("  - User: {} can {} Resource: {}", user.get_user_id(), actions_str, resource.get_resource_id());
        // }
    }
    let total_duration = total_duration.elapsed();
    println!("Total analysis time: {:.2?}", total_duration);
    Ok(())
}

/// Classify edocument condition type based on attributes involved
#[derive(Debug, Clone)]
enum EdocumentConditionType {
    UserCondition,      // Only user attributes
    ResourceCondition,  // Only resource attributes  
    ComparisonCondition, // Both user and resource attributes
    Unknown,           // Cannot determine
}

fn classify_edocument_condition(condition: &EdocumentRule) -> EdocumentConditionType {
    // Since EdocumentRule now has separate condition lists, this classification is not needed
    EdocumentConditionType::Unknown
}

fn is_user_attribute_expression(expr: &EdocAttributeExpression) -> bool {
    matches!(expr, 
        EdocAttributeExpression::Role(_) |
        EdocAttributeExpression::Position(_) |
        EdocAttributeExpression::Registered(_) |
        EdocAttributeExpression::Projects(_) |
        EdocAttributeExpression::Supervisor(_) |
        EdocAttributeExpression::Supervisee(_) |
        EdocAttributeExpression::PayrollingPermissions(_) |
        EdocAttributeExpression::Uid(_) |
        // Shared attributes that can be user attributes
        EdocAttributeExpression::Tenant(_) |
        EdocAttributeExpression::Department(_) |
        EdocAttributeExpression::Office(_)
    )
}

fn is_resource_attribute_expression(expr: &EdocAttributeExpression) -> bool {
    matches!(expr,
        EdocAttributeExpression::Type(_) |
        EdocAttributeExpression::Owner(_) |
        EdocAttributeExpression::Recipients(_) |
        EdocAttributeExpression::IsConfidential(_) |
        EdocAttributeExpression::ContainsPersonalInfo(_) |
        EdocAttributeExpression::Rid(_) |
        // Shared attributes that can be resource attributes
        EdocAttributeExpression::Tenant(_) |
        EdocAttributeExpression::Department(_) |
        EdocAttributeExpression::Office(_)
    )
}

/// Evaluate edocument user condition (only user attributes)
fn evaluate_edocument_user_condition(
    user: &EdocumentUserAttribute,
    condition: &EdocumentRule
) -> Result<bool, Box<dyn std::error::Error>>
{
    // Create a dummy resource for evaluation
    let dummy_resource = EdocumentResourceAttribute {
        resource_id: "dummy".to_string(),
        resource_type: None,
        owner: None,
        tenant: None,
        department: None,
        office: None,
        recipients: std::collections::HashSet::new(),
        is_confidential: None,
        contains_personal_info: None,
    };
    
    evaluate_edocument_condition(user, &dummy_resource, condition)
}

/// Evaluate edocument resource condition (only resource attributes)
fn evaluate_edocument_resource_condition(
    resource: &EdocumentResourceAttribute,
    condition: &EdocumentRule
) -> Result<bool, Box<dyn std::error::Error>>
{
    // Create a dummy user for evaluation
    let dummy_user = EdocumentUserAttribute {
        user_id: "dummy".to_string(),
        role: None,
        position: None,
        tenant: None,
        department: None,
        office: None,
        registered: None,
        projects: std::collections::HashSet::new(),
        supervisor: None,
        supervisee: std::collections::HashSet::new(),
        payrolling_permissions: None,
    };
    
    evaluate_edocument_condition(&dummy_user, resource, condition)
}

/// Evaluate edocument user condition (simplified for single condition)
fn evaluate_edocument_user_condition_simple(
    user: &EdocumentUserAttribute,
    condition: &Condition<EdocAttributeExpression>
) -> Result<bool, Box<dyn std::error::Error>>
{
    let left_value = get_edocument_user_attribute_value(user, &condition.left)?;
    let right_value = get_edocument_condition_value(&condition.right)?;
    
    match condition.operator {
        ComparisonOperator::ContainedIn => evaluate_edocument_contained_in(&left_value, &right_value),
        ComparisonOperator::Contains => evaluate_edocument_contains(&left_value, &right_value),
        ComparisonOperator::Equals => evaluate_edocument_equals(&left_value, &right_value),
    }
}

/// Evaluate edocument resource condition (simplified for single condition)
fn evaluate_edocument_resource_condition_simple(
    resource: &EdocumentResourceAttribute,
    condition: &Condition<EdocAttributeExpression>
) -> Result<bool, Box<dyn std::error::Error>>
{
    let left_value = get_edocument_resource_attribute_value(resource, &condition.left)?;
    let right_value = get_edocument_condition_value(&condition.right)?;
    
    match condition.operator {
        ComparisonOperator::ContainedIn => evaluate_edocument_contained_in(&left_value, &right_value),
        ComparisonOperator::Contains => evaluate_edocument_contains(&left_value, &right_value),
        ComparisonOperator::Equals => evaluate_edocument_equals(&left_value, &right_value),
    }
}

/// Evaluate edocument comparison condition (user and resource attributes)
fn evaluate_edocument_comparison_condition(
    user: &EdocumentUserAttribute,
    resource: &EdocumentResourceAttribute,
    condition: &Condition<EdocAttributeExpression>
) -> Result<bool, Box<dyn std::error::Error>>
{
    let left_value = get_edocument_comparison_attribute_value(user, resource, &condition.left)?;
    let right_value = get_edocument_comparison_attribute_value(user, resource, &condition.right)?;
    
    match condition.operator {
        ComparisonOperator::ContainedIn => evaluate_edocument_contained_in(&left_value, &right_value),
        ComparisonOperator::Contains => evaluate_edocument_contains(&left_value, &right_value),
        ComparisonOperator::Equals => evaluate_edocument_equals(&left_value, &right_value),
    }
}

/// ユーザが指定された条件を満たすかどうかを評価
fn evaluate_user_condition(user: &UniversityUserAttribute, condition: &Condition<UniversityAttributeExpression>) -> Result<bool, Box<dyn std::error::Error>>
{
    let left_value = get_user_attribute_value(user, &condition.left)?;
    let right_value = get_condition_value(&condition.right)?;
    
    match condition.operator {
        ComparisonOperator::ContainedIn => evaluate_contained_in(&left_value, &right_value),
        ComparisonOperator::Contains => evaluate_contains(&left_value, &right_value),
        ComparisonOperator::Equals => evaluate_equals(&left_value, &right_value),
    }
}

/// リソースが指定された条件を満たすかどうかを評価
fn evaluate_resource_condition(resource: &UniversityResourceAttribute, condition: &Condition<UniversityAttributeExpression>) -> Result<bool, Box<dyn std::error::Error>>
{
    let left_value = get_resource_attribute_value(resource, &condition.left)?;
    let right_value = get_condition_value(&condition.right)?;
    
    match condition.operator {
        ComparisonOperator::ContainedIn => evaluate_contained_in(&left_value, &right_value),
        ComparisonOperator::Contains => evaluate_contains(&left_value, &right_value),
        ComparisonOperator::Equals => evaluate_equals(&left_value, &right_value),
    }
}

/// ユーザとリソースの比較条件を評価
fn evaluate_comparison_condition(
    user: &UniversityUserAttribute, 
    resource: &UniversityResourceAttribute, 
    condition: &Condition<UniversityAttributeExpression>
) -> Result<bool, Box<dyn std::error::Error>>
{
    let left_value = get_comparison_attribute_value(user, resource, &condition.left)?;
    let right_value = get_comparison_attribute_value(user, resource, &condition.right)?;
    
    match condition.operator {
        ComparisonOperator::ContainedIn => evaluate_contained_in(&left_value, &right_value),
        ComparisonOperator::Contains => evaluate_contains(&left_value, &right_value),
        ComparisonOperator::Equals => evaluate_equals(&left_value, &right_value),
    }
}

/// リソースから指定された属性の値を取得
fn get_resource_attribute_value(resource: &UniversityResourceAttribute, attr_expr: &UniversityAttributeExpression) -> Result<ConditionValue, Box<dyn std::error::Error>>
{
    match attr_expr {
        UniversityAttributeExpression::AttributeName(attr_name) => {
            // 単一値の属性を試す
            if let Some(value) = resource.get_attribute_value(attr_name) {
                Ok(ConditionValue::Single(value))
            }
            // セット値の属性を試す
            else if let Some(values) = resource.get_attribute_set(attr_name) {
                Ok(ConditionValue::Set(values))
            }
            // 該当する属性がない場合
            else {
                Ok(ConditionValue::None)
            }
        },
        UniversityAttributeExpression::AttributeValue(value) => {
            Ok(ConditionValue::Single(value.clone()))
        },
        UniversityAttributeExpression::ValueSet(values) => {
            Ok(ConditionValue::Set(values.clone()))
        },
    }
}

/// ユーザから指定された属性の値を取得
fn get_user_attribute_value(user: &UniversityUserAttribute, attr_expr: &UniversityAttributeExpression) -> Result<ConditionValue, Box<dyn std::error::Error>>
{
    match attr_expr {
        UniversityAttributeExpression::AttributeName(attr_name) => {
            // 単一値の属性を試す
            if let Some(value) = user.get_attribute_value(attr_name) {
                Ok(ConditionValue::Single(value))
            }
            // セット値の属性を試す
            else if let Some(values) = user.get_attribute_set(attr_name) {
                Ok(ConditionValue::Set(values))
            }
            // 該当する属性がない場合
            else {
                Ok(ConditionValue::None)
            }
        },
        UniversityAttributeExpression::AttributeValue(value) => {
            Ok(ConditionValue::Single(value.clone()))
        },
        UniversityAttributeExpression::ValueSet(values) => {
            Ok(ConditionValue::Set(values.clone()))
        },
    }
}

/// 条件式の右辺値を取得
fn get_condition_value(attr_expr: &UniversityAttributeExpression) -> Result<ConditionValue, Box<dyn std::error::Error>> {
    match attr_expr {
        UniversityAttributeExpression::AttributeName(_) => {
            // 条件の右辺で属性名が来ることは通常ないが、比較条件では可能
            Err("Attribute name in condition value not supported in user conditions".into())
        },
        UniversityAttributeExpression::AttributeValue(value) => {
            Ok(ConditionValue::Single(value.clone()))
        },
        UniversityAttributeExpression::ValueSet(values) => {
            Ok(ConditionValue::Set(values.clone()))
        },
    }
}

/// 比較条件でユーザまたはリソースの属性値を取得
fn get_comparison_attribute_value(
    user: &UniversityUserAttribute,
    resource: &UniversityResourceAttribute,
    attr_expr: &UniversityAttributeExpression
) -> Result<ConditionValue, Box<dyn std::error::Error>>
{
    match attr_expr {
        UniversityAttributeExpression::AttributeName(attr_name) => {
            // まずユーザ属性を試す
            if let Some(value) = user.get_attribute_value(attr_name) {
                Ok(ConditionValue::Single(value))
            } else if let Some(values) = user.get_attribute_set(attr_name) {
                Ok(ConditionValue::Set(values))
            }
            // 次にリソース属性を試す
            else if let Some(value) = resource.get_attribute_value(attr_name) {
                Ok(ConditionValue::Single(value))
            } else if let Some(values) = resource.get_attribute_set(attr_name) {
                Ok(ConditionValue::Set(values))
            }
            // どちらにも該当しない場合
            else {
                Ok(ConditionValue::None)
            }
        },
        UniversityAttributeExpression::AttributeValue(value) => {
            Ok(ConditionValue::Single(value.clone()))
        },
        UniversityAttributeExpression::ValueSet(values) => {
            Ok(ConditionValue::Set(values.clone()))
        },
    }
}

/// 条件値を表現する内部型
#[derive(Debug, Clone)]
enum ConditionValue {
    Single(UniversityAttributeValue),
    Set(Vec<UniversityAttributeValue>),
    None,
}

/// 集合の所属関係を評価する共通関数: 値が集合に含まれるかチェック
fn evaluate_set_membership(value: &ConditionValue, set: &ConditionValue) -> Result<bool, Box<dyn std::error::Error>> {
    match (value, set) {
        (ConditionValue::Single(val), ConditionValue::Set(set_vals)) => {
            Ok(set_vals.contains(val))
        },
        (ConditionValue::None, _) => Ok(false),
        _ => Err(format!("Invalid set membership operation: value={:?}, set={:?}", value, set).into()),
    }
}

/// ContainedIn演算子の評価: 左の値が右の集合に含まれる
fn evaluate_contained_in(left: &ConditionValue, right: &ConditionValue) -> Result<bool, Box<dyn std::error::Error>> {
    evaluate_set_membership(left, right)
        .map_err(|_| format!("Invalid ContainedIn operation: {:?} [ {:?}", left, right).into())
}

/// Contains演算子の評価: 左の集合が右の値を含む
fn evaluate_contains(left: &ConditionValue, right: &ConditionValue) -> Result<bool, Box<dyn std::error::Error>> {
    evaluate_set_membership(right, left)
        .map_err(|_| format!("Invalid Contains operation: {:?} ] {:?}", left, right).into())
}

/// Equals演算子の評価: 左の値と右の値が等しい
fn evaluate_equals(left: &ConditionValue, right: &ConditionValue) -> Result<bool, Box<dyn std::error::Error>> {
    match (left, right) {
        (ConditionValue::Single(left_val), ConditionValue::Single(right_val)) => {
            Ok(left_val == right_val)
        },
        (ConditionValue::None, ConditionValue::None) => Ok(true),
        (ConditionValue::None, _) | (_, ConditionValue::None) => Ok(false),
        _ => Err(format!("Invalid Equals operation: {:?} = {:?}", left, right).into()),
    }
}

/// Evaluate an edocument condition for a given user-resource pair
fn evaluate_edocument_condition(
    user: &EdocumentUserAttribute,
    resource: &EdocumentResourceAttribute,
    condition: &EdocumentRule
) -> Result<bool, Box<dyn std::error::Error>>
{
    // This function is no longer needed with the new structure
    // Each rule now has separate user_conditions, resource_conditions, and comparison_conditions
    Ok(true)
}

/// Get attribute value from either user or resource for edocument domain
fn get_edocument_attribute_value(
    user: &EdocumentUserAttribute,
    resource: &EdocumentResourceAttribute,
    attr_expr: &EdocAttributeExpression
) -> Result<EdocumentConditionValue, Box<dyn std::error::Error>>
{
    // Use the comparison function instead
    get_edocument_comparison_attribute_value(user, resource, attr_expr)
}

/// Condition value for edocument domain
#[derive(Debug, Clone)]
enum EdocumentConditionValue {
    Single(EdocAttributeValue),
    Set(Vec<EdocAttributeValue>),
    None,
}

/// ContainedIn evaluation for edocument domain
fn evaluate_edocument_contained_in(left: &EdocumentConditionValue, right: &EdocumentConditionValue) -> Result<bool, Box<dyn std::error::Error>> {
    match (left, right) {
        (EdocumentConditionValue::Single(val), EdocumentConditionValue::Set(set_vals)) => {
            Ok(set_vals.contains(val))
        },
        // Handle single value containment as equality check
        (EdocumentConditionValue::Single(left_val), EdocumentConditionValue::Single(right_val)) => {
            Ok(left_val == right_val)
        },
        (EdocumentConditionValue::None, _) => Ok(false),
        _ => Err(format!("Invalid ContainedIn operation: {:?} [ {:?}", left, right).into()),
    }
}

/// Contains evaluation for edocument domain
fn evaluate_edocument_contains(left: &EdocumentConditionValue, right: &EdocumentConditionValue) -> Result<bool, Box<dyn std::error::Error>> {
    match (left, right) {
        (EdocumentConditionValue::Set(set_vals), EdocumentConditionValue::Single(val)) => {
            Ok(set_vals.contains(val))
        },
        // Handle single value contains as equality check
        (EdocumentConditionValue::Single(left_val), EdocumentConditionValue::Single(right_val)) => {
            Ok(left_val == right_val)
        },
        (EdocumentConditionValue::None, _) => Ok(false),
        _ => Err(format!("Invalid Contains operation: {:?} ] {:?}", left, right).into()),
    }
}

/// Equals evaluation for edocument domain
fn evaluate_edocument_equals(left: &EdocumentConditionValue, right: &EdocumentConditionValue) -> Result<bool, Box<dyn std::error::Error>> {
    match (left, right) {
        (EdocumentConditionValue::Single(left_val), EdocumentConditionValue::Single(right_val)) => {
            Ok(left_val == right_val)
        },
        (EdocumentConditionValue::None, EdocumentConditionValue::None) => Ok(true),
        (EdocumentConditionValue::None, _) | (_, EdocumentConditionValue::None) => Ok(false),
        _ => Err(format!("Invalid Equals operation: {:?} = {:?}", left, right).into()),
    }
}

/// Get user attribute value for edocument domain
fn get_edocument_user_attribute_value(
    user: &EdocumentUserAttribute,
    attr_expr: &EdocAttributeExpression
) -> Result<EdocumentConditionValue, Box<dyn std::error::Error>>
{
    match attr_expr {
        EdocAttributeExpression::Role(s) => {
            // 属性名の場合は実際のユーザーの属性値を取得
            if s == "__ATTRIBUTE_NAME__" {
                if let Some(value) = user.get_attribute_value(&EdocAttributeName::Role) {
                    Ok(EdocumentConditionValue::Single(value))
                } else {
                    Ok(EdocumentConditionValue::None)
                }
            } else {
                // 属性値の場合はその値を返す
                use crate::edocument_types::{Role, AttributeValue as EdocAttributeValue};
                let role = match s.as_str() {
                    "employee" => Role::Employee,
                    "helpdesk" => Role::Helpdesk,
                    "admin" => Role::Admin,
                    "customer" => Role::Customer,
                    _ => return Ok(EdocumentConditionValue::None),
                };
                Ok(EdocumentConditionValue::Single(EdocAttributeValue::Role(role)))
            }
        },
        EdocAttributeExpression::Position(_s) => {
            if let Some(value) = user.get_attribute_value(&EdocAttributeName::Position) {
                Ok(EdocumentConditionValue::Single(value))
            } else {
                Ok(EdocumentConditionValue::None)
            }
        },
        EdocAttributeExpression::Registered(b) => {
            // 属性名の場合は実際のユーザーの属性値を取得
            if *b == false { // placeholder for attribute name  
                if let Some(value) = user.get_attribute_value(&EdocAttributeName::Registered) {
                    Ok(EdocumentConditionValue::Single(value))
                } else {
                    Ok(EdocumentConditionValue::None)
                }
            } else {
                // 属性値の場合はその値を返す
                use crate::edocument_types::AttributeValue as EdocAttributeValue;
                Ok(EdocumentConditionValue::Single(EdocAttributeValue::Boolean(*b)))
            }
        },
        EdocAttributeExpression::Uid(_s) => {
            if let Some(value) = user.get_attribute_value(&EdocAttributeName::Uid) {
                Ok(EdocumentConditionValue::Single(value))
            } else {
                Ok(EdocumentConditionValue::None)
            }
        },
        EdocAttributeExpression::Projects(_vec) => {
            if let Some(values) = user.get_attribute_set(&EdocAttributeName::Projects) {
                Ok(EdocumentConditionValue::Set(values))
            } else {
                Ok(EdocumentConditionValue::None)
            }
        },
        EdocAttributeExpression::Supervisee(_vec) => {
            if let Some(values) = user.get_attribute_set(&EdocAttributeName::Supervisee) {
                Ok(EdocumentConditionValue::Set(values))
            } else {
                Ok(EdocumentConditionValue::None)
            }
        },
        EdocAttributeExpression::Tenant(_s) => {
            if let Some(value) = user.get_attribute_value(&EdocAttributeName::Tenant) {
                Ok(EdocumentConditionValue::Single(value))
            } else {
                Ok(EdocumentConditionValue::None)
            }
        },
        EdocAttributeExpression::Department(_s) => {
            if let Some(value) = user.get_attribute_value(&EdocAttributeName::Department) {
                Ok(EdocumentConditionValue::Single(value))
            } else {
                Ok(EdocumentConditionValue::None)
            }
        },
        EdocAttributeExpression::Office(_s) => {
            if let Some(value) = user.get_attribute_value(&EdocAttributeName::Office) {
                Ok(EdocumentConditionValue::Single(value))
            } else {
                Ok(EdocumentConditionValue::None)
            }
        },
        EdocAttributeExpression::Supervisor(_s) => {
            if let Some(value) = user.get_attribute_value(&EdocAttributeName::Supervisor) {
                Ok(EdocumentConditionValue::Single(value))
            } else {
                Ok(EdocumentConditionValue::None)
            }
        },
        EdocAttributeExpression::PayrollingPermissions(_b) => {
            if let Some(value) = user.get_attribute_value(&EdocAttributeName::PayrollingPermissions) {
                Ok(EdocumentConditionValue::Single(value))
            } else {
                Ok(EdocumentConditionValue::None)
            }
        },
        _ => Ok(EdocumentConditionValue::None),
    }
}

/// Get resource attribute value for edocument domain
fn get_edocument_resource_attribute_value(
    resource: &EdocumentResourceAttribute,
    attr_expr: &EdocAttributeExpression
) -> Result<EdocumentConditionValue, Box<dyn std::error::Error>>
{
    match attr_expr {
        EdocAttributeExpression::Type(s) => {
            if let Some(value) = resource.get_attribute_value(&EdocAttributeName::Type) {
                Ok(EdocumentConditionValue::Single(value))
            } else {
                Ok(EdocumentConditionValue::None)
            }
        },
        EdocAttributeExpression::Owner(s) => {
            if let Some(value) = resource.get_attribute_value(&EdocAttributeName::Owner) {
                Ok(EdocumentConditionValue::Single(value))
            } else {
                Ok(EdocumentConditionValue::None)
            }
        },
        EdocAttributeExpression::IsConfidential(b) => {
            if let Some(value) = resource.get_attribute_value(&EdocAttributeName::IsConfidential) {
                Ok(EdocumentConditionValue::Single(value))
            } else {
                Ok(EdocumentConditionValue::None)
            }
        },
        EdocAttributeExpression::Recipients(vec) => {
            if let Some(values) = resource.get_attribute_set(&EdocAttributeName::Recipients) {
                Ok(EdocumentConditionValue::Set(values))
            } else {
                Ok(EdocumentConditionValue::None)
            }
        },
        _ => Ok(EdocumentConditionValue::None),
    }
}

/// Get condition value from attribute expression  
fn get_edocument_condition_value(attr_expr: &EdocAttributeExpression) -> Result<EdocumentConditionValue, Box<dyn std::error::Error>> {
    match attr_expr {
        EdocAttributeExpression::Role(s) => {
            use crate::edocument_types::{Role, AttributeValue as EdocAttributeValue};
            let role = match s.as_str() {
                "employee" => Role::Employee,
                "helpdesk" => Role::Helpdesk,
                "admin" => Role::Admin,
                "customer" => Role::Customer,
                _ => return Ok(EdocumentConditionValue::None),
            };
            Ok(EdocumentConditionValue::Single(EdocAttributeValue::Role(role)))
        },
        EdocAttributeExpression::Registered(b) => {
            use crate::edocument_types::AttributeValue as EdocAttributeValue;
            Ok(EdocumentConditionValue::Single(EdocAttributeValue::Boolean(*b)))
        },
        EdocAttributeExpression::Projects(vec) => {
            use crate::edocument_types::AttributeValue as EdocAttributeValue;
            let values: Vec<EdocAttributeValue> = vec.iter().map(|s| EdocAttributeValue::String(s.clone())).collect();
            Ok(EdocumentConditionValue::Set(values))
        },
        EdocAttributeExpression::Type(s) => {
            use crate::edocument_types::{ResourceType, AttributeValue as EdocAttributeValue};
            let resource_type = match s.as_str() {
                "bankingNote" => ResourceType::BankingNote,
                "trafficFine" => ResourceType::TrafficFine,
                "salesOffer" => ResourceType::SalesOffer,
                "contract" => ResourceType::Contract,
                "invoice" => ResourceType::Invoice,
                "paycheck" => ResourceType::Paycheck,
                _ => return Ok(EdocumentConditionValue::None),
            };
            Ok(EdocumentConditionValue::Single(EdocAttributeValue::ResourceType(resource_type)))
        },
        _ => {
            use crate::edocument_types::AttributeValue as EdocAttributeValue;
            Ok(EdocumentConditionValue::Single(EdocAttributeValue::String("default".to_string())))
        },
    }
}

/// Get comparison attribute value from user or resource
fn get_edocument_comparison_attribute_value(
    user: &EdocumentUserAttribute,
    resource: &EdocumentResourceAttribute,
    attr_expr: &EdocAttributeExpression
) -> Result<EdocumentConditionValue, Box<dyn std::error::Error>>
{
    // Try user attributes first
    match get_edocument_user_attribute_value(user, attr_expr) {
        Ok(EdocumentConditionValue::None) => {
            // If not found in user, try resource
            get_edocument_resource_attribute_value(resource, attr_expr)
        },
        other => other,
    }
}