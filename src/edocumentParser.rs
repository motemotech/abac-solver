use std::collections::HashSet;
use std::str::FromStr;

use crate::edocumentTypes::*;

#[derive(Debug, Clone)]
pub enum ParseError {
    InvalidLine(String),
    UnknownRole(String),
    UnknownPosition(String),
    UnknownTenant(String),
    UnknownDepartment(String),
    UnknownOffice(String),
    UnknownDocumentType(String),
    UnknownAction(String),
    UnknownAttributeName(String),
    UnknownOperator(String),
    MissingAttribute(String),
    InvalidFormat(String),
    InvalidCondition(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidLine(line) => write!(f, "Invalid line: {}", line),
            ParseError::UnknownRole(role) => write!(f, "Unknown role: {}", role),
            ParseError::UnknownPosition(pos) => write!(f, "Unknown position: {}", pos),
            ParseError::UnknownTenant(tenant) => write!(f, "Unknown tenant: {}", tenant),
            ParseError::UnknownDepartment(dept) => write!(f, "Unknown department: {}", dept),
            ParseError::UnknownOffice(office) => write!(f, "Unknown office: {}", office),
            ParseError::UnknownDocumentType(dt) => write!(f, "Unknown document type: {}", dt),
            ParseError::UnknownAction(action) => write!(f, "Unknown action: {}", action),
            ParseError::UnknownAttributeName(attr) => write!(f, "Unknown attribute name: {}", attr),
            ParseError::UnknownOperator(op) => write!(f, "Unknown operator: {}", op),
            ParseError::MissingAttribute(attr) => write!(f, "Missing attribute: {}", attr),
            ParseError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            ParseError::InvalidCondition(cond) => write!(f, "Invalid condition: {}", cond),
        }
    }
}

impl std::error::Error for ParseError {}

pub struct EdocumentAbacParser;

impl EdocumentAbacParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, content: &str) -> Result<EdocumentAbacData, ParseError> {
        let mut users = Vec::new();
        let mut resources = Vec::new();
        let mut rules = Vec::new();

        for line in content.lines() {
            let line = line.trim();
            
            // コメントや空行をスキップ
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if line.starts_with("userAttrib(") {
                users.push(self.parse_user_attribute(line)?);
            } else if line.starts_with("resourceAttrib(") {
                resources.push(self.parse_resource_attribute(line)?);
            } else if line.starts_with("rule(") {
                rules.push(self.parse_rule(line, rules.len())?);
            }
        }

        Ok(EdocumentAbacData {
            users,
            resources,
            rules,
        })
    }

    fn parse_user_attribute(&self, line: &str) -> Result<EdocumentUserAttribute, ParseError> {
        // userAttrib(user0, role=employee, position=seniorOfficeManager, tenant=londonOffice, department=londonOfficeAudit, office=none, registered=True, projects={doc210 doc256 doc268}, supervisor=none, supervisee={user25}, payrollingPermissions=True)
        let content = self.extract_parentheses_content(line)?;
        let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();
        
        if parts.is_empty() {
            return Err(ParseError::InvalidFormat("Empty user attribute".to_string()));
        }

        let user_id = parts[0].to_string();
        let mut user_attr = EdocumentUserAttribute::new(user_id);

        for part in &parts[1..] {
            if let Some((key, value)) = part.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                match key {
                    "role" => {
                        user_attr.role = Some(self.parse_role(value)?);
                    }
                    "position" => {
                        user_attr.position = Some(self.parse_position(value)?);
                    }
                    "tenant" => {
                        user_attr.tenant = Some(self.parse_tenant(value)?);
                    }
                    "department" => {
                        user_attr.department = Some(self.parse_department(value)?);
                    }
                    "office" => {
                        user_attr.office = Some(self.parse_office(value)?);
                    }
                    "registered" => {
                        user_attr.registered = self.parse_boolean(value)?;
                    }
                    "projects" => {
                        user_attr.projects = self.parse_string_set(value)?;
                    }
                    "supervisor" => {
                        if value != "none" {
                            user_attr.supervisor = Some(value.to_string());
                        }
                    }
                    "supervisee" => {
                        user_attr.supervisee = self.parse_string_set(value)?;
                    }
                    "payrollingPermissions" => {
                        user_attr.payrolling_permissions = self.parse_boolean(value)?;
                    }
                    "securityClearance" => {
                        user_attr.security_clearance = Some(self.parse_security_level(value)?);
                    }
                    "experience" => {
                        user_attr.experience = Some(self.parse_u32(value)?);
                    }
                    "customerTier" => {
                        user_attr.customer_tier = Some(self.parse_customer_tier(value)?);
                    }
                    "region" => {
                        user_attr.region = Some(self.parse_region(value)?);
                    }
                    "country" => {
                        user_attr.country = Some(self.parse_country(value)?);
                    }
                    "city" => {
                        user_attr.city = Some(value.to_string());
                    }
                    "timeZone" => {
                        user_attr.time_zone = Some(self.parse_time_zone(value)?);
                    }
                    "workingHours" => {
                        user_attr.working_hours = Some(value.to_string());
                    }
                    "temporaryAccess" => {
                        user_attr.temporary_access = self.parse_string_set(value)?;
                    }
                    "delegatedAuthority" => {
                        user_attr.delegated_authority = self.parse_string_set(value)?;
                    }
                    "currentProjects" => {
                        user_attr.current_projects = self.parse_string_set(value)?;
                    }
                    "pastProjects" => {
                        user_attr.past_projects = self.parse_string_set(value)?;
                    }
                    "certifications" => {
                        user_attr.certifications = self.parse_string_set(value)?;
                    }
                    "isActive" => {
                        user_attr.is_active = self.parse_boolean(value)?;
                    }
                    "lastLogin" => {
                        user_attr.last_login = Some(value.to_string());
                    }
                    "contractType" => {
                        user_attr.contract_type = Some(self.parse_contract_type(value)?);
                    }
                    "budgetAuthority" => {
                        user_attr.budget_authority = Some(self.parse_u64(value)?);
                    }
                    _ => {
                        // 未知の属性は無視
                    }
                }
            }
        }

        Ok(user_attr)
    }

    fn parse_resource_attribute(&self, line: &str) -> Result<EdocumentResourceAttribute, ParseError> {
        // resourceAttrib(doc0, type=bankingNote, owner=user321, tenant=europeRegion, department=europeRegionIT, office=none, recipients={user43 user12}, isConfidential=False, containsPersonalInfo=True)
        let content = self.extract_parentheses_content(line)?;
        let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();
        
        if parts.len() < 2 {
            return Err(ParseError::InvalidFormat("Resource attribute needs at least type".to_string()));
        }

        let resource_id = parts[0].to_string();
        let mut document_type = None;
        let mut owner = String::new();
        let mut tenant = Tenant::LargeBank;
        let mut department = Department::None;
        let mut office = Office::None;
        let mut recipients = HashSet::new();
        let mut is_confidential = false;
        let mut contains_personal_info = false;
        
        // Enhanced attributes
        let mut security_level = None;
        let mut created_date = None;
        let mut expiry_date = None;
        let mut project_id = None;
        let mut version = None;
        let mut size = None;
        let mut doc_format = None;
        let mut language = None;
        let mut region = None;
        let mut country = None;
        let mut approval_status = None;
        let mut reviewers = HashSet::new();
        let mut approvers = HashSet::new();
        let mut related_documents = HashSet::new();
        let mut tags = HashSet::new();
        let mut compliance_requirements = HashSet::new();
        let mut retention_period = None;
        let mut is_archived = false;
        let mut last_modified = None;
        let mut access_count = None;
        let mut priority = None;

        for part in &parts[1..] {
            if let Some((key, value)) = part.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                match key {
                    "type" => {
                        document_type = Some(self.parse_document_type(value)?);
                    }
                    "owner" => {
                        owner = value.to_string();
                    }
                    "tenant" => {
                        tenant = self.parse_tenant(value)?;
                    }
                    "department" => {
                        department = self.parse_department(value)?;
                    }
                    "office" => {
                        office = self.parse_office(value)?;
                    }
                    "recipients" => {
                        recipients = self.parse_string_set(value)?;
                    }
                    "isConfidential" => {
                        is_confidential = self.parse_boolean(value)?;
                    }
                    "containsPersonalInfo" => {
                        contains_personal_info = self.parse_boolean(value)?;
                    }
                    "securityLevel" => {
                        security_level = Some(self.parse_security_level(value)?);
                    }
                    "createdDate" => {
                        created_date = Some(value.to_string());
                    }
                    "expiryDate" => {
                        expiry_date = Some(value.to_string());
                    }
                    "projectId" => {
                        project_id = Some(value.to_string());
                    }
                    "version" => {
                        version = Some(value.to_string());
                    }
                    "size" => {
                        size = Some(self.parse_u64(value)?);
                    }
                    "format" => {
                        doc_format = Some(self.parse_format(value)?);
                    }
                    "language" => {
                        language = Some(self.parse_language(value)?);
                    }
                    "region" => {
                        region = Some(self.parse_region(value)?);
                    }
                    "country" => {
                        country = Some(self.parse_country(value)?);
                    }
                    "approvalStatus" => {
                        approval_status = Some(self.parse_approval_status(value)?);
                    }
                    "reviewers" => {
                        reviewers = self.parse_string_set(value)?;
                    }
                    "approvers" => {
                        approvers = self.parse_string_set(value)?;
                    }
                    "relatedDocuments" => {
                        related_documents = self.parse_string_set(value)?;
                    }
                    "tags" => {
                        tags = self.parse_string_set(value)?;
                    }
                    "complianceRequirements" => {
                        compliance_requirements = self.parse_string_set(value)?;
                    }
                    "retentionPeriod" => {
                        retention_period = Some(self.parse_u32(value)?);
                    }
                    "isArchived" => {
                        is_archived = self.parse_boolean(value)?;
                    }
                    "lastModified" => {
                        last_modified = Some(value.to_string());
                    }
                    "accessCount" => {
                        access_count = Some(self.parse_u32(value)?);
                    }
                    "priority" => {
                        priority = Some(self.parse_priority(value)?);
                    }
                    _ => {
                        // 未知の属性は無視
                    }
                }
            }
        }

        let document_type = document_type.ok_or_else(|| 
            ParseError::MissingAttribute("type".to_string()))?;

        Ok(EdocumentResourceAttribute {
            resource_id,
            document_type,
            owner,
            tenant,
            department,
            office,
            recipients,
            is_confidential,
            contains_personal_info,
            // Enhanced attributes
            security_level,
            created_date,
            expiry_date,
            project_id,
            version,
            size,
            format: doc_format,
            language,
            region,
            country,
            approval_status,
            reviewers,
            approvers,
            related_documents,
            tags,
            compliance_requirements,
            retention_period,
            is_archived,
            last_modified,
            access_count,
            priority,
        })
    }

    fn parse_rule(&self, line: &str, id: usize) -> Result<EdocumentRule, ParseError> {
        // rule(role [ {customer}, registered [ {False}; ; {view}; uid [ recipients)
        let content = self.extract_parentheses_content(line)?;
        let sections: Vec<&str> = content.split(';').collect();
        
        if sections.len() < 3 || sections.len() > 4 {
            println!("Warning: Rule {} has {} sections, attempting to parse anyway", id, sections.len());
            if sections.len() < 2 {
                return Err(ParseError::InvalidFormat(
                    "Rule must have at least 2 sections separated by semicolons".to_string()
                ));
            }
        }

        let mut rule = EdocumentRule::new(id);

        // セクション1: ユーザー条件
        let user_section = sections[0].trim();
        if !user_section.is_empty() {
            rule.user_conditions = self.parse_conditions_section(user_section)?;
        }

        // セクション2: リソース条件
        let resource_section = sections[1].trim();
        if !resource_section.is_empty() {
            rule.resource_conditions = self.parse_conditions_section(resource_section)?;
        }

        // セクション3: アクション
        let action_section = sections[2].trim();
        rule.actions = self.parse_actions_section(action_section)?;

        // セクション4: 比較条件（存在する場合）
        if sections.len() == 4 {
            let comparison_section = sections[3].trim();
            if !comparison_section.is_empty() {
                rule.comparison_conditions = self.parse_comparison_section(comparison_section)?;
            }
        }

        Ok(rule)
    }

    fn parse_conditions_section(&self, section: &str) -> Result<Vec<Condition>, ParseError> {
        let mut conditions = Vec::new();
        
        // カンマで条件を分割
        for condition_str in section.split(',') {
            let condition_str = condition_str.trim();
            if !condition_str.is_empty() {
                conditions.push(self.parse_single_condition(condition_str)?);
            }
        }

        Ok(conditions)
    }

    fn parse_single_condition(&self, condition_str: &str) -> Result<Condition, ParseError> {
        // 例: "role [ {customer}", "registered [ {False}", "uid [ recipients", "budgetAuthority > 100000"
        
        // 演算子を見つける（長い演算子を先に検索）
        let operators = vec![
            " >= ", " <= ", " != ", " [ ", " ] ", " = ", " > ", " < ",
            ">=", "<=", "!=", "[", "]", "=", ">", "<"
        ];
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

        // 数値のチェック
        if let Ok(num) = value_str.parse::<u64>() {
            return Ok(AttributeValue::Number(num));
        }

        // Role, Position, Tenant, Department, Office, DocumentTypeのチェック
        if let Ok(role) = self.parse_role(value_str) {
            return Ok(AttributeValue::Role(role));
        }
        if let Ok(pos) = self.parse_position(value_str) {
            return Ok(AttributeValue::Position(pos));
        }
        if let Ok(tenant) = self.parse_tenant(value_str) {
            return Ok(AttributeValue::Tenant(tenant));
        }
        if let Ok(dept) = self.parse_department(value_str) {
            return Ok(AttributeValue::Department(dept));
        }
        if let Ok(office) = self.parse_office(value_str) {
            return Ok(AttributeValue::Office(office));
        }
        if let Ok(doc_type) = self.parse_document_type(value_str) {
            return Ok(AttributeValue::DocumentType(doc_type));
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
        // 例: "uid [ recipients", "tenant = tenant"
        self.parse_conditions_section(section)
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

    fn parse_role(&self, value: &str) -> Result<Role, ParseError> {
        Role::from_str(value).map_err(|_| ParseError::UnknownRole(value.to_string()))
    }

    fn parse_position(&self, value: &str) -> Result<Position, ParseError> {
        Position::from_str(value).map_err(|_| ParseError::UnknownPosition(value.to_string()))
    }

    fn parse_tenant(&self, value: &str) -> Result<Tenant, ParseError> {
        Tenant::from_str(value).map_err(|_| ParseError::UnknownTenant(value.to_string()))
    }

    fn parse_department(&self, value: &str) -> Result<Department, ParseError> {
        Department::from_str(value).map_err(|_| ParseError::UnknownDepartment(value.to_string()))
    }

    fn parse_office(&self, value: &str) -> Result<Office, ParseError> {
        Office::from_str(value).map_err(|_| ParseError::UnknownOffice(value.to_string()))
    }

    fn parse_document_type(&self, value: &str) -> Result<DocumentType, ParseError> {
        DocumentType::from_str(value).map_err(|_| ParseError::UnknownDocumentType(value.to_string()))
    }

    fn parse_action(&self, value: &str) -> Result<Action, ParseError> {
        Action::from_str(value).map_err(|_| ParseError::UnknownAction(value.to_string()))
    }

    fn parse_string_set(&self, value: &str) -> Result<HashSet<String>, ParseError> {
        let mut strings = HashSet::new();
        
        if value.starts_with('{') && value.ends_with('}') {
            let content = &value[1..value.len()-1];
            for string_str in content.split_whitespace() {
                strings.insert(string_str.to_string());
            }
        } else if value != "none" && !value.is_empty() {
            strings.insert(value.to_string());
        }
        
        Ok(strings)
    }

    fn parse_boolean(&self, value: &str) -> Result<bool, ParseError> {
        match value {
            "True" | "true" => Ok(true),
            "False" | "false" => Ok(false),
            _ => Err(ParseError::InvalidFormat(format!("Invalid boolean value: {}", value))),
        }
    }

    fn parse_security_level(&self, value: &str) -> Result<SecurityLevel, ParseError> {
        SecurityLevel::from_str(value).map_err(|_| ParseError::InvalidFormat(format!("Invalid security level: {}", value)))
    }

    fn parse_customer_tier(&self, value: &str) -> Result<CustomerTier, ParseError> {
        CustomerTier::from_str(value).map_err(|_| ParseError::InvalidFormat(format!("Invalid customer tier: {}", value)))
    }

    fn parse_region(&self, value: &str) -> Result<Region, ParseError> {
        Region::from_str(value).map_err(|_| ParseError::InvalidFormat(format!("Invalid region: {}", value)))
    }

    fn parse_country(&self, value: &str) -> Result<Country, ParseError> {
        Country::from_str(value).map_err(|_| ParseError::InvalidFormat(format!("Invalid country: {}", value)))
    }

    fn parse_time_zone(&self, value: &str) -> Result<TimeZone, ParseError> {
        TimeZone::from_str(value).map_err(|_| ParseError::InvalidFormat(format!("Invalid time zone: {}", value)))
    }

    fn parse_contract_type(&self, value: &str) -> Result<ContractType, ParseError> {
        ContractType::from_str(value).map_err(|_| ParseError::InvalidFormat(format!("Invalid contract type: {}", value)))
    }

    fn parse_approval_status(&self, value: &str) -> Result<ApprovalStatus, ParseError> {
        ApprovalStatus::from_str(value).map_err(|_| ParseError::InvalidFormat(format!("Invalid approval status: {}", value)))
    }

    fn parse_priority(&self, value: &str) -> Result<Priority, ParseError> {
        Priority::from_str(value).map_err(|_| ParseError::InvalidFormat(format!("Invalid priority: {}", value)))
    }

    fn parse_format(&self, value: &str) -> Result<Format, ParseError> {
        Format::from_str(value).map_err(|_| ParseError::InvalidFormat(format!("Invalid format: {}", value)))
    }

    fn parse_language(&self, value: &str) -> Result<Language, ParseError> {
        Language::from_str(value).map_err(|_| ParseError::InvalidFormat(format!("Invalid language: {}", value)))
    }

    fn parse_u32(&self, value: &str) -> Result<u32, ParseError> {
        value.parse::<u32>().map_err(|_| ParseError::InvalidFormat(format!("Invalid number: {}", value)))
    }

    fn parse_u64(&self, value: &str) -> Result<u64, ParseError> {
        value.parse::<u64>().map_err(|_| ParseError::InvalidFormat(format!("Invalid number: {}", value)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_user_attribute() {
        let parser = EdocumentAbacParser::new();
        let line = "userAttrib(user0, role=employee, position=seniorOfficeManager, tenant=londonOffice, department=londonOfficeAudit, office=none, registered=True, projects={doc210 doc256}, supervisor=none, supervisee={user25}, payrollingPermissions=True)";
        let result = parser.parse_user_attribute(line).unwrap();
        
        assert_eq!(result.user_id, "user0");
        assert_eq!(result.role, Some(Role::Employee));
        assert_eq!(result.position, Some(Position::SeniorOfficeManager));
        assert_eq!(result.tenant, Some(Tenant::LondonOffice));
        assert_eq!(result.department, Some(Department::LondonOfficeAudit));
        assert_eq!(result.office, Some(Office::None));
        assert_eq!(result.registered, true);
        assert!(result.projects.contains("doc210"));
        assert!(result.projects.contains("doc256"));
        assert_eq!(result.supervisor, None);
        assert!(result.supervisee.contains("user25"));
        assert_eq!(result.payrolling_permissions, true);
    }

    #[test]
    fn test_parse_resource_attribute() {
        let parser = EdocumentAbacParser::new();
        let line = "resourceAttrib(doc0, type=bankingNote, owner=user321, tenant=europeRegion, department=europeRegionIT, office=none, recipients={user43 user12}, isConfidential=False, containsPersonalInfo=True)";
        let result = parser.parse_resource_attribute(line).unwrap();
        
        assert_eq!(result.resource_id, "doc0");
        assert_eq!(result.document_type, DocumentType::BankingNote);
        assert_eq!(result.owner, "user321");
        assert_eq!(result.tenant, Tenant::EuropeRegion);
        assert_eq!(result.department, Department::EuropeRegionIT);
        assert_eq!(result.office, Office::None);
        assert!(result.recipients.contains("user43"));
        assert!(result.recipients.contains("user12"));
        assert_eq!(result.is_confidential, false);
        assert_eq!(result.contains_personal_info, true);
    }

    #[test]
    fn test_parse_rule() {
        let parser = EdocumentAbacParser::new();
        let line = "rule(role [ {customer}, registered [ {False}; ; {view}; uid [ recipients)";
        let result = parser.parse_rule(line, 0).unwrap();
        
        assert_eq!(result.id, 0);
        assert_eq!(result.user_conditions.len(), 2); // role and registered
        assert_eq!(result.resource_conditions.len(), 0); // empty section
        assert_eq!(result.actions.len(), 1); // view
        assert_eq!(result.comparison_conditions.len(), 1); // uid [ recipients
    }

    #[test]
    fn test_parse_helpdesk_user() {
        let parser = EdocumentAbacParser::new();
        let line = "userAttrib(hdop0, role=helpdesk, position=none, tenant=reseller, department=none, office=none, registered=False, projects={doc110 doc42}, supervisor=none, supervisee={}, payrollingPermissions=False)";
        let result = parser.parse_user_attribute(line).unwrap();
        
        assert_eq!(result.user_id, "hdop0");
        assert_eq!(result.role, Some(Role::Helpdesk));
        assert_eq!(result.position, Some(Position::None));
        assert_eq!(result.tenant, Some(Tenant::Reseller));
        assert_eq!(result.department, Some(Department::None));
        assert_eq!(result.office, Some(Office::None));
        assert_eq!(result.registered, false);
        assert!(result.projects.contains("doc110"));
        assert!(result.projects.contains("doc42"));
        assert_eq!(result.supervisor, None);
        assert!(result.supervisee.is_empty());
        assert_eq!(result.payrolling_permissions, false);
    }

    #[test]
    fn test_parse_admin_user() {
        let parser = EdocumentAbacParser::new();
        let line = "userAttrib(admin0, role=admin, position=none, tenant=europeRegion, department=none, office=none, registered=False, projects={doc269}, supervisor=none, supervisee={}, payrollingPermissions=False)";
        let result = parser.parse_user_attribute(line).unwrap();
        
        assert_eq!(result.user_id, "admin0");
        assert_eq!(result.role, Some(Role::Admin));
        assert_eq!(result.position, Some(Position::None));
        assert_eq!(result.tenant, Some(Tenant::EuropeRegion));
        assert_eq!(result.department, Some(Department::None));
        assert_eq!(result.office, Some(Office::None));
        assert_eq!(result.registered, false);
        assert!(result.projects.contains("doc269"));
        assert_eq!(result.supervisor, None);
        assert!(result.supervisee.is_empty());
        assert_eq!(result.payrolling_permissions, false);
    }

    #[test]
    fn test_parse_customer_user() {
        let parser = EdocumentAbacParser::new();
        let line = "userAttrib(cstmr0, role=customer, position=none, tenant=carLeaser, department=carLeaserAccounting, office=none, registered=False, projects={}, supervisor=none, supervisee={}, payrollingPermissions=False)";
        let result = parser.parse_user_attribute(line).unwrap();
        
        assert_eq!(result.user_id, "cstmr0");
        assert_eq!(result.role, Some(Role::Customer));
        assert_eq!(result.position, Some(Position::None));
        assert_eq!(result.tenant, Some(Tenant::CarLeaser));
        assert_eq!(result.department, Some(Department::CarLeaserAccounting));
        assert_eq!(result.office, Some(Office::None));
        assert_eq!(result.registered, false);
        assert!(result.projects.is_empty());
        assert_eq!(result.supervisor, None);
        assert!(result.supervisee.is_empty());
        assert_eq!(result.payrolling_permissions, false);
    }

    #[test]
    fn test_parse_document_types() {
        let parser = EdocumentAbacParser::new();
        
        // Test different document types
        let line_banking = "resourceAttrib(doc0, type=bankingNote, owner=user0)";
        let result_banking = parser.parse_resource_attribute(line_banking).unwrap();
        assert_eq!(result_banking.document_type, DocumentType::BankingNote);
        
        let line_traffic = "resourceAttrib(doc1, type=trafficFine, owner=user1)";
        let result_traffic = parser.parse_resource_attribute(line_traffic).unwrap();
        assert_eq!(result_traffic.document_type, DocumentType::TrafficFine);
        
        let line_sales = "resourceAttrib(doc2, type=salesOffer, owner=user2)";
        let result_sales = parser.parse_resource_attribute(line_sales).unwrap();
        assert_eq!(result_sales.document_type, DocumentType::SalesOffer);
        
        let line_contract = "resourceAttrib(doc3, type=contract, owner=user3)";
        let result_contract = parser.parse_resource_attribute(line_contract).unwrap();
        assert_eq!(result_contract.document_type, DocumentType::Contract);
        
        let line_invoice = "resourceAttrib(doc4, type=invoice, owner=user4)";
        let result_invoice = parser.parse_resource_attribute(line_invoice).unwrap();
        assert_eq!(result_invoice.document_type, DocumentType::Invoice);
        
        let line_paycheck = "resourceAttrib(doc5, type=paycheck, owner=user5)";
        let result_paycheck = parser.parse_resource_attribute(line_paycheck).unwrap();
        assert_eq!(result_paycheck.document_type, DocumentType::Paycheck);
    }

    #[test]
    fn test_parse_full_content() {
        let parser = EdocumentAbacParser::new();
        let content = r#"
            # Test ABAC file for edocument
            userAttrib(user0, role=employee, position=secretary, tenant=largeBank, department=largeBankSales, office=largeBankOffice1, registered=True, projects={}, supervisor=none, supervisee={}, payrollingPermissions=False)
            resourceAttrib(doc0, type=invoice, owner=user0, tenant=largeBank, department=largeBankSales, office=largeBankOffice1, recipients={user1}, isConfidential=False, containsPersonalInfo=False)
            rule(role [ {employee}; type [ {invoice}; {view}; )
        "#;
        
        let result = parser.parse(content).unwrap();
        assert_eq!(result.users.len(), 1);
        assert_eq!(result.resources.len(), 1);
        assert_eq!(result.rules.len(), 1);
    }
} 