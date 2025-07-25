use std::collections::HashSet;
use std::str::FromStr;
use serde::{Serialize, Deserialize};
// 再エクスポートして他のモジュールから使えるようにする
pub use crate::types::{ComparisonOperator, AttributeValueExtractor, UserAttribute, ResourceAttribute, Condition, AbacData, ParseError, DomainParser, GenericAbacParser};

// ユーザーの役職を表現
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Role {
    Employee,
    Manager,
    Admin,
    Helpdesk,
    Customer,
}

// ポジションを表現
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Position {
    Secretary,
    Director,
    SeniorOfficeManager,
    OfficeManager,
    InsuranceAgent,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DocumentType {
    Invoice,
    Contract,
    Paycheck,
    BankingNote,
    SalesOffer,
    TrafficFine,
    None,
}

// テナントを表現
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Tenant {
    LargeBank,
    LargeBankLeasing,
    NewsAgency,
    EuropeRegion,
    LondonOffice,
    Reseller,
    CarLeaser,
    IctProvider,
    PrivateReceiver,
}

// 部門を表現（すべてのテナントの部門を含む）
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Department {
    // LargeBank departments
    LargeBankSales,
    LargeBankICT,
    LargeBankHR,
    LargeBankIT,
    LargeBankAudit,
    
    // LargeBankLeasing departments
    LargeBankLeasingCustomerCare,
    LargeBankLeasingSales,
    
    // NewsAgency departments
    NewsAgencyAudit,
    NewsAgencyIT,
    
    // EuropeRegion departments
    EuropeRegionIT,
    EuropeRegionHR,
    
    // LondonOffice departments
    LondonOfficeAudit,
    LondonOfficeHR,
    LondonOfficeSales,
    
    // Reseller departments
    ResellerSales,
    ResellerCustomer,
    ResellerAccounting,
    
    // CarLeaser departments
    CarLeaserAudit,
    CarLeaserSecretary,
    CarLeaserAccounting,
    
    // IctProvider departments
    IctProviderAudit,
    IctProviderSecretary,
    IctProviderAccounting,
    IctProviderICT,
    
    // PrivateReceiver departments
    PrivateReceiverAudit,
    PrivateReceiverSecretary,
    PrivateReceiverAccounting,
    
    None,
}

// オフィスを表現（テナントごとに異なる数のオフィス）
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Office {
    // LargeBank offices (1-10)
    LargeBankOffice1,
    LargeBankOffice2,
    LargeBankOffice3,
    LargeBankOffice4,
    LargeBankOffice5,
    LargeBankOffice6,
    LargeBankOffice7,
    LargeBankOffice8,
    LargeBankOffice9,
    LargeBankOffice10,
    
    // LargeBankLeasing offices (1-2)
    LargeBankLeasingOffice1,
    LargeBankLeasingOffice2,
    
    // IctProvider offices (1-5)
    IctProviderOffice1,
    IctProviderOffice2,
    IctProviderOffice3,
    IctProviderOffice4,
    IctProviderOffice5,
    
    None,
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
    ClearanceLevel, // Added
    SecurityLevel, // Added
    Type,
    Owner,
    Recipients,
    IsConfidential,
    ContainsPersonalInfo,
    Uid,
    Rid,
}

// 属性値の型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum AttributeValue {
    Role(Role),
    Position(Position),
    Tenant(Tenant),
    ResourceType(DocumentType),
    String(String),
    Boolean(bool),
    Integer(i32),
    StringSet(Vec<String>),
}

impl PartialOrd for AttributeValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (AttributeValue::Integer(a), AttributeValue::Integer(b)) => a.partial_cmp(b),
            _ => None, // Comparison is not defined for other types
        }
    }
}

impl Ord for AttributeValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

// 属性表現を表現
// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// pub enum AttributeExpression {
//     AttributeName(AttributeName),
//     AttributeValue(AttributeValue),
//     ValueSet(Vec<AttributeValue>),
// }

// 属性表現の型エイリアス
pub type AttributeExpression = crate::types::AttributeExpression<AttributeName, AttributeValue>;

// ユーザー属性の具体的な型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdocumentUserAttribute {
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Tenant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub office: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered: Option<bool>,
    pub projects: HashSet<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supervisor: Option<String>,
    pub supervisee: HashSet<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payrolling_permissions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clearance_level: Option<i32>,
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
            clearance_level: None,
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
            AttributeName::ClearanceLevel => {
                self.clearance_level.map(|cl| AttributeValue::Integer(cl))
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<DocumentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Tenant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub office: Option<String>,
    pub recipients: HashSet<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_confidential: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains_personal_info: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_level: Option<i32>,
}

impl EdocumentResourceAttribute {
    pub fn new(resource_id: String, resource_type: &str) -> Result<Self, ParseError> {
        let parsed_type = Self::parse_document_type(resource_type)?;

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
            security_level: None,
        })
    }

    fn parse_document_type(doc_type: &str) -> Result<DocumentType, ParseError> {
        match doc_type {
            "bankingNote" => Ok(DocumentType::BankingNote),
            "trafficFine" => Ok(DocumentType::TrafficFine),
            "salesOffer" => Ok(DocumentType::SalesOffer),
            "contract" => Ok(DocumentType::Contract),
            "invoice" => Ok(DocumentType::Invoice),
            "paycheck" => Ok(DocumentType::Paycheck),
            _ => Err(ParseError::UnknownValue(
                "resource_type".to_string(), 
                doc_type.to_string()
            )),
        }
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
            AttributeName::SecurityLevel => {
                self.security_level.map(|sl| AttributeValue::Integer(sl))
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
    type AttributeName = AttributeName;
    type AttributeValue = AttributeValue;
    type Action = Action;

    fn parse_attribute_name(&self, s: &str) -> Result<Self::AttributeName, ParseError> {
        AttributeName::from_str(s)
    }

    fn parse_attribute_value(&self, s: &str) -> Result<Self::AttributeValue, ParseError> {
        match s {
            "True" => return Ok(AttributeValue::Boolean(true)),
            "False" => return Ok(AttributeValue::Boolean(false)),
            _ => {}
        }

        // Role
        if let Ok(role) = self.parse_role_value(s) {
            return Ok(AttributeValue::Role(role));
        }
        // Position
        if let Ok(pos) = self.parse_position_value(s) {
            return Ok(AttributeValue::Position(pos));
        }
        // Tenant
        if let Ok(tenant) = self.parse_tenant_value(s) {
            return Ok(AttributeValue::Tenant(tenant));
        }
        // DocumentType
        if let Ok(doc_type) = self.parse_document_type(s) {
            return Ok(AttributeValue::ResourceType(doc_type));
        }
        // Default to String
        Ok(AttributeValue::String(s.to_string()))
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

        // 属性名かどうかを最初に試す
        if let Ok(name) = self.parse_attribute_name(expr_str) {
            return Ok(AttributeExpression::AttributeName(name));
        }

        // 波括弧で囲まれたセットかチェック
        if expr_str.starts_with('{') && expr_str.ends_with('}') {
            let content = &expr_str[1..expr_str.len() - 1];
            let values = content
                .split_whitespace()
                .map(|s| self.parse_attribute_value(s))
                .collect::<Result<Vec<_>, _>>()?;
            return Ok(AttributeExpression::ValueSet(values));
        }

        // 最後に属性値として解析
        let value = self.parse_attribute_value(expr_str)?;
        Ok(AttributeExpression::AttributeValue(value))
    }

    fn parse_user_attribute_field(
        &self,
        user: &mut Self::UserAttribute,
        key: &str,
        value: &str
    ) -> Result<(), ParseError> {
        match key {
            "role" => {
                user.role = Some(self.parse_role_value(value)?);
            },
            "position" => {
                user.position = Some(self.parse_position_value(value)?);
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
                resource.resource_type = Some(EdocumentResourceAttribute::parse_document_type(value)?);
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

    fn parse_role_value(&self, value: &str) -> Result<Role, ParseError> {
        match value {
            "employee" => Ok(Role::Employee),
            "manager" => Ok(Role::Manager),
            "admin" => Ok(Role::Admin),
            "helpdesk" => Ok(Role::Helpdesk),
            "customer" => Ok(Role::Customer),
            _ => Err(ParseError::UnknownValue(
                "role".to_string(),
                value.to_string()
            )),
        }
    }

    fn parse_position_value(&self, value: &str) -> Result<Position, ParseError> {
        match value {
            "secretary" => Ok(Position::Secretary),
            "director" => Ok(Position::Director),
            "seniorOfficeManager" => Ok(Position::SeniorOfficeManager),
            "officeManager" => Ok(Position::OfficeManager),
            "insuranceAgent" => Ok(Position::InsuranceAgent),
            "none" => Ok(Position::None),
            _ => Err(ParseError::UnknownValue(
                "position".to_string(),
                value.to_string()
            )),
        }
    }

    fn parse_document_type(&self, value: &str) -> Result<DocumentType, ParseError> {
        match value {
            "bankingNote" => Ok(DocumentType::BankingNote),
            "trafficFine" => Ok(DocumentType::TrafficFine),
            "salesOffer" => Ok(DocumentType::SalesOffer),
            "contract" => Ok(DocumentType::Contract),
            "invoice" => Ok(DocumentType::Invoice),
            "paycheck" => Ok(DocumentType::Paycheck),
            _ => Err(ParseError::UnknownValue(
                "document_type".to_string(),
                value.to_string()
            )),
        }
    }

    fn parse_tenant_value(&self, value: &str) -> Result<Tenant, ParseError> {
        match value {
            "largeBank" => Ok(Tenant::LargeBank),
            "largeBankLeasing" => Ok(Tenant::LargeBankLeasing),
            "newsAgency" => Ok(Tenant::NewsAgency),
            "europeRegion" => Ok(Tenant::EuropeRegion),
            "londonOffice" => Ok(Tenant::LondonOffice),
            "reseller" => Ok(Tenant::Reseller),
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

pub use EdocumentAbac as EdocumentAbacData;

// 属性名の文字列からの変換
impl FromStr for AttributeName {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "role" => Ok(AttributeName::Role),
            "position" => Ok(AttributeName::Position),
            "tenant" => Ok(AttributeName::Tenant),
            "department" => Ok(AttributeName::Department),
            "office" => Ok(AttributeName::Office),
            "registered" => Ok(AttributeName::Registered),
            "projects" => Ok(AttributeName::Projects),
            "supervisor" => Ok(AttributeName::Supervisor),
            "supervisee" => Ok(AttributeName::Supervisee),
            "payrollingPermissions" => Ok(AttributeName::PayrollingPermissions),
            "type" => Ok(AttributeName::Type),
            "owner" => Ok(AttributeName::Owner),
            "recipients" => Ok(AttributeName::Recipients),
            "isConfidential" => Ok(AttributeName::IsConfidential),
            "containsPersonalInfo" => Ok(AttributeName::ContainsPersonalInfo),
            "uid" => Ok(AttributeName::Uid),
            "rid" => Ok(AttributeName::Rid),
            _ => Err(ParseError::UnknownValue("attribute_name".to_string(), s.to_string())),
        }
    }
}
