use serde::{Serialize, Deserialize};
use std::str::FromStr;

// 比較演算子を表現（一般的）
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComparisonOperator {
    Contains,     // ] : 左の属性セットが右の属性値を含む
    ContainedIn,  // [ : 左の属性値が右の属性セットに含まれる
    Equals,       // = : 等価関係
}

// 汎用的な属性表現
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttributeExpression<N, V> {
    AttributeName(N),
    AttributeValue(V),
    ValueSet(Vec<V>),
}

// 属性値を動的に取得するためのtrait（一般的）
pub trait AttributeValueExtractor {
    type AttributeName;
    type AttributeValue;
    
    fn get_attribute_value(&self, attr_name: &Self::AttributeName) -> Option<Self::AttributeValue>;
    fn get_attribute_set(&self, attr_name: &Self::AttributeName) -> Option<Vec<Self::AttributeValue>>;
}

// 汎用的なユーザ属性trait
pub trait UserAttribute: AttributeValueExtractor + Clone {
    fn get_user_id(&self) -> String;
}

// 汎用的なリソース属性trait
pub trait ResourceAttribute: AttributeValueExtractor + Clone {
    fn get_resource_id(&self) -> String;
}

// 汎用的な条件式を表現
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Condition<T> {
    pub left: T,
    pub operator: ComparisonOperator,
    pub right: T,
}

// 汎用的なABACデータ構造
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbacData<U: UserAttribute, R: ResourceAttribute, Rule> {
    pub users: Vec<U>,
    pub resources: Vec<R>,
    pub rules: Vec<Rule>,
}

// パースエラー（一般的）
#[derive(Debug, Clone)]
pub enum ParseError {
    InvalidLine(String),
    UnknownValue(String, String), // value_type, value
    MissingAttribute(String),
    InvalidFormat(String),
    InvalidCondition(String),
    FileError(String),
    ParseErrorAtLine(usize, String, String), // line_number, line_content, error_message
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidLine(line) => write!(f, "Invalid line: {}", line),
            ParseError::UnknownValue(value_type, value) => write!(f, "Unknown {}: {}", value_type, value),
            ParseError::MissingAttribute(attr) => write!(f, "Missing attribute: {}", attr),
            ParseError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            ParseError::InvalidCondition(cond) => write!(f, "Invalid condition: {}", cond),
            ParseError::FileError(msg) => write!(f, "File error: {}", msg),
            ParseError::ParseErrorAtLine(line_num, line_content, error_msg) => {
                write!(f, "Parse error at line {}: {}\nLine content: '{}'", line_num, error_msg, line_content)
            },
        }
    }
}

impl std::error::Error for ParseError {}

// ドメイン固有のパース処理を抽象化するトレイト
pub trait DomainParser {
    type UserAttribute: UserAttribute;
    type ResourceAttribute: ResourceAttribute;
    type Rule;
    type AttributeExpression: Clone + PartialEq + Eq + std::fmt::Debug;
    type AttributeName: Clone + PartialEq + Eq + std::hash::Hash;
    type AttributeValue: Clone + PartialEq + Eq + std::fmt::Debug;
    type Action: Clone + PartialEq + Eq + std::hash::Hash;

    // ドメイン固有の値をパースする
    fn parse_attribute_name(&self, s: &str) -> Result<Self::AttributeName, ParseError>;
    fn parse_attribute_value(&self, s: &str) -> Result<Self::AttributeValue, ParseError>;
    fn parse_action(&self, s: &str) -> Result<Self::Action, ParseError>;
    
    // 属性表現をパースする
    fn parse_attribute_expression(&self, expr_str: &str) -> Result<Self::AttributeExpression, ParseError>;
    
    // ユーザー属性をパースする（ドメイン固有の属性設定）
    fn parse_user_attribute_field(
        &self, 
        user: &mut Self::UserAttribute, 
        key: &str, 
        value: &str
    ) -> Result<(), ParseError>;
    
    // リソース属性をパースする（ドメイン固有の属性設定）
    fn parse_resource_attribute_field(
        &self, 
        resource: &mut Self::ResourceAttribute, 
        key: &str, 
        value: &str
    ) -> Result<(), ParseError>;
    
    // 新しいユーザー属性を作成
    fn create_user_attribute(&self, user_id: String) -> Self::UserAttribute;
    
    // 新しいリソース属性を作成
    fn create_resource_attribute(&self, resource_id: String, resource_type: &str) -> Result<Self::ResourceAttribute, ParseError>;
    
    // 新しいルールを作成
    fn create_rule(
        &self,
        id: usize,
        user_conditions: Vec<Condition<Self::AttributeExpression>>,
        resource_conditions: Vec<Condition<Self::AttributeExpression>>,
        actions: std::collections::HashSet<Self::Action>,
        comparison_conditions: Vec<Condition<Self::AttributeExpression>>,
    ) -> Self::Rule;
}

// 汎用的なABACパーサー
pub struct GenericAbacParser<D: DomainParser> {
    domain_parser: D,
}

impl<D: DomainParser> GenericAbacParser<D> {
    pub fn new(domain_parser: D) -> Self {
        Self { domain_parser }
    }

    /// ファイルパスからファイルを読み取ってパースします
    pub fn parse_file(&self, file_path: &str) -> Result<AbacData<D::UserAttribute, D::ResourceAttribute, D::Rule>, ParseError> {
        let content = std::fs::read_to_string(file_path)
            .map_err(|e| ParseError::FileError(format!("Failed to read file '{}': {}", file_path, e)))?;
        self.parse(&content)
    }

    /// 文字列コンテンツをパースします
    pub fn parse(&self, content: &str) -> Result<AbacData<D::UserAttribute, D::ResourceAttribute, D::Rule>, ParseError> {
        let mut users = Vec::new();
        let mut resources = Vec::new();
        let mut rules = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();
            
            // コメントや空行をスキップ
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if line.starts_with("userAttrib(") {
                users.push(self.parse_user_attribute(line_num, line)?);
            } else if line.starts_with("resourceAttrib(") {
                resources.push(self.parse_resource_attribute(line_num, line)?);
            } else if line.starts_with("rule(") {
                rules.push(self.parse_rule(line_num, line, rules.len())?);
            }
        }

        Ok(AbacData {
            users,
            resources,
            rules,
        })
    }

    fn parse_user_attribute(&self, line_num: usize, line: &str) -> Result<D::UserAttribute, ParseError> {
        let content = self.extract_parentheses_content(line)
            .map_err(|e| ParseError::ParseErrorAtLine(
                line_num + 1,
                line.to_string(),
                format!("Error in user attribute: {}", e)
            ))?;
        let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();
        
        if parts.is_empty() {
            return Err(ParseError::ParseErrorAtLine(
                line_num + 1,
                line.to_string(),
                "Empty user attribute".to_string()
            ));
        }

        let user_id = parts[0].to_string();
        let mut user_attr = self.domain_parser.create_user_attribute(user_id);

        for part in &parts[1..] {
            if let Some((key, value)) = part.split_once('=') {
                let key = key.trim();
                let value = value.trim();
                self.domain_parser.parse_user_attribute_field(&mut user_attr, key, value)
                    .map_err(|e| ParseError::ParseErrorAtLine(
                        line_num + 1,
                        line.to_string(),
                        format!("Error parsing user attribute field '{}': {}", key, e)
                    ))?;
            }
        }

        Ok(user_attr)
    }

    fn parse_resource_attribute(&self, line_num: usize, line: &str) -> Result<D::ResourceAttribute, ParseError> {
        let content = self.extract_parentheses_content(line)
            .map_err(|e| ParseError::ParseErrorAtLine(
                line_num + 1,
                line.to_string(),
                format!("Error in resource attribute: {}", e)
            ))?;
        let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();
        
        if parts.len() < 2 {
            return Err(ParseError::ParseErrorAtLine(
                line_num + 1,
                line.to_string(),
                "Resource attribute needs at least type".to_string()
            ));
        }

        let resource_id = parts[0].to_string();
        let mut resource_type = None;

        // typeを先に見つける
        for part in &parts[1..] {
            if let Some((key, value)) = part.split_once('=') {
                if key.trim() == "type" {
                    resource_type = Some(value.trim());
                    break;
                }
            }
        }

        let resource_type = resource_type.ok_or_else(|| 
            ParseError::ParseErrorAtLine(
                line_num + 1,
                line.to_string(),
                "Missing required attribute: type".to_string()
            ))?;

        let mut resource_attr = self.domain_parser.create_resource_attribute(resource_id, resource_type)
            .map_err(|e| ParseError::ParseErrorAtLine(
                line_num + 1,
                line.to_string(),
                format!("Error creating resource attribute: {}", e)
            ))?;

        for part in &parts[1..] {
            if let Some((key, value)) = part.split_once('=') {
                let key = key.trim();
                let value = value.trim();
                if key != "type" { // typeは既に処理済み
                    self.domain_parser.parse_resource_attribute_field(&mut resource_attr, key, value)
                        .map_err(|e| ParseError::ParseErrorAtLine(
                            line_num + 1,
                            line.to_string(),
                            format!("Error parsing resource attribute field '{}': {}", key, e)
                        ))?;
                }
            }
        }

        Ok(resource_attr)
    }

    fn parse_rule(&self, line_num: usize, line: &str, id: usize) -> Result<D::Rule, ParseError> {
        let content = self.extract_parentheses_content(line)?;
        let sections: Vec<&str> = content.split(';').collect();
        
        if sections.len() < 3 || sections.len() > 4 {
            return Err(ParseError::ParseErrorAtLine(
                line_num + 1,
                line.to_string(),
                "Rule must have 3 or 4 sections separated by semicolons".to_string()
            ));
        }

        // セクション1: ユーザー条件
        let user_conditions = if sections[0].trim().is_empty() {
            Vec::new()
        } else {
            self.parse_conditions_section(sections[0].trim())
                .map_err(|e| ParseError::ParseErrorAtLine(
                    line_num + 1,
                    line.to_string(),
                    format!("Error parsing user conditions: {}", e)
                ))?
        };

        // セクション2: リソース条件
        let resource_conditions = if sections[1].trim().is_empty() {
            Vec::new()
        } else {
            self.parse_conditions_section(sections[1].trim())
                .map_err(|e| ParseError::ParseErrorAtLine(
                    line_num + 1,
                    line.to_string(),
                    format!("Error parsing resource conditions: {}", e)
                ))?
        };

        // セクション3: アクション
        let actions = self.parse_actions_section(sections[2].trim())
            .map_err(|e| ParseError::ParseErrorAtLine(
                line_num + 1,
                line.to_string(),
                format!("Error parsing actions: {}", e)
            ))?;

        // セクション4: 比較条件（存在する場合）
        let comparison_conditions = if sections.len() == 4 && !sections[3].trim().is_empty() {
            self.parse_conditions_section(sections[3].trim())
                .map_err(|e| ParseError::ParseErrorAtLine(
                    line_num + 1,
                    line.to_string(),
                    format!("Error parsing comparison conditions: {}", e)
                ))?
        } else {
            Vec::new()
        };

        Ok(self.domain_parser.create_rule(id, user_conditions, resource_conditions, actions, comparison_conditions))
    }

    fn parse_conditions_section(&self, section: &str) -> Result<Vec<Condition<D::AttributeExpression>>, ParseError> {
        let mut conditions = Vec::new();
        
        for condition_str in section.split(',') {
            let condition_str = condition_str.trim();
            if !condition_str.is_empty() {
                conditions.push(self.parse_single_condition(condition_str)?);
            }
        }

        Ok(conditions)
    }

    fn parse_single_condition(&self, condition_str: &str) -> Result<Condition<D::AttributeExpression>, ParseError> {
        let operators = vec![" [ ", " ] ", " = ", "[", "]", "="];
        let mut found_operator = None;
        let mut split_pos = None;
        let mut operator_len = 0;

        for op in &operators {
            if let Some(pos) = condition_str.find(op) {
                found_operator = Some(op.trim());
                split_pos = Some(pos);
                operator_len = op.len();
                break;
            }
        }

        let (operator, pos) = match (found_operator, split_pos) {
            (Some(op), Some(pos)) => (op, pos),
            _ => return Err(ParseError::InvalidCondition(format!("No operator found: {}", condition_str))),
        };

        let left_str = condition_str[..pos].trim();
        let right_str = condition_str[pos + operator_len..].trim();

        let left = self.domain_parser.parse_attribute_expression(left_str)?;
        let right = self.domain_parser.parse_attribute_expression(right_str)?;
        let op = ComparisonOperator::from_str(operator)
            .map_err(|_| ParseError::UnknownValue("operator".to_string(), operator.to_string()))?;

        Ok(Condition {
            left,
            operator: op,
            right,
        })
    }

    fn parse_actions_section(&self, section: &str) -> Result<std::collections::HashSet<D::Action>, ParseError> {
        let mut actions = std::collections::HashSet::new();
        
        let content = if section.starts_with('{') && section.ends_with('}') {
            &section[1..section.len()-1]
        } else {
            section
        };

        for action_str in content.split_whitespace() {
            let action = self.domain_parser.parse_action(action_str)?;
            actions.insert(action);
        }

        Ok(actions)
    }

    fn extract_parentheses_content(&self, line: &str) -> Result<String, ParseError> {
        let start = line.find('(').ok_or_else(|| 
            ParseError::InvalidFormat("Missing opening parenthesis".to_string()))?;
        let end = line.rfind(')').ok_or_else(|| 
            ParseError::InvalidFormat("Missing closing parenthesis".to_string()))?;
        
        if start >= end {
            return Err(ParseError::InvalidFormat("Invalid parentheses".to_string()));
        }

        Ok(line[start + 1..end].to_string())
    }
}

// 比較演算子の文字列からの変換
impl std::str::FromStr for ComparisonOperator {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "]" => Ok(ComparisonOperator::Contains),
            "[" => Ok(ComparisonOperator::ContainedIn),
            "=" => Ok(ComparisonOperator::Equals),
            _ => Err(format!("Unknown comparison operator: {}", s)),
        }
    }
}
