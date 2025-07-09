use std::collections::{HashMap, HashSet};
use std::fs;
use regex::Regex;
use crate::types::{
    AbacPolicy, UserAttribute, ResourceAttribute, AttributeValue, ParsedRule,
    AttributeCondition, AttributeComparison, ConditionOperator, ComparisonOperator
};

impl AbacPolicy {
    pub fn parse_from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(filename)?;
        let mut policy = AbacPolicy::new(filename.to_string());

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

            if line.starts_with('#') {
                if line.len() > 2 && !line.contains("----") {
                    current_rule_description = line[1..].trim().to_string();
                }
                continue;
            }

            if let Some(caps) = user_attrib_re.captures(line) {
                let name = caps[1].to_string();
                let attributes = Self::parse_attributes(&caps[2])?;
                policy.users.push(UserAttribute { name, attributes });
            }
            else if let Some(caps) = resource_attrib_re.captures(line) {
                let name = caps[1].to_string();
                let attributes = Self::parse_attributes(&caps[2])?;
                policy.resources.push(ResourceAttribute { name, attributes });
            }
            else if let Some(caps) = rule_re.captures(line) {
                let rule_content = caps[1].to_string();
                let parsed_rule = Self::parse_rule(
                    rule_counter,
                    rule_content.clone(),
                    if current_rule_description.is_empty() {
                        format!("Rule {}", rule_counter)
                    } else {
                        current_rule_description.clone()
                    }
                )?;
                policy.rules.push(parsed_rule);
                rule_counter += 1;
                current_rule_description.clear();
            }
        }

        Ok(policy)
    }

    fn parse_rule(
        id: usize, 
        rule_content: String, 
        description: String
    ) -> Result<ParsedRule, Box<dyn std::error::Error>> {
        let sections = Self::split_rule_sections(&rule_content)?;
        
        if sections.len() != 4 {
            return Err(format!("Rule {} has {} sections, expected 4: {}", id, sections.len(), rule_content).into());
        }

        let user_conditions = Self::parse_conditions(&sections[0])?;
        let resource_conditions = Self::parse_conditions(&sections[1])?;
        let actions = Self::parse_actions(&sections[2])?;
        let comparisons = Self::parse_comparisons(&sections[3])?;

        Ok(ParsedRule {
            id,
            description,
            raw_content: rule_content,
            user_conditions,
            resource_conditions,
            actions,
            comparisons,
        })
    }

    fn split_rule_sections(rule_content: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut sections = Vec::new();
        let mut current_section = String::new();
        let mut brace_depth = 0;
        
        for ch in rule_content.chars() {
            match ch {
                '{' => brace_depth += 1,
                '}' => brace_depth -= 1,
                ';' if brace_depth == 0 => {
                    sections.push(current_section.trim().to_string());
                    current_section.clear();
                    continue;
                },
                _ => {}
            }
            current_section.push(ch);
        }
        
        sections.push(current_section.trim().to_string());
        
        Ok(sections)
    }

    fn parse_conditions(section: &str) -> Result<Vec<AttributeCondition>, Box<dyn std::error::Error>> {
        let mut conditions = Vec::new();
        
        if section.trim().is_empty() {
            return Ok(conditions);
        }

        let condition_re = Regex::new(r"(\w+)\s*\[\s*\{([^}]*)\}")?;
        
        for caps in condition_re.captures_iter(section) {
            let attribute = caps[1].trim().to_string();
            let values_str = caps[2].trim();
            
            let values: HashSet<String> = if values_str.is_empty() {
                HashSet::new()
            } else {
                values_str.split_whitespace()
                    .map(|s| s.to_string())
                    .collect()
            };

            conditions.push(AttributeCondition {
                attribute,
                operator: ConditionOperator::In,
                values,
            });
        }

        Ok(conditions)
    }

    fn parse_actions(section: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let section = section.trim();
        
        if section.is_empty() {
            return Ok(Vec::new());
        }

        if section.starts_with('{') && section.ends_with('}') {
            let actions_str = &section[1..section.len()-1];
            Ok(actions_str.split_whitespace()
                .map(|s| s.to_string())
                .collect())
        } else {
            Err(format!("Invalid actions format: {}", section).into())
        }
    }

    fn parse_comparisons(section: &str) -> Result<Vec<AttributeComparison>, Box<dyn std::error::Error>> {
        let mut comparisons = Vec::new();
        let section = section.trim();
        
        if section.is_empty() || section == ")" {
            return Ok(comparisons);
        }

        if let Some(eq_pos) = section.find('=') {
            let user_part = section[..eq_pos].trim();
            let resource_part = section[eq_pos + 1..].trim();
            
            if user_part == "uid" {
                comparisons.push(AttributeComparison {
                    user_attribute: "uid".to_string(),
                    operator: ComparisonOperator::UserIdEqual,
                    resource_attribute: resource_part.to_string(),
                });
            } else {
                comparisons.push(AttributeComparison {
                    user_attribute: user_part.to_string(),
                    operator: ComparisonOperator::Equal,
                    resource_attribute: resource_part.to_string(),
                });
            }
        }
        else if let Some(contains_pos) = section.find(" ] ") {
            let user_part = section[..contains_pos].trim();
            let resource_part = section[contains_pos + 3..].trim();
            
            comparisons.push(AttributeComparison {
                user_attribute: user_part.to_string(),
                operator: ComparisonOperator::Contains,
                resource_attribute: resource_part.to_string(),
            });
        }
        else if let Some(contained_pos) = section.find(" [ ") {
            let user_part = section[..contained_pos].trim();
            let resource_part = section[contained_pos + 3..].trim();
            
            comparisons.push(AttributeComparison {
                user_attribute: user_part.to_string(),
                operator: ComparisonOperator::ContainedIn,
                resource_attribute: resource_part.to_string(),
            });
        }

        Ok(comparisons)
    }

    fn parse_attributes(attr_str: &str) -> Result<HashMap<String, AttributeValue>, Box<dyn std::error::Error>> {
        let mut attributes = HashMap::new();
        let parts: Vec<&str> = attr_str.split(',').collect();
        
        for part in parts {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }
            
            if let Some(eq_pos) = part.find('=') {
                let key = part[..eq_pos].trim().to_string();
                let value_str = part[eq_pos + 1..].trim();
                
                if value_str.starts_with('{') && value_str.ends_with('}') {
                    let set_content = &value_str[1..value_str.len()-1];
                    let set_values: HashSet<String> = set_content
                        .split_whitespace()
                        .filter(|s| !s.is_empty())
                        .map(|s| s.to_string())
                        .collect();
                    attributes.insert(key, AttributeValue::StringSet(set_values));
                } else if value_str == "True" || value_str == "False" {
                    let bool_val = value_str == "True";
                    attributes.insert(key, AttributeValue::Boolean(bool_val));
                } else {
                    attributes.insert(key, AttributeValue::String(value_str.to_string()));
                }
            }
        }
        
        Ok(attributes)
    }
} 