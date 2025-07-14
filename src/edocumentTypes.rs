use std::collections::HashSet;

// ユーザーの役割を表現（拡張版）
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Role {
    Employee,
    Manager,
    Admin,
    Helpdesk,
    Customer,
    ProjectManager,
    LegalOfficer,
    FinancialOfficer,
    Auditor,
    Consultant,
}

// ユーザーの役職を表現（拡張版）
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Position {
    Secretary,
    Director,
    SeniorOfficeManager,
    OfficeManager,
    InsuranceAgent,
    Analyst,
    Specialist,
    Coordinator,
    Lead,
    Associate,
    Senior,
    Principal,
    VicePresident,
    None,
}

// セキュリティクリアランスレベル
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SecurityLevel {
    Public,
    Internal,
    Confidential,
    Secret,
    TopSecret,
}

// 顧客ティア
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CustomerTier {
    Bronze,
    Silver,
    Gold,
    Platinum,
    Vip,
    None,
}

// 地域
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Region {
    NorthAmerica,
    Europe,
    Asia,
    LatinAmerica,
    Africa,
}

// 国
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Country {
    // North America
    USA,
    Canada,
    Mexico,
    // Europe
    UK,
    Germany,
    France,
    Netherlands,
    Spain,
    // Asia
    Japan,
    China,
    India,
    Singapore,
    SouthKorea,
    // Latin America
    Brazil,
    Argentina,
    Chile,
    Colombia,
    // Africa
    SouthAfrica,
    Nigeria,
    Egypt,
    Kenya,
}

// タイムゾーン
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TimeZone {
    UTC,
    EST,
    PST,
    GMT,
    CET,
    JST,
    IST,
}

// 契約タイプ
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContractType {
    Permanent,
    Temporary,
    Contractor,
    Consultant,
}

// 承認ステータス
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ApprovalStatus {
    Draft,
    Pending,
    Approved,
    Rejected,
    Archived,
}

// 優先度
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

// ファイルフォーマット
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Format {
    Pdf,
    Docx,
    Xlsx,
    Txt,
    Pptx,
    Xml,
    Json,
}

// 言語
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Language {
    En,
    Es,
    Fr,
    De,
    Ja,
    Zh,
    Pt,
}

// テナントを表現（拡張版）
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Tenant {
    // Organization tenants
    LargeBank,
    LargeBankLeasing,
    NewsAgency,
    EuropeRegion,
    LondonOffice,
    Reseller,
    TechCorp,
    Pharmaceuticals,
    EnergyCompany,
    RetailChain,
    ConsultingFirm,
    LawFirm,
    FinancialServices,
    HealthcareSystem,
    EducationInstitute,
    // Customer tenants
    CarLeaser,
    IctProvider,
    PrivateReceiver,
    SmallBusiness,
    StartupCompany,
    GovernmentAgency,
    Nonprofit,
    MediaCompany,
}

// 部署を表現（拡張版）
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Department {
    // LargeBank departments
    LargeBankSales,
    LargeBankICT,
    LargeBankIT,
    LargeBankHR,
    LargeBankAudit,
    LargeBankLegal,
    LargeBankRisk,
    LargeBankCompliance,
    
    // LargeBankLeasing departments
    LargeBankLeasingSales,
    LargeBankLeasingCustomerCare,
    LargeBankLeasingRisk,
    LargeBankLeasingOperations,
    
    // Reseller departments
    ResellerSales,
    ResellerAccounting,
    ResellerCustomer,
    ResellerSupport,
    ResellerMarketing,
    
    // NewsAgency departments
    NewsAgencyIT,
    NewsAgencyAudit,
    NewsAgencyEditorial,
    NewsAgencyMarketing,
    NewsAgencyLegal,
    
    // EuropeRegion departments
    EuropeRegionHR,
    EuropeRegionIT,
    EuropeRegionFinance,
    EuropeRegionStrategy,
    EuropeRegionCompliance,
    
    // LondonOffice departments
    LondonOfficeAudit,
    LondonOfficeHR,
    LondonOfficeSales,
    LondonOfficeOperations,
    LondonOfficeResearch,
    
    // TechCorp departments
    TechCorpEngineering,
    TechCorpProduct,
    TechCorpSales,
    TechCorpSupport,
    TechCorpResearch,
    TechCorpSecurity,
    
    // Pharmaceuticals departments
    PharmaceuticalsResearch,
    PharmaceuticalsRegulatory,
    PharmaceuticalsSales,
    PharmaceuticalsManufacturing,
    
    // EnergyCompany departments
    EnergyCompanyExploration,
    EnergyCompanyProduction,
    EnergyCompanyRefining,
    EnergyCompanyTradingAudit,
    
    // RetailChain departments
    RetailChainOperations,
    RetailChainMarketing,
    RetailChainSupplyChain,
    RetailChainFinance,
    
    // ConsultingFirm departments
    ConsultingFirmStrategy,
    ConsultingFirmTechnology,
    ConsultingFirmOperations,
    ConsultingFirmHR,
    
    // LawFirm departments
    LawFirmCorporate,
    LawFirmLitigation,
    LawFirmIntellectualProperty,
    LawFirmTax,
    LawFirmEmployment,
    
    // FinancialServices departments
    FinancialServicesWealth,
    FinancialServicesRetail,
    FinancialServicesInvestment,
    FinancialServicesRisk,
    
    // HealthcareSystem departments
    HealthcareSystemClinical,
    HealthcareSystemAdministrative,
    HealthcareSystemResearch,
    HealthcareSystemIT,
    
    // EducationInstitute departments
    EducationInstituteAcademic,
    EducationInstituteAdministrative,
    EducationInstituteResearch,
    EducationInstituteIT,
    
    // CarLeaser departments
    CarLeaserAudit,
    CarLeaserSecretary,
    CarLeaserAccounting,
    CarLeaserOperations,
    CarLeaserMarketing,
    
    // IctProvider departments
    IctProviderAudit,
    IctProviderSecretary,
    IctProviderAccounting,
    IctProviderOperations,
    IctProviderMarketing,
    IctProviderICT,
    IctProviderDevelopment,
    IctProviderSupport,
    
    // PrivateReceiver departments
    PrivateReceiverAudit,
    PrivateReceiverSecretary,
    PrivateReceiverAccounting,
    PrivateReceiverOperations,
    PrivateReceiverMarketing,
    
    // SmallBusiness departments
    SmallBusinessAudit,
    SmallBusinessSecretary,
    SmallBusinessAccounting,
    SmallBusinessOperations,
    SmallBusinessMarketing,
    
    // StartupCompany departments
    StartupCompanyAudit,
    StartupCompanySecretary,
    StartupCompanyAccounting,
    StartupCompanyOperations,
    StartupCompanyMarketing,
    StartupCompanyProduct,
    StartupCompanyEngineering,
    StartupCompanyBusinessDevelopment,
    
    // GovernmentAgency departments
    GovernmentAgencyAudit,
    GovernmentAgencySecretary,
    GovernmentAgencyAccounting,
    GovernmentAgencyOperations,
    GovernmentAgencyMarketing,
    GovernmentAgencyPolicy,
    GovernmentAgencyPublicRelations,
    GovernmentAgencyCompliance,
    
    // Nonprofit departments
    NonprofitAudit,
    NonprofitSecretary,
    NonprofitAccounting,
    NonprofitOperations,
    NonprofitMarketing,
    
    // MediaCompany departments
    MediaCompanyAudit,
    MediaCompanySecretary,
    MediaCompanyAccounting,
    MediaCompanyOperations,
    MediaCompanyMarketing,
    
    // None for helpdesk/admin users
    None,
}

// オフィスを表現
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Office {
    // LargeBank offices (15個)
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
    LargeBankOffice11,
    LargeBankOffice12,
    LargeBankOffice13,
    LargeBankOffice14,
    LargeBankOffice15,
    
    // LargeBankLeasing offices (5個)
    LargeBankLeasingOffice1,
    LargeBankLeasingOffice2,
    LargeBankLeasingOffice3,
    LargeBankLeasingOffice4,
    LargeBankLeasingOffice5,
    
    // NewsAgency offices (6個)
    NewsAgencyOffice1,
    NewsAgencyOffice2,
    NewsAgencyOffice3,
    NewsAgencyOffice4,
    NewsAgencyOffice5,
    NewsAgencyOffice6,
    
    // EuropeRegion offices (12個)
    EuropeRegionOffice1,
    EuropeRegionOffice2,
    EuropeRegionOffice3,
    EuropeRegionOffice4,
    EuropeRegionOffice5,
    EuropeRegionOffice6,
    EuropeRegionOffice7,
    EuropeRegionOffice8,
    EuropeRegionOffice9,
    EuropeRegionOffice10,
    EuropeRegionOffice11,
    EuropeRegionOffice12,
    
    // LondonOffice offices (4個)
    LondonOfficeOffice1,
    LondonOfficeOffice2,
    LondonOfficeOffice3,
    LondonOfficeOffice4,
    
    // Reseller offices (7個)
    ResellerOffice1,
    ResellerOffice2,
    ResellerOffice3,
    ResellerOffice4,
    ResellerOffice5,
    ResellerOffice6,
    ResellerOffice7,
    
    // TechCorp offices (20個)
    TechCorpOffice1,
    TechCorpOffice2,
    TechCorpOffice3,
    TechCorpOffice4,
    TechCorpOffice5,
    TechCorpOffice6,
    TechCorpOffice7,
    TechCorpOffice8,
    TechCorpOffice9,
    TechCorpOffice10,
    TechCorpOffice11,
    TechCorpOffice12,
    TechCorpOffice13,
    TechCorpOffice14,
    TechCorpOffice15,
    TechCorpOffice16,
    TechCorpOffice17,
    TechCorpOffice18,
    TechCorpOffice19,
    TechCorpOffice20,
    
    // Pharmaceuticals offices (18個)
    PharmaceuticalsOffice1,
    PharmaceuticalsOffice2,
    PharmaceuticalsOffice3,
    PharmaceuticalsOffice4,
    PharmaceuticalsOffice5,
    PharmaceuticalsOffice6,
    PharmaceuticalsOffice7,
    PharmaceuticalsOffice8,
    PharmaceuticalsOffice9,
    PharmaceuticalsOffice10,
    PharmaceuticalsOffice11,
    PharmaceuticalsOffice12,
    PharmaceuticalsOffice13,
    PharmaceuticalsOffice14,
    PharmaceuticalsOffice15,
    PharmaceuticalsOffice16,
    PharmaceuticalsOffice17,
    PharmaceuticalsOffice18,
    
    // EnergyCompany offices (25個)
    EnergyCompanyOffice1,
    EnergyCompanyOffice2,
    EnergyCompanyOffice3,
    EnergyCompanyOffice4,
    EnergyCompanyOffice5,
    EnergyCompanyOffice6,
    EnergyCompanyOffice7,
    EnergyCompanyOffice8,
    EnergyCompanyOffice9,
    EnergyCompanyOffice10,
    EnergyCompanyOffice11,
    EnergyCompanyOffice12,
    EnergyCompanyOffice13,
    EnergyCompanyOffice14,
    EnergyCompanyOffice15,
    EnergyCompanyOffice16,
    EnergyCompanyOffice17,
    EnergyCompanyOffice18,
    EnergyCompanyOffice19,
    EnergyCompanyOffice20,
    EnergyCompanyOffice21,
    EnergyCompanyOffice22,
    EnergyCompanyOffice23,
    EnergyCompanyOffice24,
    EnergyCompanyOffice25,
    
    // RetailChain offices (50個)
    RetailChainOffice1,
    RetailChainOffice2,
    RetailChainOffice3,
    RetailChainOffice4,
    RetailChainOffice5,
    RetailChainOffice6,
    RetailChainOffice7,
    RetailChainOffice8,
    RetailChainOffice9,
    RetailChainOffice10,
    RetailChainOffice11,
    RetailChainOffice12,
    RetailChainOffice13,
    RetailChainOffice14,
    RetailChainOffice15,
    RetailChainOffice16,
    RetailChainOffice17,
    RetailChainOffice18,
    RetailChainOffice19,
    RetailChainOffice20,
    RetailChainOffice21,
    RetailChainOffice22,
    RetailChainOffice23,
    RetailChainOffice24,
    RetailChainOffice25,
    RetailChainOffice26,
    RetailChainOffice27,
    RetailChainOffice28,
    RetailChainOffice29,
    RetailChainOffice30,
    RetailChainOffice31,
    RetailChainOffice32,
    RetailChainOffice33,
    RetailChainOffice34,
    RetailChainOffice35,
    RetailChainOffice36,
    RetailChainOffice37,
    RetailChainOffice38,
    RetailChainOffice39,
    RetailChainOffice40,
    RetailChainOffice41,
    RetailChainOffice42,
    RetailChainOffice43,
    RetailChainOffice44,
    RetailChainOffice45,
    RetailChainOffice46,
    RetailChainOffice47,
    RetailChainOffice48,
    RetailChainOffice49,
    RetailChainOffice50,
    
    // ConsultingFirm offices (10個)
    ConsultingFirmOffice1,
    ConsultingFirmOffice2,
    ConsultingFirmOffice3,
    ConsultingFirmOffice4,
    ConsultingFirmOffice5,
    ConsultingFirmOffice6,
    ConsultingFirmOffice7,
    ConsultingFirmOffice8,
    ConsultingFirmOffice9,
    ConsultingFirmOffice10,
    
    // LawFirm offices (8個)
    LawFirmOffice1,
    LawFirmOffice2,
    LawFirmOffice3,
    LawFirmOffice4,
    LawFirmOffice5,
    LawFirmOffice6,
    LawFirmOffice7,
    LawFirmOffice8,
    
    // FinancialServices offices (15個)
    FinancialServicesOffice1,
    FinancialServicesOffice2,
    FinancialServicesOffice3,
    FinancialServicesOffice4,
    FinancialServicesOffice5,
    FinancialServicesOffice6,
    FinancialServicesOffice7,
    FinancialServicesOffice8,
    FinancialServicesOffice9,
    FinancialServicesOffice10,
    FinancialServicesOffice11,
    FinancialServicesOffice12,
    FinancialServicesOffice13,
    FinancialServicesOffice14,
    FinancialServicesOffice15,
    
    // HealthcareSystem offices (12個)
    HealthcareSystemOffice1,
    HealthcareSystemOffice2,
    HealthcareSystemOffice3,
    HealthcareSystemOffice4,
    HealthcareSystemOffice5,
    HealthcareSystemOffice6,
    HealthcareSystemOffice7,
    HealthcareSystemOffice8,
    HealthcareSystemOffice9,
    HealthcareSystemOffice10,
    HealthcareSystemOffice11,
    HealthcareSystemOffice12,
    
    // EducationInstitute offices (8個)
    EducationInstituteOffice1,
    EducationInstituteOffice2,
    EducationInstituteOffice3,
    EducationInstituteOffice4,
    EducationInstituteOffice5,
    EducationInstituteOffice6,
    EducationInstituteOffice7,
    EducationInstituteOffice8,
    
    // IctProvider offices (8個)
    IctProviderOffice1,
    IctProviderOffice2,
    IctProviderOffice3,
    IctProviderOffice4,
    IctProviderOffice5,
    IctProviderOffice6,
    IctProviderOffice7,
    IctProviderOffice8,
    
    // Customer tenant offices (1-5個ずつ)
    CarLeaserOffice1,
    CarLeaserOffice2,
    CarLeaserOffice3,
    CarLeaserOffice4,
    CarLeaserOffice5,
    
    PrivateReceiverOffice1,
    PrivateReceiverOffice2,
    PrivateReceiverOffice3,
    PrivateReceiverOffice4,
    PrivateReceiverOffice5,
    
    SmallBusinessOffice1,
    SmallBusinessOffice2,
    SmallBusinessOffice3,
    SmallBusinessOffice4,
    SmallBusinessOffice5,
    
    StartupCompanyOffice1,
    StartupCompanyOffice2,
    StartupCompanyOffice3,
    StartupCompanyOffice4,
    StartupCompanyOffice5,
    
    GovernmentAgencyOffice1,
    GovernmentAgencyOffice2,
    GovernmentAgencyOffice3,
    GovernmentAgencyOffice4,
    GovernmentAgencyOffice5,
    
    NonprofitOffice1,
    NonprofitOffice2,
    NonprofitOffice3,
    NonprofitOffice4,
    NonprofitOffice5,
    
    MediaCompanyOffice1,
    MediaCompanyOffice2,
    MediaCompanyOffice3,
    MediaCompanyOffice4,
    MediaCompanyOffice5,
    
    // None for users without specific office
    None,
}

// ドキュメントタイプを表現（拡張版）
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DocumentType {
    BankingNote,
    TrafficFine,
    SalesOffer,
    Contract,
    Invoice,
    Paycheck,
    LegalDocument,
    TechnicalSpecification,
    FinancialReport,
    AuditReport,
    HrDocument,
    PolicyDocument,
    ProjectPlan,
    MeetingMinutes,
    ComplianceReport,
    RiskAssessment,
    BudgetReport,
    PerformanceReview,
    StrategicPlan,
    MarketAnalysis,
}

// アクションを表現
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    View,
    Search,
    ReadMetaInfo,
    Send,
    Edit,
    Approve,
}

// 比較演算子を表現
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComparisonOperator {
    Contains,          // ] : 左の属性セットが右の属性値を含む
    ContainedIn,       // [ : 左の属性値が右の属性セットに含まれる
    Equals,            // = : 等価関係
    GreaterThan,       // > : より大きい
    LessThan,          // < : より小さい
    GreaterThanOrEqual,// >= : 以上
    LessThanOrEqual,   // <= : 以下
    NotEqual,          // != : 等価でない
}

// 属性値を表現
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AttributeValue {
    Role(Role),
    Position(Position),
    Tenant(Tenant),
    Department(Department),
    Office(Office),
    DocumentType(DocumentType),
    Boolean(bool),
    String(String),
    Set(Vec<String>),
    Number(u64),
}

// 属性名を表現
#[derive(Debug, Clone, PartialEq, Eq)]
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
    // 数値属性
    Experience,
    BudgetAuthority,
    Size,
    AccessCount,
    RetentionPeriod,
    SecurityClearance,
    CustomerTier,
    Region,
    Country,
    TimeZone,
    ContractType,
}

// 条件式を表現
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Condition {
    pub left: AttributeExpression,
    pub operator: ComparisonOperator,
    pub right: AttributeExpression,
}

// 属性表現を表現
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributeExpression {
    AttributeName(AttributeName),
    AttributeValue(AttributeValue),
    ValueSet(Vec<AttributeValue>),
}

// edocumentのユーザー属性を表現（拡張版）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdocumentUserAttribute {
    pub user_id: String,
    pub role: Option<Role>,
    pub position: Option<Position>,
    pub tenant: Option<Tenant>,
    pub department: Option<Department>,
    pub office: Option<Office>,
    pub registered: bool,
    pub projects: HashSet<String>,
    pub supervisor: Option<String>,
    pub supervisee: HashSet<String>,
    pub payrolling_permissions: bool,
    
    // Enhanced attributes
    pub security_clearance: Option<SecurityLevel>,
    pub experience: Option<u32>,  // years of experience
    pub customer_tier: Option<CustomerTier>,
    pub region: Option<Region>,
    pub country: Option<Country>,
    pub city: Option<String>,
    pub time_zone: Option<TimeZone>,
    pub working_hours: Option<String>,  // e.g., "09:00-17:00"
    pub temporary_access: HashSet<String>,  // temporary access to resources
    pub delegated_authority: HashSet<String>,  // users who delegated authority to this user
    pub current_projects: HashSet<String>,
    pub past_projects: HashSet<String>,
    pub certifications: HashSet<String>,
    pub is_active: bool,
    pub last_login: Option<String>,  // date string
    pub contract_type: Option<ContractType>,
    pub budget_authority: Option<u64>,  // budget authority amount
}

// edocumentのリソース属性を表現（拡張版）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdocumentResourceAttribute {
    pub resource_id: String,
    pub document_type: DocumentType,
    pub owner: String,
    pub tenant: Tenant,
    pub department: Department,
    pub office: Office,
    pub recipients: HashSet<String>,
    pub is_confidential: bool,
    pub contains_personal_info: bool,
    
    // Enhanced attributes
    pub security_level: Option<SecurityLevel>,
    pub created_date: Option<String>,
    pub expiry_date: Option<String>,
    pub project_id: Option<String>,
    pub version: Option<String>,
    pub size: Option<u64>,  // KB
    pub format: Option<Format>,
    pub language: Option<Language>,
    pub region: Option<Region>,
    pub country: Option<Country>,
    pub approval_status: Option<ApprovalStatus>,
    pub reviewers: HashSet<String>,
    pub approvers: HashSet<String>,
    pub related_documents: HashSet<String>,
    pub tags: HashSet<String>,
    pub compliance_requirements: HashSet<String>,
    pub retention_period: Option<u32>,  // years
    pub is_archived: bool,
    pub last_modified: Option<String>,
    pub access_count: Option<u32>,
    pub priority: Option<Priority>,
}

// edocumentのルールを表現
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdocumentRule {
    pub id: usize,
    pub description: String,
    pub user_conditions: Vec<Condition>,
    pub resource_conditions: Vec<Condition>,
    pub actions: HashSet<Action>,
    pub comparison_conditions: Vec<Condition>,
}

// パース結果全体を表現
#[derive(Debug, Clone)]
pub struct EdocumentAbacData {
    pub users: Vec<EdocumentUserAttribute>,
    pub resources: Vec<EdocumentResourceAttribute>,
    pub rules: Vec<EdocumentRule>,
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
            registered: false,
            projects: HashSet::new(),
            supervisor: None,
            supervisee: HashSet::new(),
            payrolling_permissions: false,
            // Enhanced attributes
            security_clearance: None,
            experience: None,
            customer_tier: None,
            region: None,
            country: None,
            city: None,
            time_zone: None,
            working_hours: None,
            temporary_access: HashSet::new(),
            delegated_authority: HashSet::new(),
            current_projects: HashSet::new(),
            past_projects: HashSet::new(),
            certifications: HashSet::new(),
            is_active: true,
            last_login: None,
            contract_type: None,
            budget_authority: None,
        }
    }
}

impl EdocumentResourceAttribute {
    pub fn new(resource_id: String, document_type: DocumentType) -> Self {
        Self {
            resource_id,
            document_type,
            owner: String::new(),
            tenant: Tenant::LargeBank,
            department: Department::None,
            office: Office::None,
            recipients: HashSet::new(),
            is_confidential: false,
            contains_personal_info: false,
            // Enhanced attributes
            security_level: None,
            created_date: None,
            expiry_date: None,
            project_id: None,
            version: None,
            size: None,
            format: None,
            language: None,
            region: None,
            country: None,
            approval_status: None,
            reviewers: HashSet::new(),
            approvers: HashSet::new(),
            related_documents: HashSet::new(),
            tags: HashSet::new(),
            compliance_requirements: HashSet::new(),
            retention_period: None,
            is_archived: false,
            last_modified: None,
            access_count: None,
            priority: None,
        }
    }
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

// 文字列からの変換実装
impl std::str::FromStr for Role {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "employee" => Ok(Role::Employee),
            "manager" => Ok(Role::Manager),
            "admin" => Ok(Role::Admin),
            "helpdesk" => Ok(Role::Helpdesk),
            "customer" => Ok(Role::Customer),
            "projectManager" => Ok(Role::ProjectManager),
            "legalOfficer" => Ok(Role::LegalOfficer),
            "financialOfficer" => Ok(Role::FinancialOfficer),
            "auditor" => Ok(Role::Auditor),
            "consultant" => Ok(Role::Consultant),
            _ => Err(format!("Unknown role: {}", s)),
        }
    }
}

impl std::str::FromStr for Position {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "secretary" => Ok(Position::Secretary),
            "director" => Ok(Position::Director),
            "seniorOfficeManager" => Ok(Position::SeniorOfficeManager),
            "officeManager" => Ok(Position::OfficeManager),
            "insuranceAgent" => Ok(Position::InsuranceAgent),
            "analyst" => Ok(Position::Analyst),
            "specialist" => Ok(Position::Specialist),
            "coordinator" => Ok(Position::Coordinator),
            "lead" => Ok(Position::Lead),
            "associate" => Ok(Position::Associate),
            "senior" => Ok(Position::Senior),
            "principal" => Ok(Position::Principal),
            "vicePresident" => Ok(Position::VicePresident),
            "none" => Ok(Position::None),
            _ => Err(format!("Unknown position: {}", s)),
        }
    }
}

impl std::str::FromStr for Tenant {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "largeBank" => Ok(Tenant::LargeBank),
            "largeBankLeasing" => Ok(Tenant::LargeBankLeasing),
            "newsAgency" => Ok(Tenant::NewsAgency),
            "europeRegion" => Ok(Tenant::EuropeRegion),
            "londonOffice" => Ok(Tenant::LondonOffice),
            "reseller" => Ok(Tenant::Reseller),
            "techCorp" => Ok(Tenant::TechCorp),
            "pharmaceuticals" => Ok(Tenant::Pharmaceuticals),
            "energyCompany" => Ok(Tenant::EnergyCompany),
            "retailChain" => Ok(Tenant::RetailChain),
            "consultingFirm" => Ok(Tenant::ConsultingFirm),
            "lawFirm" => Ok(Tenant::LawFirm),
            "financialServices" => Ok(Tenant::FinancialServices),
            "healthcareSystem" => Ok(Tenant::HealthcareSystem),
            "educationInstitute" => Ok(Tenant::EducationInstitute),
            "carLeaser" => Ok(Tenant::CarLeaser),
            "ictProvider" => Ok(Tenant::IctProvider),
            "privateReceiver" => Ok(Tenant::PrivateReceiver),
            "smallBusiness" => Ok(Tenant::SmallBusiness),
            "startupCompany" => Ok(Tenant::StartupCompany),
            "governmentAgency" => Ok(Tenant::GovernmentAgency),
            "nonprofit" => Ok(Tenant::Nonprofit),
            "mediaCompany" => Ok(Tenant::MediaCompany),
            _ => Err(format!("Unknown tenant: {}", s)),
        }
    }
}

impl std::str::FromStr for Department {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // LargeBank departments
            "largeBankSales" => Ok(Department::LargeBankSales),
            "largeBankICT" => Ok(Department::LargeBankICT),
            "largeBankIT" => Ok(Department::LargeBankIT),
            "largeBankHR" => Ok(Department::LargeBankHR),
            "largeBankAudit" => Ok(Department::LargeBankAudit),
            "largeBankLegal" => Ok(Department::LargeBankLegal),
            "largeBankRisk" => Ok(Department::LargeBankRisk),
            "largeBankCompliance" => Ok(Department::LargeBankCompliance),
            // LargeBankLeasing departments
            "largeBankLeasingSales" => Ok(Department::LargeBankLeasingSales),
            "largeBankLeasingCustomerCare" => Ok(Department::LargeBankLeasingCustomerCare),
            "largeBankLeasingRisk" => Ok(Department::LargeBankLeasingRisk),
            "largeBankLeasingOperations" => Ok(Department::LargeBankLeasingOperations),
            // Reseller departments
            "resellerSales" => Ok(Department::ResellerSales),
            "resellerAccounting" => Ok(Department::ResellerAccounting),
            "resellerCustomer" => Ok(Department::ResellerCustomer),
            "resellerSupport" => Ok(Department::ResellerSupport),
            "resellerMarketing" => Ok(Department::ResellerMarketing),
            // NewsAgency departments
            "newsAgencyIT" => Ok(Department::NewsAgencyIT),
            "newsAgencyAudit" => Ok(Department::NewsAgencyAudit),
            "newsAgencyEditorial" => Ok(Department::NewsAgencyEditorial),
            "newsAgencyMarketing" => Ok(Department::NewsAgencyMarketing),
            "newsAgencyLegal" => Ok(Department::NewsAgencyLegal),
            // EuropeRegion departments
            "europeRegionHR" => Ok(Department::EuropeRegionHR),
            "europeRegionIT" => Ok(Department::EuropeRegionIT),
            "europeRegionFinance" => Ok(Department::EuropeRegionFinance),
            "europeRegionStrategy" => Ok(Department::EuropeRegionStrategy),
            "europeRegionCompliance" => Ok(Department::EuropeRegionCompliance),
            // LondonOffice departments
            "londonOfficeAudit" => Ok(Department::LondonOfficeAudit),
            "londonOfficeHR" => Ok(Department::LondonOfficeHR),
            "londonOfficeSales" => Ok(Department::LondonOfficeSales),
            "londonOfficeOperations" => Ok(Department::LondonOfficeOperations),
            "londonOfficeResearch" => Ok(Department::LondonOfficeResearch),
            // TechCorp departments
            "techCorpEngineering" => Ok(Department::TechCorpEngineering),
            "techCorpProduct" => Ok(Department::TechCorpProduct),
            "techCorpSales" => Ok(Department::TechCorpSales),
            "techCorpSupport" => Ok(Department::TechCorpSupport),
            "techCorpResearch" => Ok(Department::TechCorpResearch),
            "techCorpSecurity" => Ok(Department::TechCorpSecurity),
            // Pharmaceuticals departments
            "pharmaceuticalsResearch" => Ok(Department::PharmaceuticalsResearch),
            "pharmaceuticalsRegulatory" => Ok(Department::PharmaceuticalsRegulatory),
            "pharmaceuticalsSales" => Ok(Department::PharmaceuticalsSales),
            "pharmaceuticalsManufacturing" => Ok(Department::PharmaceuticalsManufacturing),
            // EnergyCompany departments
            "energyCompanyExploration" => Ok(Department::EnergyCompanyExploration),
            "energyCompanyProduction" => Ok(Department::EnergyCompanyProduction),
            "energyCompanyRefining" => Ok(Department::EnergyCompanyRefining),
            "energyCompanyTradingAudit" => Ok(Department::EnergyCompanyTradingAudit),
            // RetailChain departments
            "retailChainOperations" => Ok(Department::RetailChainOperations),
            "retailChainMarketing" => Ok(Department::RetailChainMarketing),
            "retailChainSupplyChain" => Ok(Department::RetailChainSupplyChain),
            "retailChainFinance" => Ok(Department::RetailChainFinance),
            // ConsultingFirm departments
            "consultingFirmStrategy" => Ok(Department::ConsultingFirmStrategy),
            "consultingFirmTechnology" => Ok(Department::ConsultingFirmTechnology),
            "consultingFirmOperations" => Ok(Department::ConsultingFirmOperations),
            "consultingFirmHR" => Ok(Department::ConsultingFirmHR),
            // LawFirm departments
            "lawFirmCorporate" => Ok(Department::LawFirmCorporate),
            "lawFirmLitigation" => Ok(Department::LawFirmLitigation),
            "lawFirmIntellectualProperty" => Ok(Department::LawFirmIntellectualProperty),
            "lawFirmTax" => Ok(Department::LawFirmTax),
            "lawFirmEmployment" => Ok(Department::LawFirmEmployment),
            // FinancialServices departments
            "financialServicesWealth" => Ok(Department::FinancialServicesWealth),
            "financialServicesRetail" => Ok(Department::FinancialServicesRetail),
            "financialServicesInvestment" => Ok(Department::FinancialServicesInvestment),
            "financialServicesRisk" => Ok(Department::FinancialServicesRisk),
            // HealthcareSystem departments
            "healthcareSystemClinical" => Ok(Department::HealthcareSystemClinical),
            "healthcareSystemAdministrative" => Ok(Department::HealthcareSystemAdministrative),
            "healthcareSystemResearch" => Ok(Department::HealthcareSystemResearch),
            "healthcareSystemIT" => Ok(Department::HealthcareSystemIT),
            // EducationInstitute departments
            "educationInstituteAcademic" => Ok(Department::EducationInstituteAcademic),
            "educationInstituteAdministrative" => Ok(Department::EducationInstituteAdministrative),
            "educationInstituteResearch" => Ok(Department::EducationInstituteResearch),
            "educationInstituteIT" => Ok(Department::EducationInstituteIT),
            // Customer tenant departments
            "carLeaserAudit" => Ok(Department::CarLeaserAudit),
            "carLeaserSecretary" => Ok(Department::CarLeaserSecretary),
            "carLeaserAccounting" => Ok(Department::CarLeaserAccounting),
            "carLeaserOperations" => Ok(Department::CarLeaserOperations),
            "carLeaserMarketing" => Ok(Department::CarLeaserMarketing),
            "ictProviderAudit" => Ok(Department::IctProviderAudit),
            "ictProviderSecretary" => Ok(Department::IctProviderSecretary),
            "ictProviderAccounting" => Ok(Department::IctProviderAccounting),
            "ictProviderOperations" => Ok(Department::IctProviderOperations),
            "ictProviderMarketing" => Ok(Department::IctProviderMarketing),
            "ictProviderICT" => Ok(Department::IctProviderICT),
            "ictProviderDevelopment" => Ok(Department::IctProviderDevelopment),
            "ictProviderSupport" => Ok(Department::IctProviderSupport),
            "privateReceiverAudit" => Ok(Department::PrivateReceiverAudit),
            "privateReceiverSecretary" => Ok(Department::PrivateReceiverSecretary),
            "privateReceiverAccounting" => Ok(Department::PrivateReceiverAccounting),
            "privateReceiverOperations" => Ok(Department::PrivateReceiverOperations),
            "privateReceiverMarketing" => Ok(Department::PrivateReceiverMarketing),
            // SmallBusiness departments
            "smallBusinessAudit" => Ok(Department::SmallBusinessAudit),
            "smallBusinessSecretary" => Ok(Department::SmallBusinessSecretary),
            "smallBusinessAccounting" => Ok(Department::SmallBusinessAccounting),
            "smallBusinessOperations" => Ok(Department::SmallBusinessOperations),
            "smallBusinessMarketing" => Ok(Department::SmallBusinessMarketing),
            // StartupCompany departments
            "startupCompanyAudit" => Ok(Department::StartupCompanyAudit),
            "startupCompanySecretary" => Ok(Department::StartupCompanySecretary),
            "startupCompanyAccounting" => Ok(Department::StartupCompanyAccounting),
            "startupCompanyOperations" => Ok(Department::StartupCompanyOperations),
            "startupCompanyMarketing" => Ok(Department::StartupCompanyMarketing),
            "startupCompanyProduct" => Ok(Department::StartupCompanyProduct),
            "startupCompanyEngineering" => Ok(Department::StartupCompanyEngineering),
            "startupCompanyBusinessDevelopment" => Ok(Department::StartupCompanyBusinessDevelopment),
            // GovernmentAgency departments
            "governmentAgencyAudit" => Ok(Department::GovernmentAgencyAudit),
            "governmentAgencySecretary" => Ok(Department::GovernmentAgencySecretary),
            "governmentAgencyAccounting" => Ok(Department::GovernmentAgencyAccounting),
            "governmentAgencyOperations" => Ok(Department::GovernmentAgencyOperations),
            "governmentAgencyMarketing" => Ok(Department::GovernmentAgencyMarketing),
            "governmentAgencyPolicy" => Ok(Department::GovernmentAgencyPolicy),
            "governmentAgencyPublicRelations" => Ok(Department::GovernmentAgencyPublicRelations),
            "governmentAgencyCompliance" => Ok(Department::GovernmentAgencyCompliance),
            // Nonprofit departments
            "nonprofitAudit" => Ok(Department::NonprofitAudit),
            "nonprofitSecretary" => Ok(Department::NonprofitSecretary),
            "nonprofitAccounting" => Ok(Department::NonprofitAccounting),
            "nonprofitOperations" => Ok(Department::NonprofitOperations),
            "nonprofitMarketing" => Ok(Department::NonprofitMarketing),
            // MediaCompany departments
            "mediaCompanyAudit" => Ok(Department::MediaCompanyAudit),
            "mediaCompanySecretary" => Ok(Department::MediaCompanySecretary),
            "mediaCompanyAccounting" => Ok(Department::MediaCompanyAccounting),
            "mediaCompanyOperations" => Ok(Department::MediaCompanyOperations),
            "mediaCompanyMarketing" => Ok(Department::MediaCompanyMarketing),
            "none" => Ok(Department::None),
            _ => Err(format!("Unknown department: {}", s)),
        }
    }
}

impl std::str::FromStr for Office {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "largeBankOffice1" => Ok(Office::LargeBankOffice1),
            "largeBankOffice2" => Ok(Office::LargeBankOffice2),
            "largeBankOffice3" => Ok(Office::LargeBankOffice3),
            "largeBankOffice4" => Ok(Office::LargeBankOffice4),
            "largeBankOffice5" => Ok(Office::LargeBankOffice5),
            "largeBankOffice6" => Ok(Office::LargeBankOffice6),
            "largeBankOffice7" => Ok(Office::LargeBankOffice7),
            "largeBankOffice8" => Ok(Office::LargeBankOffice8),
            "largeBankOffice9" => Ok(Office::LargeBankOffice9),
            "largeBankOffice10" => Ok(Office::LargeBankOffice10),
            "largeBankOffice11" => Ok(Office::LargeBankOffice11),
            "largeBankOffice12" => Ok(Office::LargeBankOffice12),
            "largeBankOffice13" => Ok(Office::LargeBankOffice13),
            "largeBankOffice14" => Ok(Office::LargeBankOffice14),
            "largeBankOffice15" => Ok(Office::LargeBankOffice15),
            "largeBankLeasingOffice1" => Ok(Office::LargeBankLeasingOffice1),
            "largeBankLeasingOffice2" => Ok(Office::LargeBankLeasingOffice2),
            "largeBankLeasingOffice3" => Ok(Office::LargeBankLeasingOffice3),
            "largeBankLeasingOffice4" => Ok(Office::LargeBankLeasingOffice4),
            "largeBankLeasingOffice5" => Ok(Office::LargeBankLeasingOffice5),
            "newsAgencyOffice1" => Ok(Office::NewsAgencyOffice1),
            "newsAgencyOffice2" => Ok(Office::NewsAgencyOffice2),
            "newsAgencyOffice3" => Ok(Office::NewsAgencyOffice3),
            "newsAgencyOffice4" => Ok(Office::NewsAgencyOffice4),
            "newsAgencyOffice5" => Ok(Office::NewsAgencyOffice5),
            "newsAgencyOffice6" => Ok(Office::NewsAgencyOffice6),
            "europeRegionOffice1" => Ok(Office::EuropeRegionOffice1),
            "europeRegionOffice2" => Ok(Office::EuropeRegionOffice2),
            "europeRegionOffice3" => Ok(Office::EuropeRegionOffice3),
            "europeRegionOffice4" => Ok(Office::EuropeRegionOffice4),
            "europeRegionOffice5" => Ok(Office::EuropeRegionOffice5),
            "europeRegionOffice6" => Ok(Office::EuropeRegionOffice6),
            "europeRegionOffice7" => Ok(Office::EuropeRegionOffice7),
            "europeRegionOffice8" => Ok(Office::EuropeRegionOffice8),
            "europeRegionOffice9" => Ok(Office::EuropeRegionOffice9),
            "europeRegionOffice10" => Ok(Office::EuropeRegionOffice10),
            "europeRegionOffice11" => Ok(Office::EuropeRegionOffice11),
            "europeRegionOffice12" => Ok(Office::EuropeRegionOffice12),
            "londonOfficeOffice1" => Ok(Office::LondonOfficeOffice1),
            "londonOfficeOffice2" => Ok(Office::LondonOfficeOffice2),
            "londonOfficeOffice3" => Ok(Office::LondonOfficeOffice3),
            "londonOfficeOffice4" => Ok(Office::LondonOfficeOffice4),
            "resellerOffice1" => Ok(Office::ResellerOffice1),
            "resellerOffice2" => Ok(Office::ResellerOffice2),
            "resellerOffice3" => Ok(Office::ResellerOffice3),
            "resellerOffice4" => Ok(Office::ResellerOffice4),
            "resellerOffice5" => Ok(Office::ResellerOffice5),
            "resellerOffice6" => Ok(Office::ResellerOffice6),
            "resellerOffice7" => Ok(Office::ResellerOffice7),
            "techCorpOffice1" => Ok(Office::TechCorpOffice1),
            "techCorpOffice2" => Ok(Office::TechCorpOffice2),
            "techCorpOffice3" => Ok(Office::TechCorpOffice3),
            "techCorpOffice4" => Ok(Office::TechCorpOffice4),
            "techCorpOffice5" => Ok(Office::TechCorpOffice5),
            "techCorpOffice6" => Ok(Office::TechCorpOffice6),
            "techCorpOffice7" => Ok(Office::TechCorpOffice7),
            "techCorpOffice8" => Ok(Office::TechCorpOffice8),
            "techCorpOffice9" => Ok(Office::TechCorpOffice9),
            "techCorpOffice10" => Ok(Office::TechCorpOffice10),
            "techCorpOffice11" => Ok(Office::TechCorpOffice11),
            "techCorpOffice12" => Ok(Office::TechCorpOffice12),
            "techCorpOffice13" => Ok(Office::TechCorpOffice13),
            "techCorpOffice14" => Ok(Office::TechCorpOffice14),
            "techCorpOffice15" => Ok(Office::TechCorpOffice15),
            "techCorpOffice16" => Ok(Office::TechCorpOffice16),
            "techCorpOffice17" => Ok(Office::TechCorpOffice17),
            "techCorpOffice18" => Ok(Office::TechCorpOffice18),
            "techCorpOffice19" => Ok(Office::TechCorpOffice19),
            "techCorpOffice20" => Ok(Office::TechCorpOffice20),
            "pharmaceuticalsOffice1" => Ok(Office::PharmaceuticalsOffice1),
            "pharmaceuticalsOffice2" => Ok(Office::PharmaceuticalsOffice2),
            "pharmaceuticalsOffice3" => Ok(Office::PharmaceuticalsOffice3),
            "pharmaceuticalsOffice4" => Ok(Office::PharmaceuticalsOffice4),
            "pharmaceuticalsOffice5" => Ok(Office::PharmaceuticalsOffice5),
            "pharmaceuticalsOffice6" => Ok(Office::PharmaceuticalsOffice6),
            "pharmaceuticalsOffice7" => Ok(Office::PharmaceuticalsOffice7),
            "pharmaceuticalsOffice8" => Ok(Office::PharmaceuticalsOffice8),
            "pharmaceuticalsOffice9" => Ok(Office::PharmaceuticalsOffice9),
            "pharmaceuticalsOffice10" => Ok(Office::PharmaceuticalsOffice10),
            "pharmaceuticalsOffice11" => Ok(Office::PharmaceuticalsOffice11),
            "pharmaceuticalsOffice12" => Ok(Office::PharmaceuticalsOffice12),
            "pharmaceuticalsOffice13" => Ok(Office::PharmaceuticalsOffice13),
            "pharmaceuticalsOffice14" => Ok(Office::PharmaceuticalsOffice14),
            "pharmaceuticalsOffice15" => Ok(Office::PharmaceuticalsOffice15),
            "pharmaceuticalsOffice16" => Ok(Office::PharmaceuticalsOffice16),
            "pharmaceuticalsOffice17" => Ok(Office::PharmaceuticalsOffice17),
            "pharmaceuticalsOffice18" => Ok(Office::PharmaceuticalsOffice18),
            "energyCompanyOffice1" => Ok(Office::EnergyCompanyOffice1),
            "energyCompanyOffice2" => Ok(Office::EnergyCompanyOffice2),
            "energyCompanyOffice3" => Ok(Office::EnergyCompanyOffice3),
            "energyCompanyOffice4" => Ok(Office::EnergyCompanyOffice4),
            "energyCompanyOffice5" => Ok(Office::EnergyCompanyOffice5),
            "energyCompanyOffice6" => Ok(Office::EnergyCompanyOffice6),
            "energyCompanyOffice7" => Ok(Office::EnergyCompanyOffice7),
            "energyCompanyOffice8" => Ok(Office::EnergyCompanyOffice8),
            "energyCompanyOffice9" => Ok(Office::EnergyCompanyOffice9),
            "energyCompanyOffice10" => Ok(Office::EnergyCompanyOffice10),
            "energyCompanyOffice11" => Ok(Office::EnergyCompanyOffice11),
            "energyCompanyOffice12" => Ok(Office::EnergyCompanyOffice12),
            "energyCompanyOffice13" => Ok(Office::EnergyCompanyOffice13),
            "energyCompanyOffice14" => Ok(Office::EnergyCompanyOffice14),
            "energyCompanyOffice15" => Ok(Office::EnergyCompanyOffice15),
            "energyCompanyOffice16" => Ok(Office::EnergyCompanyOffice16),
            "energyCompanyOffice17" => Ok(Office::EnergyCompanyOffice17),
            "energyCompanyOffice18" => Ok(Office::EnergyCompanyOffice18),
            "energyCompanyOffice19" => Ok(Office::EnergyCompanyOffice19),
            "energyCompanyOffice20" => Ok(Office::EnergyCompanyOffice20),
            "energyCompanyOffice21" => Ok(Office::EnergyCompanyOffice21),
            "energyCompanyOffice22" => Ok(Office::EnergyCompanyOffice22),
            "energyCompanyOffice23" => Ok(Office::EnergyCompanyOffice23),
            "energyCompanyOffice24" => Ok(Office::EnergyCompanyOffice24),
            "energyCompanyOffice25" => Ok(Office::EnergyCompanyOffice25),
            "retailChainOffice1" => Ok(Office::RetailChainOffice1),
            "retailChainOffice2" => Ok(Office::RetailChainOffice2),
            "retailChainOffice3" => Ok(Office::RetailChainOffice3),
            "retailChainOffice4" => Ok(Office::RetailChainOffice4),
            "retailChainOffice5" => Ok(Office::RetailChainOffice5),
            "retailChainOffice6" => Ok(Office::RetailChainOffice6),
            "retailChainOffice7" => Ok(Office::RetailChainOffice7),
            "retailChainOffice8" => Ok(Office::RetailChainOffice8),
            "retailChainOffice9" => Ok(Office::RetailChainOffice9),
            "retailChainOffice10" => Ok(Office::RetailChainOffice10),
            "retailChainOffice11" => Ok(Office::RetailChainOffice11),
            "retailChainOffice12" => Ok(Office::RetailChainOffice12),
            "retailChainOffice13" => Ok(Office::RetailChainOffice13),
            "retailChainOffice14" => Ok(Office::RetailChainOffice14),
            "retailChainOffice15" => Ok(Office::RetailChainOffice15),
            "retailChainOffice16" => Ok(Office::RetailChainOffice16),
            "retailChainOffice17" => Ok(Office::RetailChainOffice17),
            "retailChainOffice18" => Ok(Office::RetailChainOffice18),
            "retailChainOffice19" => Ok(Office::RetailChainOffice19),
            "retailChainOffice20" => Ok(Office::RetailChainOffice20),
            "retailChainOffice21" => Ok(Office::RetailChainOffice21),
            "retailChainOffice22" => Ok(Office::RetailChainOffice22),
            "retailChainOffice23" => Ok(Office::RetailChainOffice23),
            "retailChainOffice24" => Ok(Office::RetailChainOffice24),
            "retailChainOffice25" => Ok(Office::RetailChainOffice25),
            "retailChainOffice26" => Ok(Office::RetailChainOffice26),
            "retailChainOffice27" => Ok(Office::RetailChainOffice27),
            "retailChainOffice28" => Ok(Office::RetailChainOffice28),
            "retailChainOffice29" => Ok(Office::RetailChainOffice29),
            "retailChainOffice30" => Ok(Office::RetailChainOffice30),
            "retailChainOffice31" => Ok(Office::RetailChainOffice31),
            "retailChainOffice32" => Ok(Office::RetailChainOffice32),
            "retailChainOffice33" => Ok(Office::RetailChainOffice33),
            "retailChainOffice34" => Ok(Office::RetailChainOffice34),
            "retailChainOffice35" => Ok(Office::RetailChainOffice35),
            "retailChainOffice36" => Ok(Office::RetailChainOffice36),
            "retailChainOffice37" => Ok(Office::RetailChainOffice37),
            "retailChainOffice38" => Ok(Office::RetailChainOffice38),
            "retailChainOffice39" => Ok(Office::RetailChainOffice39),
            "retailChainOffice40" => Ok(Office::RetailChainOffice40),
            "retailChainOffice41" => Ok(Office::RetailChainOffice41),
            "retailChainOffice42" => Ok(Office::RetailChainOffice42),
            "retailChainOffice43" => Ok(Office::RetailChainOffice43),
            "retailChainOffice44" => Ok(Office::RetailChainOffice44),
            "retailChainOffice45" => Ok(Office::RetailChainOffice45),
            "retailChainOffice46" => Ok(Office::RetailChainOffice46),
            "retailChainOffice47" => Ok(Office::RetailChainOffice47),
            "retailChainOffice48" => Ok(Office::RetailChainOffice48),
            "retailChainOffice49" => Ok(Office::RetailChainOffice49),
            "retailChainOffice50" => Ok(Office::RetailChainOffice50),
            "consultingFirmOffice1" => Ok(Office::ConsultingFirmOffice1),
            "consultingFirmOffice2" => Ok(Office::ConsultingFirmOffice2),
            "consultingFirmOffice3" => Ok(Office::ConsultingFirmOffice3),
            "consultingFirmOffice4" => Ok(Office::ConsultingFirmOffice4),
            "consultingFirmOffice5" => Ok(Office::ConsultingFirmOffice5),
            "consultingFirmOffice6" => Ok(Office::ConsultingFirmOffice6),
            "consultingFirmOffice7" => Ok(Office::ConsultingFirmOffice7),
            "consultingFirmOffice8" => Ok(Office::ConsultingFirmOffice8),
            "consultingFirmOffice9" => Ok(Office::ConsultingFirmOffice9),
            "consultingFirmOffice10" => Ok(Office::ConsultingFirmOffice10),
            "lawFirmOffice1" => Ok(Office::LawFirmOffice1),
            "lawFirmOffice2" => Ok(Office::LawFirmOffice2),
            "lawFirmOffice3" => Ok(Office::LawFirmOffice3),
            "lawFirmOffice4" => Ok(Office::LawFirmOffice4),
            "lawFirmOffice5" => Ok(Office::LawFirmOffice5),
            "lawFirmOffice6" => Ok(Office::LawFirmOffice6),
            "lawFirmOffice7" => Ok(Office::LawFirmOffice7),
            "lawFirmOffice8" => Ok(Office::LawFirmOffice8),
            "financialServicesOffice1" => Ok(Office::FinancialServicesOffice1),
            "financialServicesOffice2" => Ok(Office::FinancialServicesOffice2),
            "financialServicesOffice3" => Ok(Office::FinancialServicesOffice3),
            "financialServicesOffice4" => Ok(Office::FinancialServicesOffice4),
            "financialServicesOffice5" => Ok(Office::FinancialServicesOffice5),
            "financialServicesOffice6" => Ok(Office::FinancialServicesOffice6),
            "financialServicesOffice7" => Ok(Office::FinancialServicesOffice7),
            "financialServicesOffice8" => Ok(Office::FinancialServicesOffice8),
            "financialServicesOffice9" => Ok(Office::FinancialServicesOffice9),
            "financialServicesOffice10" => Ok(Office::FinancialServicesOffice10),
            "financialServicesOffice11" => Ok(Office::FinancialServicesOffice11),
            "financialServicesOffice12" => Ok(Office::FinancialServicesOffice12),
            "financialServicesOffice13" => Ok(Office::FinancialServicesOffice13),
            "financialServicesOffice14" => Ok(Office::FinancialServicesOffice14),
            "financialServicesOffice15" => Ok(Office::FinancialServicesOffice15),
            "healthcareSystemOffice1" => Ok(Office::HealthcareSystemOffice1),
            "healthcareSystemOffice2" => Ok(Office::HealthcareSystemOffice2),
            "healthcareSystemOffice3" => Ok(Office::HealthcareSystemOffice3),
            "healthcareSystemOffice4" => Ok(Office::HealthcareSystemOffice4),
            "healthcareSystemOffice5" => Ok(Office::HealthcareSystemOffice5),
            "healthcareSystemOffice6" => Ok(Office::HealthcareSystemOffice6),
            "healthcareSystemOffice7" => Ok(Office::HealthcareSystemOffice7),
            "healthcareSystemOffice8" => Ok(Office::HealthcareSystemOffice8),
            "healthcareSystemOffice9" => Ok(Office::HealthcareSystemOffice9),
            "healthcareSystemOffice10" => Ok(Office::HealthcareSystemOffice10),
            "healthcareSystemOffice11" => Ok(Office::HealthcareSystemOffice11),
            "healthcareSystemOffice12" => Ok(Office::HealthcareSystemOffice12),
            "educationInstituteOffice1" => Ok(Office::EducationInstituteOffice1),
            "educationInstituteOffice2" => Ok(Office::EducationInstituteOffice2),
            "educationInstituteOffice3" => Ok(Office::EducationInstituteOffice3),
            "educationInstituteOffice4" => Ok(Office::EducationInstituteOffice4),
            "educationInstituteOffice5" => Ok(Office::EducationInstituteOffice5),
            "educationInstituteOffice6" => Ok(Office::EducationInstituteOffice6),
            "educationInstituteOffice7" => Ok(Office::EducationInstituteOffice7),
            "educationInstituteOffice8" => Ok(Office::EducationInstituteOffice8),
            "ictProviderOffice1" => Ok(Office::IctProviderOffice1),
            "ictProviderOffice2" => Ok(Office::IctProviderOffice2),
            "ictProviderOffice3" => Ok(Office::IctProviderOffice3),
            "ictProviderOffice4" => Ok(Office::IctProviderOffice4),
            "ictProviderOffice5" => Ok(Office::IctProviderOffice5),
            "ictProviderOffice6" => Ok(Office::IctProviderOffice6),
            "ictProviderOffice7" => Ok(Office::IctProviderOffice7),
            "ictProviderOffice8" => Ok(Office::IctProviderOffice8),
            "carLeaserOffice1" => Ok(Office::CarLeaserOffice1),
            "carLeaserOffice2" => Ok(Office::CarLeaserOffice2),
            "carLeaserOffice3" => Ok(Office::CarLeaserOffice3),
            "carLeaserOffice4" => Ok(Office::CarLeaserOffice4),
            "carLeaserOffice5" => Ok(Office::CarLeaserOffice5),
            "privateReceiverOffice1" => Ok(Office::PrivateReceiverOffice1),
            "privateReceiverOffice2" => Ok(Office::PrivateReceiverOffice2),
            "privateReceiverOffice3" => Ok(Office::PrivateReceiverOffice3),
            "privateReceiverOffice4" => Ok(Office::PrivateReceiverOffice4),
            "privateReceiverOffice5" => Ok(Office::PrivateReceiverOffice5),
            "smallBusinessOffice1" => Ok(Office::SmallBusinessOffice1),
            "smallBusinessOffice2" => Ok(Office::SmallBusinessOffice2),
            "smallBusinessOffice3" => Ok(Office::SmallBusinessOffice3),
            "smallBusinessOffice4" => Ok(Office::SmallBusinessOffice4),
            "smallBusinessOffice5" => Ok(Office::SmallBusinessOffice5),
            "startupCompanyOffice1" => Ok(Office::StartupCompanyOffice1),
            "startupCompanyOffice2" => Ok(Office::StartupCompanyOffice2),
            "startupCompanyOffice3" => Ok(Office::StartupCompanyOffice3),
            "startupCompanyOffice4" => Ok(Office::StartupCompanyOffice4),
            "startupCompanyOffice5" => Ok(Office::StartupCompanyOffice5),
            "governmentAgencyOffice1" => Ok(Office::GovernmentAgencyOffice1),
            "governmentAgencyOffice2" => Ok(Office::GovernmentAgencyOffice2),
            "governmentAgencyOffice3" => Ok(Office::GovernmentAgencyOffice3),
            "governmentAgencyOffice4" => Ok(Office::GovernmentAgencyOffice4),
            "governmentAgencyOffice5" => Ok(Office::GovernmentAgencyOffice5),
            "nonprofitOffice1" => Ok(Office::NonprofitOffice1),
            "nonprofitOffice2" => Ok(Office::NonprofitOffice2),
            "nonprofitOffice3" => Ok(Office::NonprofitOffice3),
            "nonprofitOffice4" => Ok(Office::NonprofitOffice4),
            "nonprofitOffice5" => Ok(Office::NonprofitOffice5),
            "mediaCompanyOffice1" => Ok(Office::MediaCompanyOffice1),
            "mediaCompanyOffice2" => Ok(Office::MediaCompanyOffice2),
            "mediaCompanyOffice3" => Ok(Office::MediaCompanyOffice3),
            "mediaCompanyOffice4" => Ok(Office::MediaCompanyOffice4),
            "mediaCompanyOffice5" => Ok(Office::MediaCompanyOffice5),
            "none" => Ok(Office::None),
            _ => Err(format!("Unknown office: {}", s)),
        }
    }
}

impl std::str::FromStr for DocumentType {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bankingNote" => Ok(DocumentType::BankingNote),
            "trafficFine" => Ok(DocumentType::TrafficFine),
            "salesOffer" => Ok(DocumentType::SalesOffer),
            "contract" => Ok(DocumentType::Contract),
            "invoice" => Ok(DocumentType::Invoice),
            "paycheck" => Ok(DocumentType::Paycheck),
            "legalDocument" => Ok(DocumentType::LegalDocument),
            "technicalSpecification" => Ok(DocumentType::TechnicalSpecification),
            "financialReport" => Ok(DocumentType::FinancialReport),
            "auditReport" => Ok(DocumentType::AuditReport),
            "hrDocument" => Ok(DocumentType::HrDocument),
            "policyDocument" => Ok(DocumentType::PolicyDocument),
            "projectPlan" => Ok(DocumentType::ProjectPlan),
            "meetingMinutes" => Ok(DocumentType::MeetingMinutes),
            "complianceReport" => Ok(DocumentType::ComplianceReport),
            "riskAssessment" => Ok(DocumentType::RiskAssessment),
            "budgetReport" => Ok(DocumentType::BudgetReport),
            "performanceReview" => Ok(DocumentType::PerformanceReview),
            "strategicPlan" => Ok(DocumentType::StrategicPlan),
            "marketAnalysis" => Ok(DocumentType::MarketAnalysis),
            _ => Err(format!("Unknown document type: {}", s)),
        }
    }
}

impl std::str::FromStr for Action {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "view" => Ok(Action::View),
            "search" => Ok(Action::Search),
            "readMetaInfo" => Ok(Action::ReadMetaInfo),
            "send" => Ok(Action::Send),
            "edit" => Ok(Action::Edit),
            "approve" => Ok(Action::Approve),
            _ => Err(format!("Unknown action: {}", s)),
        }
    }
}

impl std::str::FromStr for AttributeName {
    type Err = String;
    
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
            // 数値属性
            "experience" => Ok(AttributeName::Experience),
            "budgetAuthority" => Ok(AttributeName::BudgetAuthority),
            "size" => Ok(AttributeName::Size),
            "accessCount" => Ok(AttributeName::AccessCount),
            "retentionPeriod" => Ok(AttributeName::RetentionPeriod),
            "securityClearance" => Ok(AttributeName::SecurityClearance),
            "customerTier" => Ok(AttributeName::CustomerTier),
            "region" => Ok(AttributeName::Region),
            "country" => Ok(AttributeName::Country),
            "timeZone" => Ok(AttributeName::TimeZone),
            "contractType" => Ok(AttributeName::ContractType),
            _ => Err(format!("Unknown attribute name: {}", s)),
        }
    }
}

impl std::str::FromStr for ComparisonOperator {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "]" => Ok(ComparisonOperator::Contains),
            "[" => Ok(ComparisonOperator::ContainedIn),
            "=" => Ok(ComparisonOperator::Equals),
            ">" => Ok(ComparisonOperator::GreaterThan),
            "<" => Ok(ComparisonOperator::LessThan),
            ">=" => Ok(ComparisonOperator::GreaterThanOrEqual),
            "<=" => Ok(ComparisonOperator::LessThanOrEqual),
            "!=" => Ok(ComparisonOperator::NotEqual),
            _ => Err(format!("Unknown comparison operator: {}", s)),
        }
    }
} 

impl std::str::FromStr for SecurityLevel {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "public" => Ok(SecurityLevel::Public),
            "internal" => Ok(SecurityLevel::Internal),
            "confidential" => Ok(SecurityLevel::Confidential),
            "secret" => Ok(SecurityLevel::Secret),
            "topSecret" => Ok(SecurityLevel::TopSecret),
            _ => Err(format!("Unknown security level: {}", s)),
        }
    }
}

impl std::str::FromStr for CustomerTier {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bronze" => Ok(CustomerTier::Bronze),
            "silver" => Ok(CustomerTier::Silver),
            "gold" => Ok(CustomerTier::Gold),
            "platinum" => Ok(CustomerTier::Platinum),
            "vip" => Ok(CustomerTier::Vip),
            "none" => Ok(CustomerTier::None),
            _ => Err(format!("Unknown customer tier: {}", s)),
        }
    }
}

impl std::str::FromStr for Region {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NorthAmerica" => Ok(Region::NorthAmerica),
            "Europe" => Ok(Region::Europe),
            "Asia" => Ok(Region::Asia),
            "LatinAmerica" => Ok(Region::LatinAmerica),
            "Africa" => Ok(Region::Africa),
            _ => Err(format!("Unknown region: {}", s)),
        }
    }
}

impl std::str::FromStr for Country {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // North America
            "USA" => Ok(Country::USA),
            "Canada" => Ok(Country::Canada),
            "Mexico" => Ok(Country::Mexico),
            // Europe
            "UK" => Ok(Country::UK),
            "Germany" => Ok(Country::Germany),
            "France" => Ok(Country::France),
            "Netherlands" => Ok(Country::Netherlands),
            "Spain" => Ok(Country::Spain),
            // Asia
            "Japan" => Ok(Country::Japan),
            "China" => Ok(Country::China),
            "India" => Ok(Country::India),
            "Singapore" => Ok(Country::Singapore),
            "SouthKorea" => Ok(Country::SouthKorea),
            // Latin America
            "Brazil" => Ok(Country::Brazil),
            "Argentina" => Ok(Country::Argentina),
            "Chile" => Ok(Country::Chile),
            "Colombia" => Ok(Country::Colombia),
            // Africa
            "SouthAfrica" => Ok(Country::SouthAfrica),
            "Nigeria" => Ok(Country::Nigeria),
            "Egypt" => Ok(Country::Egypt),
            "Kenya" => Ok(Country::Kenya),
            _ => Err(format!("Unknown country: {}", s)),
        }
    }
}

impl std::str::FromStr for TimeZone {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UTC" => Ok(TimeZone::UTC),
            "EST" => Ok(TimeZone::EST),
            "PST" => Ok(TimeZone::PST),
            "GMT" => Ok(TimeZone::GMT),
            "CET" => Ok(TimeZone::CET),
            "JST" => Ok(TimeZone::JST),
            "IST" => Ok(TimeZone::IST),
            _ => Err(format!("Unknown time zone: {}", s)),
        }
    }
}

impl std::str::FromStr for ContractType {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "permanent" => Ok(ContractType::Permanent),
            "temporary" => Ok(ContractType::Temporary),
            "contractor" => Ok(ContractType::Contractor),
            "consultant" => Ok(ContractType::Consultant),
            _ => Err(format!("Unknown contract type: {}", s)),
        }
    }
}

impl std::str::FromStr for ApprovalStatus {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "draft" => Ok(ApprovalStatus::Draft),
            "pending" => Ok(ApprovalStatus::Pending),
            "approved" => Ok(ApprovalStatus::Approved),
            "rejected" => Ok(ApprovalStatus::Rejected),
            "archived" => Ok(ApprovalStatus::Archived),
            _ => Err(format!("Unknown approval status: {}", s)),
        }
    }
}

impl std::str::FromStr for Priority {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "low" => Ok(Priority::Low),
            "medium" => Ok(Priority::Medium),
            "high" => Ok(Priority::High),
            "critical" => Ok(Priority::Critical),
            _ => Err(format!("Unknown priority: {}", s)),
        }
    }
}

impl std::str::FromStr for Format {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pdf" => Ok(Format::Pdf),
            "docx" => Ok(Format::Docx),
            "xlsx" => Ok(Format::Xlsx),
            "txt" => Ok(Format::Txt),
            "pptx" => Ok(Format::Pptx),
            "xml" => Ok(Format::Xml),
            "json" => Ok(Format::Json),
            _ => Err(format!("Unknown format: {}", s)),
        }
    }
}

impl std::str::FromStr for Language {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "en" => Ok(Language::En),
            "es" => Ok(Language::Es),
            "fr" => Ok(Language::Fr),
            "de" => Ok(Language::De),
            "ja" => Ok(Language::Ja),
            "zh" => Ok(Language::Zh),
            "pt" => Ok(Language::Pt),
            _ => Err(format!("Unknown language: {}", s)),
        }
    }
} 