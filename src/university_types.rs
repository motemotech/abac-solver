use std::collections::HashSet;
use std::str::FromStr;
use serde::{Serialize, Deserialize};
// 再エクスポートして他のモジュールから使えるようにする
pub use crate::types::{ComparisonOperator, AttributeValueExtractor, UserAttribute, ResourceAttribute, Condition, AbacData, ParseError, DomainParser, GenericAbacParser};

// ユーザーの役職を表現
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Position {
    Applicant,
    Student,
    Faculty,
    Staff,
}

// 部署を表現
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Department {
    Cs,
    Ee,
    Registrar,
    Admissions,
}

// コース名を表現
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Course {
    Cs101,
    Cs601,
    Cs602,
    Ee101,
    Ee601,
    Ee602,
}

// リソースタイプを表現
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    Application,
    Gradebook,
    Roster,
    Transcript,
}

// アクション（権限）を表現
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

// 属性値を表現
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttributeValue {
    Position(Position),
    Department(Department),
    Course(Course),
    ResourceType(ResourceType),
    Boolean(bool),
    String(String),
    Set(Vec<String>),
}

impl PartialOrd for AttributeValue {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        None // No natural ordering for university attributes
    }
}

impl Ord for AttributeValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

// 属性名を表現
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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



pub type AttributeExpression = crate::types::AttributeExpression<AttributeName, AttributeValue>;

// 大学のユーザー属性を表現
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UniversityUserAttribute {
    pub user_id: String,
    pub position: Option<Position>,
    pub department: Option<Department>,
    pub crs_taken: HashSet<Course>,
    pub crs_taught: HashSet<Course>,
    pub is_chair: Option<bool>,
}

// 大学のリソース属性を表現
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UniversityResourceAttribute {
    pub resource_id: String,
    pub resource_type: ResourceType,
    pub student: Option<String>,
    pub departments: HashSet<Department>,
    pub crs: Option<Course>,
}

// 大学のルールを表現（改良版）
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UniversityRule {
    pub id: usize,
    pub description: String,
    pub user_conditions: Vec<Condition<AttributeExpression>>,      // ユーザー条件
    pub resource_conditions: Vec<Condition<AttributeExpression>>,  // リソース条件
    pub actions: HashSet<Action>,             // アクション
    pub comparison_conditions: Vec<Condition<AttributeExpression>>, // 比較条件
}

// パース結果全体を表現
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversityAbacData {
    pub users: Vec<UniversityUserAttribute>,
    pub resources: Vec<UniversityResourceAttribute>,
    pub rules: Vec<UniversityRule>,
}

// University specific type alias
pub type UniversityAbac = AbacData<UniversityUserAttribute, UniversityResourceAttribute, UniversityRule>;

impl From<UniversityAbacData> for UniversityAbac {
    fn from(data: UniversityAbacData) -> Self {
        Self {
            users: data.users,
            resources: data.resources,
            rules: data.rules,
        }
    }
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

impl AttributeValueExtractor for UniversityUserAttribute {
    type AttributeName = AttributeName;
    type AttributeValue = AttributeValue;

    fn get_attribute_value(&self, attr_name: &AttributeName) -> Option<AttributeValue> {
        match attr_name {
            AttributeName::Position => {
                self.position.as_ref().map(|p| AttributeValue::Position(p.clone()))
            },
            AttributeName::Department => {
                self.department.as_ref().map(|d| AttributeValue::Department(d.clone()))
            },
            AttributeName::IsChair => {
                self.is_chair.map(|ic| AttributeValue::Boolean(ic))
            },
            AttributeName::Uid => {
                Some(AttributeValue::String(self.user_id.clone()))
            },
            _ => None,
        }
    }

    fn get_attribute_set(&self, attr_name: &AttributeName) -> Option<Vec<AttributeValue>> {
        match attr_name {
            AttributeName::CrsTaken => {
                Some(self.crs_taken.iter()
                    .map(|course| AttributeValue::Course(course.clone()))
                    .collect())
            },
            AttributeName::CrsTaught => {
                Some(self.crs_taught.iter()
                    .map(|course| AttributeValue::Course(course.clone()))
                    .collect())
            },
            _ => None,
        }
    }
}

impl UserAttribute for UniversityUserAttribute {
    fn get_user_id(&self) -> String {
        self.user_id.clone()
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

impl AttributeValueExtractor for UniversityResourceAttribute {
    type AttributeName = AttributeName;
    type AttributeValue = AttributeValue;

    fn get_attribute_value(&self, attr_name: &AttributeName) -> Option<AttributeValue> {
        match attr_name {
            AttributeName::Type => {
                Some(AttributeValue::ResourceType(self.resource_type.clone()))
            },
            AttributeName::Student => {
                self.student.as_ref().map(|s| AttributeValue::String(s.clone()))
            },
            AttributeName::Crs => {
                self.crs.as_ref().map(|c| AttributeValue::Course(c.clone()))
            },
            _ => None,
        }
    }

    fn get_attribute_set(&self, attr_name: &AttributeName) -> Option<Vec<AttributeValue>> {
        match attr_name {
            AttributeName::Departments => {
                Some(self.departments.iter()
                    .map(|dept| AttributeValue::Department(dept.clone()))
                    .collect())
            },
            _ => None,
        }
    }
}

impl ResourceAttribute for UniversityResourceAttribute {
    fn get_resource_id(&self) -> String {
        self.resource_id.clone()
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

// 大学ドメイン固有のパーサー
pub struct UniversityDomainParser;

impl DomainParser for UniversityDomainParser {
    type UserAttribute = UniversityUserAttribute;
    type ResourceAttribute = UniversityResourceAttribute;
    type Rule = UniversityRule;
    type AttributeExpression = AttributeExpression;
    type AttributeName = AttributeName;
    type AttributeValue = AttributeValue;
    type Action = Action;

    fn parse_attribute_name(&self, s: &str) -> Result<AttributeName, ParseError> {
        AttributeName::from_str(s)
    }

    fn parse_attribute_value(&self, s: &str) -> Result<AttributeValue, ParseError> {
        // boolean値のチェック
        if s == "True" || s == "true" || s == "False" || s == "false" {
            return Ok(AttributeValue::Boolean(self.parse_boolean(s)?));
        }

        // Position, Department, Course, ResourceTypeのチェック
        if let Ok(pos) = self.parse_position(s) {
            return Ok(AttributeValue::Position(pos));
        }
        if let Ok(dept) = self.parse_department(s) {
            return Ok(AttributeValue::Department(dept));
        }
        if let Ok(course) = self.parse_course(s) {
            return Ok(AttributeValue::Course(course));
        }
        if let Ok(rt) = self.parse_resource_type(s) {
            return Ok(AttributeValue::ResourceType(rt));
        }

        // 文字列として扱う
        Ok(AttributeValue::String(s.to_string()))
    }

    fn parse_action(&self, s: &str) -> Result<Action, ParseError> {
        match s {
            "readMyScores" => Ok(Action::ReadMyScores),
            "addScore" => Ok(Action::AddScore),
            "readScore" => Ok(Action::ReadScore),
            "changeScore" => Ok(Action::ChangeScore),
            "assignGrade" => Ok(Action::AssignGrade),
            "read" => Ok(Action::Read),
            "write" => Ok(Action::Write),
            "checkStatus" => Ok(Action::CheckStatus),
            "setStatus" => Ok(Action::SetStatus),
            _ => Err(ParseError::UnknownValue("action".to_string(), s.to_string())),
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
        user: &mut UniversityUserAttribute, 
        key: &str, 
        value: &str
    ) -> Result<(), ParseError> {
        match key {
            "position" => {
                user.position = Some(self.parse_position(value)?);
            }
            "department" => {
                user.department = Some(self.parse_department(value)?);
            }
            "crsTaken" => {
                user.crs_taken = self.parse_course_set(value)?;
            }
            "crsTaught" => {
                user.crs_taught = self.parse_course_set(value)?;
            }
            "isChair" => {
                user.is_chair = Some(self.parse_boolean(value)?);
            }
            _ => {
                // 未知の属性は無視
            }
        }
        Ok(())
    }

    fn parse_resource_attribute_field(
        &self, 
        resource: &mut UniversityResourceAttribute, 
        key: &str, 
        value: &str
    ) -> Result<(), ParseError> {
        match key {
            "student" => {
                resource.student = Some(value.to_string());
            }
            "departments" => {
                resource.departments = self.parse_department_set(value)?;
            }
            "crs" => {
                resource.crs = Some(self.parse_course(value)?);
            }
            _ => {
                // 未知の属性は無視
            }
        }
        Ok(())
    }

    fn create_user_attribute(&self, user_id: String) -> UniversityUserAttribute {
        UniversityUserAttribute::new(user_id)
    }

    fn create_resource_attribute(&self, resource_id: String, resource_type: &str) -> Result<UniversityResourceAttribute, ParseError> {
        let rt = self.parse_resource_type(resource_type)?;
        Ok(UniversityResourceAttribute::new(resource_id, rt))
    }

    fn create_rule(
        &self,
        id: usize,
        user_conditions: Vec<Condition<AttributeExpression>>,
        resource_conditions: Vec<Condition<AttributeExpression>>,
        actions: HashSet<Action>,
        comparison_conditions: Vec<Condition<AttributeExpression>>,
    ) -> UniversityRule {
        UniversityRule {
            id,
            description: format!("Rule {}", id + 1),
            user_conditions,
            resource_conditions,
            actions,
            comparison_conditions,
        }
    }
}

impl UniversityDomainParser {
    fn parse_position(&self, value: &str) -> Result<Position, ParseError> {
        match value {
            "applicant" => Ok(Position::Applicant),
            "student" => Ok(Position::Student),
            "faculty" => Ok(Position::Faculty),
            "staff" => Ok(Position::Staff),
            _ => Err(ParseError::UnknownValue("position".to_string(), value.to_string())),
        }
    }

    fn parse_department(&self, value: &str) -> Result<Department, ParseError> {
        match value {
            "cs" => Ok(Department::Cs),
            "ee" => Ok(Department::Ee),
            "registrar" => Ok(Department::Registrar),
            "admissions" => Ok(Department::Admissions),
            _ => Err(ParseError::UnknownValue("department".to_string(), value.to_string())),
        }
    }

    fn parse_course(&self, value: &str) -> Result<Course, ParseError> {
        match value {
            "cs101" => Ok(Course::Cs101),
            "cs601" => Ok(Course::Cs601),
            "cs602" => Ok(Course::Cs602),
            "ee101" => Ok(Course::Ee101),
            "ee601" => Ok(Course::Ee601),
            "ee602" => Ok(Course::Ee602),
            _ => Err(ParseError::UnknownValue("course".to_string(), value.to_string())),
        }
    }

    fn parse_resource_type(&self, value: &str) -> Result<ResourceType, ParseError> {
        match value {
            "application" => Ok(ResourceType::Application),
            "gradebook" => Ok(ResourceType::Gradebook),
            "roster" => Ok(ResourceType::Roster),
            "transcript" => Ok(ResourceType::Transcript),
            _ => Err(ParseError::UnknownValue("resource_type".to_string(), value.to_string())),
        }
    }

    fn parse_course_set(&self, value: &str) -> Result<HashSet<Course>, ParseError> {
        let mut courses = HashSet::new();
        
        if value.starts_with('{') && value.ends_with('}') {
            let content = &value[1..value.len()-1];
            for course_str in content.split_whitespace() {
                courses.insert(self.parse_course(course_str)?);
            }
        } else {
            courses.insert(self.parse_course(value)?);
        }
        
        Ok(courses)
    }

    fn parse_department_set(&self, value: &str) -> Result<HashSet<Department>, ParseError> {
        let mut departments = HashSet::new();
        
        if value.starts_with('{') && value.ends_with('}') {
            let content = &value[1..value.len()-1];
            for dept_str in content.split_whitespace() {
                departments.insert(self.parse_department(dept_str)?);
            }
        } else {
            departments.insert(self.parse_department(value)?);
        }
        
        Ok(departments)
    }

    fn parse_boolean(&self, value: &str) -> Result<bool, ParseError> {
        match value {
            "True" | "true" => Ok(true),
            "False" | "false" => Ok(false),
            _ => Err(ParseError::UnknownValue("boolean".to_string(), value.to_string())),
        }
    }
}

// 後方互換性のための型エイリアス
pub type UniversityAbacParser = GenericAbacParser<UniversityDomainParser>;

// 属性名の文字列からの変換
impl std::str::FromStr for AttributeName {
    type Err = ParseError;
    
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
            _ => Err(ParseError::UnknownValue("attribute_name".to_string(), s.to_string())),
        }
    }
}
