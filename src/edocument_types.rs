use std::collections::HashSet;
use std::str::FromStr;
use serde::{Serialize, Deserialize};
// 再エクスポートして他のモジュールから使えるようにする
pub use crate::types::{ComparisonOperator, AttributeValueExtractor, UserAttribute, ResourceAttribute, Condition, AbacData, ParseError, DomainParser, GenericAbacParser};

// ユーザーの役職を表現
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Role {
    Employee,
    Helpdesk,
    Admin,
    Customer,
}

// ポジションを表現
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Position {
    Secretary,
    InsuranceAgent,
    OfficeManager,
    SeniorOfficeManager,
    Director,
    None,
}

// テナントを表現
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Tenant {
    LondonOffice,
    LargeBank,
    NewsAgency,
    Reseller,
    EuropeRegion,
    LargeBankLeasing,
    CarLeaser,
    IctProvider,
    PrivateReceiver,
}

// リソースタイプを表現
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    BankingNote,
    TrafficFine,
    SalesOffer,
    Contract,
    Invoice,
    Paycheck,
}

// アクション（権限）を表現
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {
    View,
    Send,
    Search,
    ReadMetaInfo,
    Edit,
    Approve,
}

// 属性名の型
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttributeName {
    Role,
    Position,
    Tenant,
    Department,
    Office,
    Registered,
    Projects,
    Supervisor,
    Supervisee,
    PayrollingPermissions,
    Type,
    Owner,
    Recipients,
    IsConfidential,
    ContainsPersonalInfo,
    Uid,
    Rid,
}

// 属性値の型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttributeValue {
    Role(Role),
    Position(Position),
    Tenant(Tenant),
    ResourceType(ResourceType),
    String(String),
    Boolean(bool),
    StringSet(Vec<String>),
}

// 属性表現のenum（Hashを含まないバージョン）
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttributeExpression {
    // User attributes
    Role(String),
    Position(String),
    Tenant(String),
    Department(String),
    Office(String),
    Registered(bool),
    Projects(Vec<String>),
    Supervisor(Option<String>),
    Supervisee(Vec<String>),
    PayrollingPermissions(bool),
    
    // Resource attributes
    Type(String),
    Owner(String),
    Recipients(Vec<String>),
    IsConfidential(bool),
    ContainsPersonalInfo(bool),
    
    // Special attributes
    Uid(String),
    Rid(String),
}

impl AttributeValueExtractor for AttributeExpression {
    type AttributeName = String;
    type AttributeValue = String;

    fn get_attribute_value(&self, attr_name: &Self::AttributeName) -> Option<Self::AttributeValue> {
        match (attr_name.as_str(), self) {
            ("role", AttributeExpression::Role(s)) => Some(s.clone()),
            ("position", AttributeExpression::Position(s)) => Some(s.clone()),
            ("tenant", AttributeExpression::Tenant(s)) => Some(s.clone()),
            ("department", AttributeExpression::Department(s)) => Some(s.clone()),
            ("office", AttributeExpression::Office(s)) => Some(s.clone()),
            ("registered", AttributeExpression::Registered(b)) => Some(b.to_string()),
            ("type", AttributeExpression::Type(s)) => Some(s.clone()),
            ("owner", AttributeExpression::Owner(s)) => Some(s.clone()),
            ("isConfidential", AttributeExpression::IsConfidential(b)) => Some(b.to_string()),
            ("containsPersonalInfo", AttributeExpression::ContainsPersonalInfo(b)) => Some(b.to_string()),
            ("payrollingPermissions", AttributeExpression::PayrollingPermissions(b)) => Some(b.to_string()),
            ("uid", AttributeExpression::Uid(s)) => Some(s.clone()),
            ("rid", AttributeExpression::Rid(s)) => Some(s.clone()),
            _ => None,
        }
    }

    fn get_attribute_set(&self, attr_name: &Self::AttributeName) -> Option<Vec<Self::AttributeValue>> {
        match (attr_name.as_str(), self) {
            ("projects", AttributeExpression::Projects(vec)) => Some(vec.clone()),
            ("supervisee", AttributeExpression::Supervisee(vec)) => Some(vec.clone()),
            ("recipients", AttributeExpression::Recipients(vec)) => Some(vec.clone()),
            _ => None,
        }
    }
}

// ユーザー属性の具体的な型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdocumentUserAttribute {
    pub user_id: String,
    pub role: Option<Role>,
    pub position: Option<Position>,
    pub tenant: Option<Tenant>,
    pub department: Option<String>,
    pub office: Option<String>,
    pub registered: Option<bool>,
    pub projects: HashSet<String>,
    pub supervisor: Option<String>,
    pub supervisee: HashSet<String>,
    pub payrolling_permissions: Option<bool>,
}

impl EdocumentUserAttribute {
    pub fn new(user_id: String) -> Self {
        Self {
            user_id,
            role: None,
            position: None,
            tenant: None,
            department: None,
            office: None,
            registered: None,
            projects: HashSet::new(),
            supervisor: None,
            supervisee: HashSet::new(),
            payrolling_permissions: None,
        }
    }
}

impl AttributeValueExtractor for EdocumentUserAttribute {
    type AttributeName = AttributeName;
    type AttributeValue = AttributeValue;

    fn get_attribute_value(&self, attr_name: &AttributeName) -> Option<AttributeValue> {
        match attr_name {
            AttributeName::Role => {
                self.role.as_ref().map(|r| AttributeValue::Role(r.clone()))
            },
            AttributeName::Position => {
                self.position.as_ref().map(|p| AttributeValue::Position(p.clone()))
            },
            AttributeName::Tenant => {
                self.tenant.as_ref().map(|t| AttributeValue::Tenant(t.clone()))
            },
            AttributeName::Department => {
                self.department.as_ref().map(|d| AttributeValue::String(d.clone()))
            },
            AttributeName::Office => {
                self.office.as_ref().map(|o| AttributeValue::String(o.clone()))
            },
            AttributeName::Registered => {
                self.registered.map(|r| AttributeValue::Boolean(r))
            },
            AttributeName::PayrollingPermissions => {
                self.payrolling_permissions.map(|p| AttributeValue::Boolean(p))
            },
            AttributeName::Uid => {
                Some(AttributeValue::String(self.user_id.clone()))
            },
            _ => None,
        }
    }

    fn get_attribute_set(&self, attr_name: &AttributeName) -> Option<Vec<AttributeValue>> {
        match attr_name {
            AttributeName::Projects => {
                Some(self.projects.iter().map(|p| AttributeValue::String(p.clone())).collect())
            },
            AttributeName::Supervisee => {
                Some(self.supervisee.iter().map(|s| AttributeValue::String(s.clone())).collect())
            },
            _ => None,
        }
    }
}

impl UserAttribute for EdocumentUserAttribute {
    fn get_user_id(&self) -> String {
        self.user_id.clone()
    }
}

// リソース属性の具体的な型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdocumentResourceAttribute {
    pub resource_id: String,
    pub resource_type: Option<ResourceType>,
    pub owner: Option<String>,
    pub tenant: Option<Tenant>,
    pub department: Option<String>,
    pub office: Option<String>,
    pub recipients: HashSet<String>,
    pub is_confidential: Option<bool>,
    pub contains_personal_info: Option<bool>,
}

impl EdocumentResourceAttribute {
    pub fn new(resource_id: String, resource_type: &str) -> Result<Self, ParseError> {
        let parsed_type = match resource_type {
            "bankingNote" => ResourceType::BankingNote,
            "trafficFine" => ResourceType::TrafficFine,
            "salesOffer" => ResourceType::SalesOffer,
            "contract" => ResourceType::Contract,
            "invoice" => ResourceType::Invoice,
            "paycheck" => ResourceType::Paycheck,
            _ => return Err(ParseError::UnknownValue(
                "resource_type".to_string(), 
                resource_type.to_string()
            )),
        };

        Ok(Self {
            resource_id,
            resource_type: Some(parsed_type),
            owner: None,
            tenant: None,
            department: None,
            office: None,
            recipients: HashSet::new(),
            is_confidential: None,
            contains_personal_info: None,
        })
    }
}

impl AttributeValueExtractor for EdocumentResourceAttribute {
    type AttributeName = AttributeName;
    type AttributeValue = AttributeValue;

    fn get_attribute_value(&self, attr_name: &AttributeName) -> Option<AttributeValue> {
        match attr_name {
            AttributeName::Type => {
                self.resource_type.as_ref().map(|t| AttributeValue::ResourceType(t.clone()))
            },
            AttributeName::Owner => {
                self.owner.as_ref().map(|o| AttributeValue::String(o.clone()))
            },
            AttributeName::Tenant => {
                self.tenant.as_ref().map(|t| AttributeValue::Tenant(t.clone()))
            },
            AttributeName::Department => {
                self.department.as_ref().map(|d| AttributeValue::String(d.clone()))
            },
            AttributeName::Office => {
                self.office.as_ref().map(|o| AttributeValue::String(o.clone()))
            },
            AttributeName::IsConfidential => {
                self.is_confidential.map(|ic| AttributeValue::Boolean(ic))
            },
            AttributeName::ContainsPersonalInfo => {
                self.contains_personal_info.map(|cpi| AttributeValue::Boolean(cpi))
            },
            AttributeName::Rid => {
                Some(AttributeValue::String(self.resource_id.clone()))
            },
            _ => None,
        }
    }

    fn get_attribute_set(&self, attr_name: &AttributeName) -> Option<Vec<AttributeValue>> {
        match attr_name {
            AttributeName::Recipients => {
                Some(self.recipients.iter().map(|r| AttributeValue::String(r.clone())).collect())
            },
            _ => None,
        }
    }
}

impl ResourceAttribute for EdocumentResourceAttribute {
    fn get_resource_id(&self) -> String {
        self.resource_id.clone()
    }
}

// Edocument rule structure (similar to university rules)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EdocumentRule {
    pub id: usize,
    pub description: String,
    pub user_conditions: Vec<Condition<AttributeExpression>>,      // ユーザー条件
    pub resource_conditions: Vec<Condition<AttributeExpression>>,  // リソース条件
    pub actions: HashSet<Action>,             // アクション
    pub comparison_conditions: Vec<Condition<AttributeExpression>>, // 比較条件
}

impl EdocumentRule {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            description: format!("Rule {}", id + 1),
            user_conditions: Vec::new(),
            resource_conditions: Vec::new(),
            actions: HashSet::new(),
            comparison_conditions: Vec::new(),
        }
    }
}

// Type aliases for edocument domain
pub type EdocumentAbac = AbacData<EdocumentUserAttribute, EdocumentResourceAttribute, EdocumentRule>;

// Domain parser implementation for edocument
#[derive(Debug, Clone)]
pub struct EdocumentDomainParser;

impl DomainParser for EdocumentDomainParser {
    type UserAttribute = EdocumentUserAttribute;
    type ResourceAttribute = EdocumentResourceAttribute;
    type Rule = EdocumentRule;
    type AttributeExpression = AttributeExpression;
    type AttributeName = String;
    type AttributeValue = String;
    type Action = Action;

    fn parse_attribute_name(&self, s: &str) -> Result<Self::AttributeName, ParseError> {
        Ok(s.to_string())
    }

    fn parse_attribute_value(&self, s: &str) -> Result<Self::AttributeValue, ParseError> {
        Ok(s.to_string())
    }

    fn parse_action(&self, s: &str) -> Result<Self::Action, ParseError> {
        match s {
            "view" => Ok(Action::View),
            "send" => Ok(Action::Send),
            "search" => Ok(Action::Search),
            "readMetaInfo" => Ok(Action::ReadMetaInfo),
            "edit" => Ok(Action::Edit),
            "approve" => Ok(Action::Approve),
            _ => Err(ParseError::UnknownValue(
                "action".to_string(),
                s.to_string()
            )),
        }
    }

    fn parse_attribute_expression(&self, expr_str: &str) -> Result<Self::AttributeExpression, ParseError> {
        let expr_str = expr_str.trim();

        // 属性名のチェック（ABACルールの左辺で使用）
        match expr_str {
            // 属性名 - 属性名の場合は特別なマーカーとして扱う
            "role" => return Ok(AttributeExpression::Role("__ATTRIBUTE_NAME__".to_string())),
            "position" => return Ok(AttributeExpression::Position("__ATTRIBUTE_NAME__".to_string())),
            "tenant" => return Ok(AttributeExpression::Tenant("__ATTRIBUTE_NAME__".to_string())),
            "department" => return Ok(AttributeExpression::Department("__ATTRIBUTE_NAME__".to_string())),
            "office" => return Ok(AttributeExpression::Office("__ATTRIBUTE_NAME__".to_string())),
            "registered" => return Ok(AttributeExpression::Registered(false)), // placeholder for attribute name
            "projects" => return Ok(AttributeExpression::Projects(vec!["__ATTRIBUTE_NAME__".to_string()])), // placeholder
            "supervisor" => return Ok(AttributeExpression::Supervisor(Some("__ATTRIBUTE_NAME__".to_string()))),
            "supervisee" => return Ok(AttributeExpression::Supervisee(vec!["__ATTRIBUTE_NAME__".to_string()])), // placeholder
            "payrollingPermissions" => return Ok(AttributeExpression::PayrollingPermissions(false)), // placeholder for attribute name
            "type" => return Ok(AttributeExpression::Type("__ATTRIBUTE_NAME__".to_string())),
            "owner" => return Ok(AttributeExpression::Owner("__ATTRIBUTE_NAME__".to_string())),
            "recipients" => return Ok(AttributeExpression::Recipients(vec!["__ATTRIBUTE_NAME__".to_string()])), // placeholder
            "isConfidential" => return Ok(AttributeExpression::IsConfidential(false)), // placeholder for attribute name
            "containsPersonalInfo" => return Ok(AttributeExpression::ContainsPersonalInfo(false)), // placeholder for attribute name
            "uid" => return Ok(AttributeExpression::Uid("__ATTRIBUTE_NAME__".to_string())),
            "rid" => return Ok(AttributeExpression::Rid("__ATTRIBUTE_NAME__".to_string())),
            _ => {}
        }

        // 波括弧で囲まれたセットかチェック
        if expr_str.starts_with('{') && expr_str.ends_with('}') {
            let content = &expr_str[1..expr_str.len()-1];
            let mut values = Vec::new();
            
            for value_str in content.split_whitespace() {
                values.push(value_str.to_string());
            }
            
            // どの属性タイプかによって適切な表現を返す
            // 簡単のため、最初の値で判定
            if let Some(first_value) = values.first() {
                match first_value.as_str() {
                    "employee" | "helpdesk" | "admin" | "customer" => {
                        return Ok(AttributeExpression::Role(values[0].clone()));
                    },
                    "secretary" | "insuranceAgent" | "officeManager" | "seniorOfficeManager" | "director" | "none" => {
                        return Ok(AttributeExpression::Position(values[0].clone()));
                    },
                    "bankingNote" | "trafficFine" | "salesOffer" | "contract" | "invoice" | "paycheck" => {
                        return Ok(AttributeExpression::Type(values[0].clone()));
                    },
                    "view" | "send" | "search" | "readMetaInfo" | "edit" | "approve" => {
                        // Actions are handled separately, but if we encounter them here, return as string
                        return Ok(AttributeExpression::Role(values[0].clone()));
                    },
                    "True" | "False" => {
                        let bool_val = values[0] == "True";
                        return Ok(AttributeExpression::Registered(bool_val));
                    },
                    _ => {
                        // Check if it's a project or user list
                        return Ok(AttributeExpression::Projects(values));
                    }
                }
            }
        }

        // Single value - determine type based on content
        match expr_str {
            // Role values
            "employee" | "helpdesk" | "admin" | "customer" => {
                Ok(AttributeExpression::Role(expr_str.to_string()))
            },
            // Position values
            "secretary" | "insuranceAgent" | "officeManager" | "seniorOfficeManager" | "director" | "none" => {
                Ok(AttributeExpression::Position(expr_str.to_string()))
            },
            // Resource types
            "bankingNote" | "trafficFine" | "salesOffer" | "contract" | "invoice" | "paycheck" => {
                Ok(AttributeExpression::Type(expr_str.to_string()))
            },
            // Boolean values
            "True" => Ok(AttributeExpression::Registered(true)),
            "False" => Ok(AttributeExpression::Registered(false)),
            // Tenant values
            "londonOffice" | "largeBank" | "newsAgency" | "reseller" | "europeRegion" | 
            "largeBankLeasing" | "carLeaser" | "ictProvider" | "privateReceiver" => {
                Ok(AttributeExpression::Tenant(expr_str.to_string()))
            },
            // Default: treat as string for departments, offices, etc.
            _ => {
                Ok(AttributeExpression::Department(expr_str.to_string()))
            }
        }
    }

    fn parse_user_attribute_field(
        &self,
        user: &mut Self::UserAttribute,
        key: &str,
        value: &str
    ) -> Result<(), ParseError> {
        match key {
            "role" => {
                user.role = Some(match value {
                    "employee" => Role::Employee,
                    "helpdesk" => Role::Helpdesk,
                    "admin" => Role::Admin,
                    "customer" => Role::Customer,
                    _ => return Err(ParseError::UnknownValue(
                        key.to_string(),
                        value.to_string()
                    )),
                });
            },
            "position" => {
                user.position = Some(match value {
                    "secretary" => Position::Secretary,
                    "insuranceAgent" => Position::InsuranceAgent,
                    "officeManager" => Position::OfficeManager,
                    "seniorOfficeManager" => Position::SeniorOfficeManager,
                    "director" => Position::Director,
                    "none" => Position::None,
                    _ => return Err(ParseError::UnknownValue(
                        key.to_string(),
                        value.to_string()
                    )),
                });
            },
            "tenant" => {
                user.tenant = Some(self.parse_tenant_value(value)?);
            },
            "department" => {
                user.department = Some(value.to_string());
            },
            "office" => {
                user.office = Some(value.to_string());
            },
            "registered" => {
                user.registered = Some(match value {
                    "True" => true,
                    "False" => false,
                    _ => return Err(ParseError::UnknownValue(
                        key.to_string(),
                        value.to_string()
                    )),
                });
            },
            "projects" => {
                user.projects = self.parse_set_value(value)?;
            },
            "supervisor" => {
                user.supervisor = if value == "none" { None } else { Some(value.to_string()) };
            },
            "supervisee" => {
                user.supervisee = self.parse_set_value(value)?;
            },
            "payrollingPermissions" => {
                user.payrolling_permissions = Some(match value {
                    "True" => true,
                    "False" => false,
                    _ => return Err(ParseError::UnknownValue(
                        key.to_string(),
                        value.to_string()
                    )),
                });
            },
            _ => return Err(ParseError::MissingAttribute(key.to_string())),
        }
        Ok(())
    }

    fn parse_resource_attribute_field(
        &self,
        resource: &mut Self::ResourceAttribute,
        key: &str,
        value: &str
    ) -> Result<(), ParseError> {
        match key {
            "type" => {
                resource.resource_type = Some(match value {
                    "bankingNote" => ResourceType::BankingNote,
                    "trafficFine" => ResourceType::TrafficFine,
                    "salesOffer" => ResourceType::SalesOffer,
                    "contract" => ResourceType::Contract,
                    "invoice" => ResourceType::Invoice,
                    "paycheck" => ResourceType::Paycheck,
                    _ => return Err(ParseError::UnknownValue(
                        key.to_string(),
                        value.to_string()
                    )),
                });
            },
            "owner" => {
                resource.owner = Some(value.to_string());
            },
            "tenant" => {
                resource.tenant = Some(self.parse_tenant_value(value)?);
            },
            "department" => {
                resource.department = Some(value.to_string());
            },
            "office" => {
                resource.office = Some(value.to_string());
            },
            "recipients" => {
                resource.recipients = self.parse_set_value(value)?;
            },
            "isConfidential" => {
                resource.is_confidential = Some(match value {
                    "True" => true,
                    "False" => false,
                    _ => return Err(ParseError::UnknownValue(
                        key.to_string(),
                        value.to_string()
                    )),
                });
            },
            "containsPersonalInfo" => {
                resource.contains_personal_info = Some(match value {
                    "True" => true,
                    "False" => false,
                    _ => return Err(ParseError::UnknownValue(
                        key.to_string(),
                        value.to_string()
                    )),
                });
            },
            _ => return Err(ParseError::MissingAttribute(key.to_string())),
        }
        Ok(())
    }

    fn create_user_attribute(&self, user_id: String) -> Self::UserAttribute {
        EdocumentUserAttribute::new(user_id)
    }

    fn create_resource_attribute(&self, resource_id: String, resource_type: &str) -> Result<Self::ResourceAttribute, ParseError> {
        EdocumentResourceAttribute::new(resource_id, resource_type)
    }

    fn create_rule(
        &self,
        id: usize,
        user_conditions: Vec<Condition<Self::AttributeExpression>>,
        resource_conditions: Vec<Condition<Self::AttributeExpression>>,
        actions: std::collections::HashSet<Self::Action>,
        comparison_conditions: Vec<Condition<Self::AttributeExpression>>,
    ) -> Self::Rule {
        // Simplified rule creation - create a basic condition
        // In a real implementation, this would construct a proper rule from the given conditions
        EdocumentRule {
            id,
            description: format!("Rule {}", id + 1),
            user_conditions,
            resource_conditions,
            actions,
            comparison_conditions,
        }
    }
}

impl EdocumentDomainParser {
    fn parse_set_value(&self, value: &str) -> Result<HashSet<String>, ParseError> {
        if value.trim() == "{}" {
            return Ok(HashSet::new());
        }
        
        let trimmed = value.trim();
        if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
            return Err(ParseError::InvalidFormat(value.to_string()));
        }
        
        let inner = &trimmed[1..trimmed.len()-1];
        if inner.trim().is_empty() {
            return Ok(HashSet::new());
        }
        
        let elements: HashSet<String> = inner
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        
        Ok(elements)
    }

    fn parse_tenant_value(&self, value: &str) -> Result<Tenant, ParseError> {
        match value {
            "londonOffice" => Ok(Tenant::LondonOffice),
            "largeBank" => Ok(Tenant::LargeBank),
            "newsAgency" => Ok(Tenant::NewsAgency),
            "reseller" => Ok(Tenant::Reseller),
            "europeRegion" => Ok(Tenant::EuropeRegion),
            "largeBankLeasing" => Ok(Tenant::LargeBankLeasing),
            "carLeaser" => Ok(Tenant::CarLeaser),
            "ictProvider" => Ok(Tenant::IctProvider),
            "privateReceiver" => Ok(Tenant::PrivateReceiver),
            _ => Err(ParseError::UnknownValue(
                "tenant".to_string(),
                value.to_string()
            )),
        }
    }
}

// Re-export commonly used types
pub use EdocumentAbac as EdocumentAbacData;
