use std::collections::HashSet;
use std::str::FromStr;
use serde::{Serialize, Deserialize};
// 再エクスポートして他のモジュールから使えるようにする
pub use crate::types::types::{ComparisonOperator, AttributeValueExtractor, UserAttribute, ResourceAttribute, Condition, AbacData, ParseError, DomainParser, GenericAbacParser};

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

// ドキュメントの種類を表現
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

// 属性表現の型エイリアス
pub type AttributeExpression = crate::types::types::AttributeExpression<AttributeName, AttributeValue>;

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

pub type EdocumentAbac = AbacData<EdocumentUserAttribute, EdocumentResourceAttribute, EdocumentRule>;
pub use EdocumentAbac as EdocumentAbacData;

impl std::fmt::Display for AttributeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttributeName::Role => write!(f, "role"),
            AttributeName::Position => write!(f, "position"),
            AttributeName::Tenant => write!(f, "tenant"),
            AttributeName::Department => write!(f, "department"),
            AttributeName::Office => write!(f, "office"),
            AttributeName::Registered => write!(f, "registered"),
            AttributeName::Projects => write!(f, "projects"),
            AttributeName::Supervisor => write!(f, "supervisor"),
            AttributeName::Supervisee => write!(f, "supervisee"),
            AttributeName::PayrollingPermissions => write!(f, "payrollingPermissions"),
            AttributeName::ClearanceLevel => write!(f, "clearanceLevel"),
            AttributeName::SecurityLevel => write!(f, "securityLevel"),
            AttributeName::Type => write!(f, "type"),
            AttributeName::Owner => write!(f, "owner"),
            AttributeName::Recipients => write!(f, "recipients"),
            AttributeName::IsConfidential => write!(f, "isConfidential"),
            AttributeName::ContainsPersonalInfo => write!(f, "containsPersonalInfo"),
            AttributeName::Uid => write!(f, "uid"),
            AttributeName::Rid => write!(f, "rid"),
        }
    }
}
