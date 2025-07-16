use std::collections::HashSet;
use std::str::FromStr;
use std::fs;

use crate::university_types::*;

#[derive(Debug, Clone)]
pub enum ParseError {
    InvalidLine(String),
    UnknownPosition(String),
    UnknownDepartment(String),
    UnknownCourse(String),
    UnknownResourceType(String),
    UnknownAction(String),
    UnknownAttributeName(String),
    UnknownOperator(String),
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
            ParseError::UnknownPosition(pos) => write!(f, "Unknown position: {}", pos),
            ParseError::UnknownDepartment(dept) => write!(f, "Unknown department: {}", dept),
            ParseError::UnknownCourse(course) => write!(f, "Unknown course: {}", course),
            ParseError::UnknownResourceType(rt) => write!(f, "Unknown resource type: {}", rt),
            ParseError::UnknownAction(action) => write!(f, "Unknown action: {}", action),
            ParseError::UnknownAttributeName(attr) => write!(f, "Unknown attribute name: {}", attr),
            ParseError::UnknownOperator(op) => write!(f, "Unknown operator: {}", op),
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

pub struct UniversityAbacParser;

impl UniversityAbacParser {
    pub fn new() -> Self {
        Self
    }

    /// ファイルパスからファイルを読み取ってパースします
    pub fn parse_file(&self, file_path: &str) -> Result<UniversityAbacData, ParseError> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| ParseError::FileError(format!("Failed to read file '{}': {}", file_path, e)))?;
        self.parse(&content)
    }

    /// 文字列コンテンツをパースします
    pub fn parse(&self, content: &str) -> Result<UniversityAbacData, ParseError> {
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

        Ok(UniversityAbacData {
            users,
            resources,
            rules,
        })
    }

    fn parse_user_attribute(&self, line_num: usize, line: &str) -> Result<UniversityUserAttribute, ParseError> {
        // userAttrib(csStu1, position=student, department=cs, crsTaken={cs101})
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
        let mut user_attr = UniversityUserAttribute::new(user_id);

        for part in &parts[1..] {
            if let Some((key, value)) = part.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                match key {
                    "position" => {
                        user_attr.position = Some(self.parse_position(value)
                            .map_err(|e| ParseError::ParseErrorAtLine(
                                line_num + 1,
                                line.to_string(),
                                format!("Error parsing position in user attribute: {}", e)
                            ))?);
                    }
                    "department" => {
                        user_attr.department = Some(self.parse_department(value)
                            .map_err(|e| ParseError::ParseErrorAtLine(
                                line_num + 1,
                                line.to_string(),
                                format!("Error parsing department in user attribute: {}", e)
                            ))?);
                    }
                    "crsTaken" => {
                        user_attr.crs_taken = self.parse_course_set(value)
                            .map_err(|e| ParseError::ParseErrorAtLine(
                                line_num + 1,
                                line.to_string(),
                                format!("Error parsing crsTaken in user attribute: {}", e)
                            ))?;
                    }
                    "crsTaught" => {
                        user_attr.crs_taught = self.parse_course_set(value)
                            .map_err(|e| ParseError::ParseErrorAtLine(
                                line_num + 1,
                                line.to_string(),
                                format!("Error parsing crsTaught in user attribute: {}", e)
                            ))?;
                    }
                    "isChair" => {
                        user_attr.is_chair = Some(self.parse_boolean(value)
                            .map_err(|e| ParseError::ParseErrorAtLine(
                                line_num + 1,
                                line.to_string(),
                                format!("Error parsing isChair in user attribute: {}", e)
                            ))?);
                    }
                    _ => {
                        // 未知の属性は無視
                    }
                }
            }
        }

        Ok(user_attr)
    }

    fn parse_resource_attribute(&self, line_num: usize, line: &str) -> Result<UniversityResourceAttribute, ParseError> {
        // resourceAttrib(application1, type=application, student=applicant1)
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
        let mut student = None;
        let mut departments = HashSet::new();
        let mut crs = None;

        for part in &parts[1..] {
            if let Some((key, value)) = part.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                match key {
                    "type" => {
                        resource_type = Some(self.parse_resource_type(value)
                            .map_err(|e| ParseError::ParseErrorAtLine(
                                line_num + 1,
                                line.to_string(),
                                format!("Error parsing type in resource attribute: {}", e)
                            ))?);
                    }
                    "student" => {
                        student = Some(value.to_string());
                    }
                    "departments" => {
                        departments = self.parse_department_set(value)
                            .map_err(|e| ParseError::ParseErrorAtLine(
                                line_num + 1,
                                line.to_string(),
                                format!("Error parsing departments in resource attribute: {}", e)
                            ))?;
                    }
                    "crs" => {
                        crs = Some(self.parse_course(value)
                            .map_err(|e| ParseError::ParseErrorAtLine(
                                line_num + 1,
                                line.to_string(),
                                format!("Error parsing crs in resource attribute: {}", e)
                            ))?);
                    }
                    _ => {
                        // 未知の属性は無視
                    }
                }
            }
        }

        let resource_type = resource_type.ok_or_else(|| 
            ParseError::ParseErrorAtLine(
                line_num + 1,
                line.to_string(),
                "Missing required attribute: type".to_string()
            ))?;

        Ok(UniversityResourceAttribute {
            resource_id,
            resource_type,
            student,
            departments,
            crs,
        })
    }

    fn parse_rule(&self, line_num: usize, line: &str, id: usize) -> Result<UniversityRule, ParseError> {
        // rule(position [ {faculty}; type [ {gradebook}; {changeScore assignGrade}; crsTaught ] crs)
        let content = self.extract_parentheses_content(line)?;
        let sections: Vec<&str> = content.split(';').collect();
        
        if sections.len() < 3 || sections.len() > 4 {
            return Err(ParseError::ParseErrorAtLine(
                line_num + 1, // 1-based line numbering for user display
                line.to_string(),
                "Rule must have 3 or 4 sections separated by semicolons".to_string()
            ));
        }

        let mut rule = UniversityRule::new(id);

        // セクション1: ユーザー条件
        let user_section = sections[0].trim();
        if !user_section.is_empty() {
            rule.user_conditions = self.parse_conditions_section(user_section)
                .map_err(|e| ParseError::ParseErrorAtLine(
                    line_num + 1,
                    line.to_string(),
                    format!("Error parsing user conditions: {}", e)
                ))?;
        }

        // セクション2: リソース条件
        let resource_section = sections[1].trim();
        if !resource_section.is_empty() {
            rule.resource_conditions = self.parse_conditions_section(resource_section)
                .map_err(|e| ParseError::ParseErrorAtLine(
                    line_num + 1,
                    line.to_string(),
                    format!("Error parsing resource conditions: {}", e)
                ))?;
        }

        // セクション3: アクション
        let action_section = sections[2].trim();
        rule.actions = self.parse_actions_section(action_section)
            .map_err(|e| ParseError::ParseErrorAtLine(
                line_num + 1,
                line.to_string(),
                format!("Error parsing actions: {}", e)
            ))?;

        // セクション4: 比較条件（存在する場合）
        if sections.len() == 4 {
            let comparison_section = sections[3].trim();
            if !comparison_section.is_empty() {
                rule.comparison_conditions = self.parse_comparison_section(comparison_section)
                    .map_err(|e| ParseError::ParseErrorAtLine(
                        line_num + 1,
                        line.to_string(),
                        format!("Error parsing comparison conditions: {}", e)
                    ))?;
            }
        }

        Ok(rule)
    }

    fn parse_conditions_section(&self, section: &str) -> Result<Vec<Condition>, ParseError> {
        let mut conditions = Vec::new();
        
        // カンマで条件を分割（セミコロンはルールのセクション間の区切りなので使わない）
        for condition_str in section.split(',') {
            let condition_str = condition_str.trim();
            if !condition_str.is_empty() {
                conditions.push(self.parse_single_condition(condition_str)?);
            }
        }

        Ok(conditions)
    }

    fn parse_single_condition(&self, condition_str: &str) -> Result<Condition, ParseError> {
        // 例: "position [ {faculty}", "type [ {gradebook}", "uid=student"
        
        // 演算子を見つける（スペースありとスペースなしの両方に対応）
        // 長い演算子を先に検索して、より具体的なマッチを優先
        let operators = vec![" [ ", " ] ", " = ", "[", "]", "="];
        let mut found_operator = None;
        let mut split_pos = None;
        let mut operator_len = 0;

        for op in &operators {
            if let Some(pos) = condition_str.find(op) {
                found_operator = Some(op.trim()); // スペースを除去して正規化
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

        let left = self.parse_attribute_expression(left_str)?;
        let right = self.parse_attribute_expression(right_str)?;
        let op = ComparisonOperator::from_str(operator)
            .map_err(|_| ParseError::UnknownOperator(operator.to_string()))?;

        Ok(Condition {
            left,
            operator: op,
            right,
        })
    }

    fn parse_attribute_expression(&self, expr_str: &str) -> Result<AttributeExpression, ParseError> {
        let expr_str = expr_str.trim();

        // 波括弧で囲まれたセットかチェック
        if expr_str.starts_with('{') && expr_str.ends_with('}') {
            let content = &expr_str[1..expr_str.len()-1];
            let mut values = Vec::new();
            
            for value_str in content.split_whitespace() {
                let value = self.parse_attribute_value(value_str)?;
                values.push(value);
            }
            
            return Ok(AttributeExpression::ValueSet(values));
        }

        // 属性名かどうかチェック
        if let Ok(attr_name) = AttributeName::from_str(expr_str) {
            return Ok(AttributeExpression::AttributeName(attr_name));
        }

        // 属性値として解析
        let value = self.parse_attribute_value(expr_str)?;
        Ok(AttributeExpression::AttributeValue(value))
    }

    fn parse_attribute_value(&self, value_str: &str) -> Result<AttributeValue, ParseError> {
        // boolean値のチェック
        if value_str == "True" || value_str == "true" || value_str == "False" || value_str == "false" {
            return Ok(AttributeValue::Boolean(self.parse_boolean(value_str)?));
        }

        // Position, Department, Course, ResourceTypeのチェック
        if let Ok(pos) = self.parse_position(value_str) {
            return Ok(AttributeValue::Position(pos));
        }
        if let Ok(dept) = self.parse_department(value_str) {
            return Ok(AttributeValue::Department(dept));
        }
        if let Ok(course) = self.parse_course(value_str) {
            return Ok(AttributeValue::Course(course));
        }
        if let Ok(rt) = self.parse_resource_type(value_str) {
            return Ok(AttributeValue::ResourceType(rt));
        }

        // 文字列として扱う
        Ok(AttributeValue::String(value_str.to_string()))
    }

    fn parse_actions_section(&self, section: &str) -> Result<HashSet<Action>, ParseError> {
        let mut actions = HashSet::new();
        
        // 波括弧を除去
        let content = if section.starts_with('{') && section.ends_with('}') {
            &section[1..section.len()-1]
        } else {
            section
        };

        for action_str in content.split_whitespace() {
            let action = self.parse_action(action_str)?;
            actions.insert(action);
        }

        Ok(actions)
    }

    fn parse_comparison_section(&self, section: &str) -> Result<Vec<Condition>, ParseError> {
        // 例: "crsTaken ] crs", "uid=student"
        self.parse_conditions_section(section)
    }

    fn parse_action(&self, value: &str) -> Result<Action, ParseError> {
        match value {
            "readMyScores" => Ok(Action::ReadMyScores),
            "addScore" => Ok(Action::AddScore),
            "readScore" => Ok(Action::ReadScore),
            "changeScore" => Ok(Action::ChangeScore),
            "assignGrade" => Ok(Action::AssignGrade),
            "read" => Ok(Action::Read),
            "write" => Ok(Action::Write),
            "checkStatus" => Ok(Action::CheckStatus),
            "setStatus" => Ok(Action::SetStatus),
            _ => Err(ParseError::UnknownAction(value.to_string())),
        }
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

    fn parse_position(&self, value: &str) -> Result<Position, ParseError> {
        match value {
            "applicant" => Ok(Position::Applicant),
            "student" => Ok(Position::Student),
            "faculty" => Ok(Position::Faculty),
            "staff" => Ok(Position::Staff),
            _ => Err(ParseError::UnknownPosition(value.to_string())),
        }
    }

    fn parse_department(&self, value: &str) -> Result<Department, ParseError> {
        match value {
            "cs" => Ok(Department::Cs),
            "ee" => Ok(Department::Ee),
            "registrar" => Ok(Department::Registrar),
            "admissions" => Ok(Department::Admissions),
            _ => Err(ParseError::UnknownDepartment(value.to_string())),
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
            _ => Err(ParseError::UnknownCourse(value.to_string())),
        }
    }

    fn parse_resource_type(&self, value: &str) -> Result<ResourceType, ParseError> {
        match value {
            "application" => Ok(ResourceType::Application),
            "gradebook" => Ok(ResourceType::Gradebook),
            "roster" => Ok(ResourceType::Roster),
            "transcript" => Ok(ResourceType::Transcript),
            _ => Err(ParseError::UnknownResourceType(value.to_string())),
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
            _ => Err(ParseError::InvalidFormat(format!("Invalid boolean value: {}", value))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_user_attribute() {
        let parser = UniversityAbacParser::new();
        let line = "userAttrib(csStu1, position=student, department=cs, crsTaken={cs101})";
        let result = parser.parse_user_attribute(0, line).unwrap();
        
        assert_eq!(result.user_id, "csStu1");
        assert_eq!(result.position, Some(Position::Student));
        assert_eq!(result.department, Some(Department::Cs));
        assert!(result.crs_taken.contains(&Course::Cs101));
    }

    #[test]
    fn test_parse_resource_attribute() {
        let parser = UniversityAbacParser::new();
        let line = "resourceAttrib(application1, type=application, student=applicant1)";
        let result = parser.parse_resource_attribute(0, line).unwrap();
        
        assert_eq!(result.resource_id, "application1");
        assert_eq!(result.resource_type, ResourceType::Application);
        assert_eq!(result.student, Some("applicant1".to_string()));
    }

    #[test]
    fn test_parse_rule() {
        let parser = UniversityAbacParser::new();
        let line = "rule(position [ {faculty}; type [ {gradebook}; {changeScore assignGrade}; crsTaught ] crs)";
        let result = parser.parse_rule(0, line, 0).unwrap();
        
        assert_eq!(result.id, 0);
        assert_eq!(result.user_conditions.len(), 1);
        assert_eq!(result.resource_conditions.len(), 1);
        assert_eq!(result.actions.len(), 2);
        assert_eq!(result.comparison_conditions.len(), 1);
    }

    #[test]
    fn test_parse_rule_with_uid_condition() {
        let parser = UniversityAbacParser::new();
        
        // 実際の問題のあったルールをテスト
        let line = "rule(; type [ {transcript}; {read}; uid=student)";
        let result = parser.parse_rule(0, line, 0).unwrap();
        
        assert_eq!(result.id, 0);
        assert_eq!(result.user_conditions.len(), 0); // 最初のセクションは空
        assert_eq!(result.resource_conditions.len(), 1); // type [ {transcript}
        assert_eq!(result.actions.len(), 1); // {read}
        assert_eq!(result.comparison_conditions.len(), 1); // uid=student
        
        // comparison_conditionsの内容を確認
        let uid_condition = &result.comparison_conditions[0];
        match (&uid_condition.left, &uid_condition.operator, &uid_condition.right) {
            (AttributeExpression::AttributeName(AttributeName::Uid), 
             ComparisonOperator::Equals, 
             AttributeExpression::AttributeValue(AttributeValue::String(value))) => {
                assert_eq!(value, "student");
            },
            _ => panic!("Unexpected condition structure in comparison_conditions: {:?}", uid_condition),
        }
    }

    #[test]
    fn test_parse_conditions_with_equals() {
        let parser = UniversityAbacParser::new();
        
        // スペースなしの等号をテスト
        let condition_str = "uid=student";
        let result = parser.parse_single_condition(condition_str).unwrap();
        
        // 左側が属性名、右側が値であることを確認
        match (&result.left, &result.operator, &result.right) {
            (AttributeExpression::AttributeName(AttributeName::Uid), 
             ComparisonOperator::Equals, 
             AttributeExpression::AttributeValue(AttributeValue::String(value))) => {
                assert_eq!(value, "student");
            },
            _ => panic!("Unexpected condition structure: {:?}", result),
        }
        
        // スペースありの等号もテスト
        let condition_str_spaced = "uid = student";
        let result_spaced = parser.parse_single_condition(condition_str_spaced).unwrap();
        
        match (&result_spaced.left, &result_spaced.operator, &result_spaced.right) {
            (AttributeExpression::AttributeName(AttributeName::Uid), 
             ComparisonOperator::Equals, 
             AttributeExpression::AttributeValue(AttributeValue::String(value))) => {
                assert_eq!(value, "student");
            },
            _ => panic!("Unexpected condition structure: {:?}", result_spaced),
        }
    }

    #[test]
    fn test_parse_boolean_values() {
        let parser = UniversityAbacParser::new();
        
        // Trueの場合をテスト
        let line_true = "userAttrib(csChair, isChair=True, department=cs)";
        let result_true = parser.parse_user_attribute(0, line_true).unwrap();
        assert_eq!(result_true.user_id, "csChair");
        assert_eq!(result_true.is_chair, Some(true));
        assert_eq!(result_true.department, Some(Department::Cs));
        
        // Falseの場合をテスト
        let line_false = "userAttrib(csFac1, isChair=False, department=cs)";
        let result_false = parser.parse_user_attribute(0, line_false).unwrap();
        assert_eq!(result_false.user_id, "csFac1");
        assert_eq!(result_false.is_chair, Some(false));
        
        // 小文字の場合もテスト
        let line_lowercase = "userAttrib(testUser, isChair=true)";
        let result_lowercase = parser.parse_user_attribute(0, line_lowercase).unwrap();
        assert_eq!(result_lowercase.is_chair, Some(true));
    }

    #[test]
    fn test_parse_full_content() {
        let parser = UniversityAbacParser::new();
        let content = r#"
            # Test ABAC file
            userAttrib(student1, position=student, department=cs)
            resourceAttrib(app1, type=application, student=student1)
            rule(; type [ {application}; {read}; uid=student)
        "#;
        
        let result = parser.parse(content).unwrap();
        assert_eq!(result.users.len(), 1);
        assert_eq!(result.resources.len(), 1);
        assert_eq!(result.rules.len(), 1);
    }
}
