use std::collections::HashSet;

// ユーザーの役職を表現
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Position {
    Applicant,
    Student,
    Faculty,
    Staff,
}

// 部署を表現
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Department {
    Cs,
    Ee,
    Registrar,
    Admissions,
}

// コース名を表現
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Course {
    Cs101,
    Cs601,
    Cs602,
    Ee101,
    Ee601,
    Ee602,
}

// リソースタイプを表現
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Application,
    Gradebook,
    Roster,
    Transcript,
}

// アクション（権限）を表現
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    ReadMyScores,
    AddScore,
    ReadScore,
    ChangeScore,
    AssignGrade,
    Read,
    Write,
    CheckStatus,
    SetStatus,
}

// 比較演算子を表現
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComparisonOperator {
    Contains,     // ] : 左の属性セットが右の属性値を含む
    ContainedIn,  // [ : 左の属性値が右の属性セットに含まれる
    Equals,       // = : 等価関係
}

// 属性値を表現
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AttributeValue {
    Position(Position),
    Department(Department),
    Course(Course),
    ResourceType(ResourceType),
    Boolean(bool),
    String(String),
    Set(Vec<String>),
}

// 属性名を表現
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributeName {
    Position,
    Department,
    Type,
    CrsTaken,
    CrsTaught,
    IsChair,
    Student,
    Departments,
    Crs,
    Uid,
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

// 大学のユーザー属性を表現
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UniversityUserAttribute {
    pub user_id: String,
    pub position: Option<Position>,
    pub department: Option<Department>,
    pub crs_taken: HashSet<Course>,
    pub crs_taught: HashSet<Course>,
    pub is_chair: Option<bool>,
}

// 大学のリソース属性を表現
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UniversityResourceAttribute {
    pub resource_id: String,
    pub resource_type: ResourceType,
    pub student: Option<String>,
    pub departments: HashSet<Department>,
    pub crs: Option<Course>,
}

// 大学のルールを表現（改良版）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UniversityRule {
    pub id: usize,
    pub description: String,
    pub user_conditions: Vec<Condition>,      // ユーザー条件
    pub resource_conditions: Vec<Condition>,  // リソース条件
    pub actions: HashSet<Action>,             // アクション
    pub comparison_conditions: Vec<Condition>, // 比較条件
}

// パース結果全体を表現
#[derive(Debug, Clone)]
pub struct UniversityAbacData {
    pub users: Vec<UniversityUserAttribute>,
    pub resources: Vec<UniversityResourceAttribute>,
    pub rules: Vec<UniversityRule>,
}

impl UniversityUserAttribute {
    pub fn new(user_id: String) -> Self {
        Self {
            user_id,
            position: None,
            department: None,
            crs_taken: HashSet::new(),
            crs_taught: HashSet::new(),
            is_chair: None,
        }
    }
}

impl UniversityResourceAttribute {
    pub fn new(resource_id: String, resource_type: ResourceType) -> Self {
        Self {
            resource_id,
            resource_type,
            student: None,
            departments: HashSet::new(),
            crs: None,
        }
    }
}

impl UniversityRule {
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

// 属性名の文字列からの変換
impl std::str::FromStr for AttributeName {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "position" => Ok(AttributeName::Position),
            "department" => Ok(AttributeName::Department),
            "type" => Ok(AttributeName::Type),
            "crsTaken" => Ok(AttributeName::CrsTaken),
            "crsTaught" => Ok(AttributeName::CrsTaught),
            "isChair" => Ok(AttributeName::IsChair),
            "student" => Ok(AttributeName::Student),
            "departments" => Ok(AttributeName::Departments),
            "crs" => Ok(AttributeName::Crs),
            "uid" => Ok(AttributeName::Uid),
            _ => Err(format!("Unknown attribute name: {}", s)),
        }
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
