use std::collections::{HashMap, HashSet};
use clap::Parser;

#[derive(Parser)]
#[command(name = "abac-solver")]
#[command(about = "ABAC Policy Analyzer with Z3")]
pub struct Args {
    /// Path to the ABAC file to analyze
    #[arg(short, long, default_value = "data/university.abac")]
    pub file: String,
    
    /// Number of users to check for resource access
    #[arg(short = 'n', long, default_value = "10")]
    pub max_users: usize,
}

#[derive(Debug, Clone)]
pub struct UserAttribute {
    pub name: String,
    pub attributes: HashMap<String, AttributeValue>,
}

#[derive(Debug, Clone)]
pub struct ResourceAttribute {
    pub name: String,
    pub attributes: HashMap<String, AttributeValue>,
}

#[derive(Debug, Clone)]
pub enum AttributeValue {
    String(String),
    StringSet(HashSet<String>),
    Boolean(bool),
}

#[derive(Debug, Clone)]
pub struct ParsedRule {
    pub id: usize,
    pub description: String,
    pub raw_content: String,
    pub user_conditions: Vec<AttributeCondition>,
    pub resource_conditions: Vec<AttributeCondition>,
    pub actions: Vec<String>,
    pub comparisons: Vec<AttributeComparison>,
}

#[derive(Debug, Clone)]
pub struct AttributeCondition {
    pub attribute: String,
    pub operator: ConditionOperator,
    pub values: HashSet<String>,
}

#[derive(Debug, Clone)]
pub enum ConditionOperator {
    In,
}

#[derive(Debug, Clone)]
pub struct AttributeComparison {
    pub user_attribute: String,
    pub operator: ComparisonOperator,
    pub resource_attribute: String,
}

#[derive(Debug, Clone)]
pub enum ComparisonOperator {
    Contains,
    ContainedIn,
    Equal,
    UserIdEqual,
}

#[derive(Debug)]
pub struct AbacPolicy {
    pub users: Vec<UserAttribute>,
    pub resources: Vec<ResourceAttribute>,
    pub rules: Vec<ParsedRule>,
    pub filename: String,
}

impl AbacPolicy {
    pub fn new(filename: String) -> Self {
        AbacPolicy {
            users: Vec::new(),
            resources: Vec::new(),
            rules: Vec::new(),
            filename,
        }
    }
}