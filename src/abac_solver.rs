use std::collections::{HashMap, HashSet};
use z3::{Config, Context, Solver, Sort, FuncDecl, DatatypeBuilder, ast::{Ast, Dynamic}};

use crate::universityTypes::*;

/// ABACソルバーのメイン構造体
pub struct UniversityAbacSolver<'ctx> {
    context: &'ctx Context,
    solver: Solver<'ctx>,
    
    // Z3データ型
    user_sort: z3::DatatypeSort<'ctx>,
    resource_sort: z3::DatatypeSort<'ctx>,
    position_sort: z3::DatatypeSort<'ctx>,
    department_sort: z3::DatatypeSort<'ctx>,
    course_sort: z3::DatatypeSort<'ctx>,
    resource_type_sort: z3::DatatypeSort<'ctx>,
    action_sort: z3::DatatypeSort<'ctx>,
    
    // 属性関数
    user_has_position: FuncDecl<'ctx>,
    user_has_department: FuncDecl<'ctx>,
    user_has_taken_course: FuncDecl<'ctx>,
    user_has_taught_course: FuncDecl<'ctx>,
    user_is_chair: FuncDecl<'ctx>,
    
    resource_has_type: FuncDecl<'ctx>,
    resource_has_student: FuncDecl<'ctx>,
    resource_has_department: FuncDecl<'ctx>,
    resource_has_course: FuncDecl<'ctx>,
    
    // アクセス制御関数
    user_can_perform_action: FuncDecl<'ctx>,
    
    // データマッピング
    user_mapping: HashMap<String, Dynamic<'ctx>>,
    resource_mapping: HashMap<String, Dynamic<'ctx>>,
    position_mapping: HashMap<Position, Dynamic<'ctx>>,
    department_mapping: HashMap<Department, Dynamic<'ctx>>,
    course_mapping: HashMap<Course, Dynamic<'ctx>>,
    resource_type_mapping: HashMap<ResourceType, Dynamic<'ctx>>,
    action_mapping: HashMap<Action, Dynamic<'ctx>>,
}

impl<'ctx> UniversityAbacSolver<'ctx> {
    /// 新しいABACソルバーを作成
    pub fn new(context: &'ctx Context, abac_data: &UniversityAbacData) -> Self {
        let solver = Solver::new(context);
        
        // Z3データ型を構築
        let user_sort = Self::build_user_sort(context, &abac_data.users);
        let resource_sort = Self::build_resource_sort(context, &abac_data.resources);
        let position_sort = Self::build_position_sort(context);
        let department_sort = Self::build_department_sort(context);
        let course_sort = Self::build_course_sort(context);
        let resource_type_sort = Self::build_resource_type_sort(context);
        let action_sort = Self::build_action_sort(context);
        
        // 属性関数を定義
        let user_has_position = FuncDecl::new(context, "user_has_position", 
            &[&user_sort.sort, &position_sort.sort], &Sort::bool(context));
        let user_has_department = FuncDecl::new(context, "user_has_department", 
            &[&user_sort.sort, &department_sort.sort], &Sort::bool(context));
        let user_has_taken_course = FuncDecl::new(context, "user_has_taken_course", 
            &[&user_sort.sort, &course_sort.sort], &Sort::bool(context));
        let user_has_taught_course = FuncDecl::new(context, "user_has_taught_course", 
            &[&user_sort.sort, &course_sort.sort], &Sort::bool(context));
        let user_is_chair = FuncDecl::new(context, "user_is_chair", 
            &[&user_sort.sort], &Sort::bool(context));
        
        let resource_has_type = FuncDecl::new(context, "resource_has_type", 
            &[&resource_sort.sort, &resource_type_sort.sort], &Sort::bool(context));
        let resource_has_student = FuncDecl::new(context, "resource_has_student", 
            &[&resource_sort.sort, &user_sort.sort], &Sort::bool(context));
        let resource_has_department = FuncDecl::new(context, "resource_has_department", 
            &[&resource_sort.sort, &department_sort.sort], &Sort::bool(context));
        let resource_has_course = FuncDecl::new(context, "resource_has_course", 
            &[&resource_sort.sort, &course_sort.sort], &Sort::bool(context));
        
        let user_can_perform_action = FuncDecl::new(context, "user_can_perform_action", 
            &[&user_sort.sort, &resource_sort.sort, &action_sort.sort], &Sort::bool(context));
        
        // データマッピングを構築
        let user_mapping = Self::build_user_mapping(context, &user_sort, &abac_data.users);
        let resource_mapping = Self::build_resource_mapping(context, &resource_sort, &abac_data.resources);
        let position_mapping = Self::build_position_mapping(context, &position_sort);
        let department_mapping = Self::build_department_mapping(context, &department_sort);
        let course_mapping = Self::build_course_mapping(context, &course_sort);
        let resource_type_mapping = Self::build_resource_type_mapping(context, &resource_type_sort);
        let action_mapping = Self::build_action_mapping(context, &action_sort);
        
        Self {
            context,
            solver,
            user_sort,
            resource_sort,
            position_sort,
            department_sort,
            course_sort,
            resource_type_sort,
            action_sort,
            user_has_position,
            user_has_department,
            user_has_taken_course,
            user_has_taught_course,
            user_is_chair,
            resource_has_type,
            resource_has_student,
            resource_has_department,
            resource_has_course,
            user_can_perform_action,
            user_mapping,
            resource_mapping,
            position_mapping,
            department_mapping,
            course_mapping,
            resource_type_mapping,
            action_mapping,
        }
    }
    
    /// ユーザーのデータ型を構築
    fn build_user_sort(context: &'ctx Context, users: &[UniversityUserAttribute]) -> z3::DatatypeSort<'ctx> {
        let mut builder = DatatypeBuilder::new(context, "User");
        
        for user in users {
            builder = builder.variant(&user.user_id, vec![]);
        }
        
        builder.finish()
    }
    
    /// リソースのデータ型を構築
    fn build_resource_sort(context: &'ctx Context, resources: &[UniversityResourceAttribute]) -> z3::DatatypeSort<'ctx> {
        let mut builder = DatatypeBuilder::new(context, "Resource");
        
        for resource in resources {
            builder = builder.variant(&resource.resource_id, vec![]);
        }
        
        builder.finish()
    }
    
    /// 役職のデータ型を構築
    fn build_position_sort(context: &'ctx Context) -> z3::DatatypeSort<'ctx> {
        DatatypeBuilder::new(context, "Position")
            .variant("applicant", vec![])
            .variant("student", vec![])
            .variant("faculty", vec![])
            .variant("staff", vec![])
            .finish()
    }
    
    /// 部署のデータ型を構築
    fn build_department_sort(context: &'ctx Context) -> z3::DatatypeSort<'ctx> {
        DatatypeBuilder::new(context, "Department")
            .variant("cs", vec![])
            .variant("ee", vec![])
            .variant("registrar", vec![])
            .variant("admissions", vec![])
            .finish()
    }
    
    /// コースのデータ型を構築
    fn build_course_sort(context: &'ctx Context) -> z3::DatatypeSort<'ctx> {
        DatatypeBuilder::new(context, "Course")
            .variant("cs101", vec![])
            .variant("cs601", vec![])
            .variant("cs602", vec![])
            .variant("ee101", vec![])
            .variant("ee601", vec![])
            .variant("ee602", vec![])
            .finish()
    }
    
    /// リソースタイプのデータ型を構築
    fn build_resource_type_sort(context: &'ctx Context) -> z3::DatatypeSort<'ctx> {
        DatatypeBuilder::new(context, "ResourceType")
            .variant("application", vec![])
            .variant("gradebook", vec![])
            .variant("roster", vec![])
            .variant("transcript", vec![])
            .finish()
    }
    
    /// アクションのデータ型を構築
    fn build_action_sort(context: &'ctx Context) -> z3::DatatypeSort<'ctx> {
        DatatypeBuilder::new(context, "Action")
            .variant("readMyScores", vec![])
            .variant("addScore", vec![])
            .variant("readScore", vec![])
            .variant("changeScore", vec![])
            .variant("assignGrade", vec![])
            .variant("read", vec![])
            .variant("write", vec![])
            .variant("checkStatus", vec![])
            .variant("setStatus", vec![])
            .finish()
    }
    
    /// ユーザーマッピングを構築
    fn build_user_mapping(
        context: &'ctx Context, 
        user_sort: &z3::DatatypeSort<'ctx>, 
        users: &[UniversityUserAttribute]
    ) -> HashMap<String, Dynamic<'ctx>> {
        let mut mapping = HashMap::new();
        
        for (i, user) in users.iter().enumerate() {
            let user_const = user_sort.variants[i].constructor.apply(&[]);
            mapping.insert(user.user_id.clone(), user_const);
        }
        
        mapping
    }
    
    /// リソースマッピングを構築
    fn build_resource_mapping(
        context: &'ctx Context, 
        resource_sort: &z3::DatatypeSort<'ctx>, 
        resources: &[UniversityResourceAttribute]
    ) -> HashMap<String, Dynamic<'ctx>> {
        let mut mapping = HashMap::new();
        
        for (i, resource) in resources.iter().enumerate() {
            let resource_const = resource_sort.variants[i].constructor.apply(&[]);
            mapping.insert(resource.resource_id.clone(), resource_const);
        }
        
        mapping
    }
    
    /// 役職マッピングを構築
    fn build_position_mapping(
        context: &'ctx Context, 
        position_sort: &z3::DatatypeSort<'ctx>
    ) -> HashMap<Position, Dynamic<'ctx>> {
        let mut mapping = HashMap::new();
        
        mapping.insert(Position::Applicant, position_sort.variants[0].constructor.apply(&[]));
        mapping.insert(Position::Student, position_sort.variants[1].constructor.apply(&[]));
        mapping.insert(Position::Faculty, position_sort.variants[2].constructor.apply(&[]));
        mapping.insert(Position::Staff, position_sort.variants[3].constructor.apply(&[]));
        
        mapping
    }
    
    /// 部署マッピングを構築
    fn build_department_mapping(
        context: &'ctx Context, 
        department_sort: &z3::DatatypeSort<'ctx>
    ) -> HashMap<Department, Dynamic<'ctx>> {
        let mut mapping = HashMap::new();
        
        mapping.insert(Department::Cs, department_sort.variants[0].constructor.apply(&[]));
        mapping.insert(Department::Ee, department_sort.variants[1].constructor.apply(&[]));
        mapping.insert(Department::Registrar, department_sort.variants[2].constructor.apply(&[]));
        mapping.insert(Department::Admissions, department_sort.variants[3].constructor.apply(&[]));
        
        mapping
    }
    
    /// コースマッピングを構築
    fn build_course_mapping(
        context: &'ctx Context, 
        course_sort: &z3::DatatypeSort<'ctx>
    ) -> HashMap<Course, Dynamic<'ctx>> {
        let mut mapping = HashMap::new();
        
        mapping.insert(Course::Cs101, course_sort.variants[0].constructor.apply(&[]));
        mapping.insert(Course::Cs601, course_sort.variants[1].constructor.apply(&[]));
        mapping.insert(Course::Cs602, course_sort.variants[2].constructor.apply(&[]));
        mapping.insert(Course::Ee101, course_sort.variants[3].constructor.apply(&[]));
        mapping.insert(Course::Ee601, course_sort.variants[4].constructor.apply(&[]));
        mapping.insert(Course::Ee602, course_sort.variants[5].constructor.apply(&[]));
        
        mapping
    }
    
    /// リソースタイプマッピングを構築
    fn build_resource_type_mapping(
        context: &'ctx Context, 
        resource_type_sort: &z3::DatatypeSort<'ctx>
    ) -> HashMap<ResourceType, Dynamic<'ctx>> {
        let mut mapping = HashMap::new();
        
        mapping.insert(ResourceType::Application, resource_type_sort.variants[0].constructor.apply(&[]));
        mapping.insert(ResourceType::Gradebook, resource_type_sort.variants[1].constructor.apply(&[]));
        mapping.insert(ResourceType::Roster, resource_type_sort.variants[2].constructor.apply(&[]));
        mapping.insert(ResourceType::Transcript, resource_type_sort.variants[3].constructor.apply(&[]));
        
        mapping
    }
    
    /// アクションマッピングを構築
    fn build_action_mapping(
        context: &'ctx Context, 
        action_sort: &z3::DatatypeSort<'ctx>
    ) -> HashMap<Action, Dynamic<'ctx>> {
        let mut mapping = HashMap::new();
        
        mapping.insert(Action::ReadMyScores, action_sort.variants[0].constructor.apply(&[]));
        mapping.insert(Action::AddScore, action_sort.variants[1].constructor.apply(&[]));
        mapping.insert(Action::ReadScore, action_sort.variants[2].constructor.apply(&[]));
        mapping.insert(Action::ChangeScore, action_sort.variants[3].constructor.apply(&[]));
        mapping.insert(Action::AssignGrade, action_sort.variants[4].constructor.apply(&[]));
        mapping.insert(Action::Read, action_sort.variants[5].constructor.apply(&[]));
        mapping.insert(Action::Write, action_sort.variants[6].constructor.apply(&[]));
        mapping.insert(Action::CheckStatus, action_sort.variants[7].constructor.apply(&[]));
        mapping.insert(Action::SetStatus, action_sort.variants[8].constructor.apply(&[]));
        
        mapping
    }
    
    /// ABACデータから制約を生成
    pub fn generate_constraints(&mut self, abac_data: &UniversityAbacData) -> Result<(), String> {
        // ユーザー属性の制約を生成
        self.generate_user_constraints(&abac_data.users)?;
        
        // リソース属性の制約を生成
        self.generate_resource_constraints(&abac_data.resources)?;
        
        // ルールの制約を生成
        self.generate_rule_constraints(&abac_data.rules)?;
        
        Ok(())
    }
    
    /// ユーザー属性の制約を生成
    fn generate_user_constraints(&mut self, users: &[UniversityUserAttribute]) -> Result<(), String> {
        for user in users {
            let user_const = self.user_mapping.get(&user.user_id)
                .ok_or_else(|| format!("User not found: {}", user.user_id))?;
            
            // 役職の制約
            if let Some(position) = &user.position {
                let position_const = self.position_mapping.get(position)
                    .ok_or_else(|| format!("Position not found: {:?}", position))?;
                
                let constraint = z3::ast::Bool::try_from(
                    self.user_has_position.apply(&[user_const, position_const])
                ).map_err(|_| "Failed to create position constraint")?;
                
                self.solver.assert(&constraint);
            }
            
            // 部署の制約
            if let Some(department) = &user.department {
                let department_const = self.department_mapping.get(department)
                    .ok_or_else(|| format!("Department not found: {:?}", department))?;
                
                let constraint = z3::ast::Bool::try_from(
                    self.user_has_department.apply(&[user_const, department_const])
                ).map_err(|_| "Failed to create department constraint")?;
                
                self.solver.assert(&constraint);
            }
            
            // 受講コースの制約
            for course in &user.crs_taken {
                let course_const = self.course_mapping.get(course)
                    .ok_or_else(|| format!("Course not found: {:?}", course))?;
                
                let constraint = z3::ast::Bool::try_from(
                    self.user_has_taken_course.apply(&[user_const, course_const])
                ).map_err(|_| "Failed to create course taken constraint")?;
                
                self.solver.assert(&constraint);
            }
            
            // 教授コースの制約
            for course in &user.crs_taught {
                let course_const = self.course_mapping.get(course)
                    .ok_or_else(|| format!("Course not found: {:?}", course))?;
                
                let constraint = z3::ast::Bool::try_from(
                    self.user_has_taught_course.apply(&[user_const, course_const])
                ).map_err(|_| "Failed to create course taught constraint")?;
                
                self.solver.assert(&constraint);
            }
            
            // 学部長の制約
            if let Some(is_chair) = user.is_chair {
                let constraint = z3::ast::Bool::try_from(
                    self.user_is_chair.apply(&[user_const])
                ).map_err(|_| "Failed to create chair constraint")?;
                
                if is_chair {
                    self.solver.assert(&constraint);
                } else {
                    self.solver.assert(&constraint.not());
                }
            }
        }
        
        Ok(())
    }
    
    /// リソース属性の制約を生成
    fn generate_resource_constraints(&mut self, resources: &[UniversityResourceAttribute]) -> Result<(), String> {
        for resource in resources {
            let resource_const = self.resource_mapping.get(&resource.resource_id)
                .ok_or_else(|| format!("Resource not found: {}", resource.resource_id))?;
            
            // リソースタイプの制約
            let resource_type_const = self.resource_type_mapping.get(&resource.resource_type)
                .ok_or_else(|| format!("Resource type not found: {:?}", resource.resource_type))?;
            
            let type_constraint = z3::ast::Bool::try_from(
                self.resource_has_type.apply(&[resource_const, resource_type_const])
            ).map_err(|_| "Failed to create resource type constraint")?;
            
            self.solver.assert(&type_constraint);
            
            // 学生の制約
            if let Some(student_id) = &resource.student {
                if let Some(student_const) = self.user_mapping.get(student_id) {
                    let student_constraint = z3::ast::Bool::try_from(
                        self.resource_has_student.apply(&[resource_const, student_const])
                    ).map_err(|_| "Failed to create student constraint")?;
                    
                    self.solver.assert(&student_constraint);
                }
            }
            
            // 部署の制約
            for department in &resource.departments {
                let department_const = self.department_mapping.get(department)
                    .ok_or_else(|| format!("Department not found: {:?}", department))?;
                
                let dept_constraint = z3::ast::Bool::try_from(
                    self.resource_has_department.apply(&[resource_const, department_const])
                ).map_err(|_| "Failed to create resource department constraint")?;
                
                self.solver.assert(&dept_constraint);
            }
            
            // コースの制約
            if let Some(course) = &resource.crs {
                let course_const = self.course_mapping.get(course)
                    .ok_or_else(|| format!("Course not found: {:?}", course))?;
                
                let course_constraint = z3::ast::Bool::try_from(
                    self.resource_has_course.apply(&[resource_const, course_const])
                ).map_err(|_| "Failed to create resource course constraint")?;
                
                self.solver.assert(&course_constraint);
            }
        }
        
        Ok(())
    }
    
    /// ルールの制約を生成
    fn generate_rule_constraints(&mut self, rules: &[UniversityRule]) -> Result<(), String> {
        for rule in rules {
            println!("Processing rule {}: {}", rule.id, rule.description);
            
            // ルールの各セクションを処理
            let rule_constraint = self.generate_single_rule_constraint(rule)?;
            
            // 生成された制約をソルバーに追加
            self.solver.assert(&rule_constraint);
        }
        
        Ok(())
    }
    
    /// 単一のルールから制約を生成
    fn generate_single_rule_constraint(&mut self, rule: &UniversityRule) -> Result<z3::ast::Bool<'ctx>, String> {
        // 変数を作成
        let user_var_name = format!("u_{}", rule.id);
        let resource_var_name = format!("r_{}", rule.id);
        let user_var = Dynamic::new_const(self.context, user_var_name.as_str(), &self.user_sort.sort);
        let resource_var = Dynamic::new_const(self.context, resource_var_name.as_str(), &self.resource_sort.sort);
        
        // 条件を評価
        let mut conditions = Vec::new();
        
        // ユーザー条件
        for user_condition in &rule.user_conditions {
            let condition = self.evaluate_condition(&user_var, &resource_var, user_condition, true)?;
            conditions.push(condition);
        }
        
        // リソース条件
        for resource_condition in &rule.resource_conditions {
            let condition = self.evaluate_condition(&user_var, &resource_var, resource_condition, false)?;
            conditions.push(condition);
        }
        
        // 比較条件
        for comparison_condition in &rule.comparison_conditions {
            let condition = self.evaluate_comparison_condition(&user_var, &resource_var, comparison_condition)?;
            conditions.push(condition);
        }
        
        // アクション条件
        let action_conditions = self.evaluate_action_conditions(&user_var, &resource_var, &rule.actions)?;
        conditions.extend(action_conditions);
        
        // 全ての条件をANDで結合
        let combined_conditions = self.combine_conditions_with_and(&conditions)?;
        
        // ルールが満たされた場合のアクセス許可
        let access_permission = self.create_access_permission(&user_var, &resource_var, &rule.actions)?;
        
        // 条件 => アクセス許可 の形で制約を作成
        let rule_constraint = combined_conditions.implies(&access_permission);
        
        Ok(rule_constraint)
    }
    
    /// 条件を評価してZ3の制約を生成
    fn evaluate_condition(
        &self, 
        user_var: &Dynamic<'ctx>,
        resource_var: &Dynamic<'ctx>,
        condition: &Condition,
        is_user_context: bool
    ) -> Result<z3::ast::Bool<'ctx>, String> {
        match (&condition.left, &condition.operator, &condition.right) {
            // 属性名 [ 値セット の形式
            (AttributeExpression::AttributeName(attr_name), ComparisonOperator::ContainedIn, AttributeExpression::ValueSet(values)) => {
                self.evaluate_containment_condition(user_var, resource_var, attr_name, values, is_user_context)
            }
            // 属性名 ] 値セット の形式
            (AttributeExpression::AttributeName(attr_name), ComparisonOperator::Contains, AttributeExpression::ValueSet(values)) => {
                self.evaluate_contains_condition(user_var, resource_var, attr_name, values, is_user_context)
            }
            // 属性名 = 値 の形式
            (AttributeExpression::AttributeName(attr_name), ComparisonOperator::Equals, AttributeExpression::AttributeValue(value)) => {
                self.evaluate_equality_condition(user_var, resource_var, attr_name, value, is_user_context)
            }
            // 属性名 = 属性名 の形式
            (AttributeExpression::AttributeName(left_attr), ComparisonOperator::Equals, AttributeExpression::AttributeName(right_attr)) => {
                self.evaluate_attribute_equality_condition(user_var, resource_var, left_attr, right_attr)
            }
            // 属性名 ] 属性名 の形式 (Contains)
            (AttributeExpression::AttributeName(left_attr), ComparisonOperator::Contains, AttributeExpression::AttributeName(right_attr)) => {
                self.evaluate_attribute_equality_condition(user_var, resource_var, left_attr, right_attr)
            }
            // 属性名 [ 属性名 の形式 (ContainedIn)
            (AttributeExpression::AttributeName(left_attr), ComparisonOperator::ContainedIn, AttributeExpression::AttributeName(right_attr)) => {
                self.evaluate_attribute_equality_condition(user_var, resource_var, left_attr, right_attr)
            }
            _ => {
                Err(format!("Unsupported condition pattern: {:?}", condition))
            }
        }
    }
    
    /// 包含条件を評価 (属性 [ 値セット)
    fn evaluate_containment_condition(
        &self,
        user_var: &Dynamic<'ctx>,
        resource_var: &Dynamic<'ctx>,
        attr_name: &AttributeName,
        values: &[AttributeValue],
        is_user_context: bool
    ) -> Result<z3::ast::Bool<'ctx>, String> {
        let mut disjuncts = Vec::new();
        
        for value in values {
            let condition = self.create_attribute_condition(user_var, resource_var, attr_name, value, is_user_context)?;
            disjuncts.push(condition);
        }
        
        self.combine_conditions_with_or(&disjuncts)
    }
    
    /// 包含条件を評価 (属性 ] 値セット)
    fn evaluate_contains_condition(
        &self,
        user_var: &Dynamic<'ctx>,
        resource_var: &Dynamic<'ctx>,
        attr_name: &AttributeName,
        values: &[AttributeValue],
        is_user_context: bool
    ) -> Result<z3::ast::Bool<'ctx>, String> {
        // ] 演算子は通常、左側の属性が右側の値セットを含むことを意味する
        // 今回は簡単のため [ と同じ処理をする
        self.evaluate_containment_condition(user_var, resource_var, attr_name, values, is_user_context)
    }
    
    /// 等価条件を評価
    fn evaluate_equality_condition(
        &self,
        user_var: &Dynamic<'ctx>,
        resource_var: &Dynamic<'ctx>,
        attr_name: &AttributeName,
        value: &AttributeValue,
        is_user_context: bool
    ) -> Result<z3::ast::Bool<'ctx>, String> {
        self.create_attribute_condition(user_var, resource_var, attr_name, value, is_user_context)
    }
    
    /// 属性同士の等価条件を評価
    fn evaluate_attribute_equality_condition(
        &self,
        user_var: &Dynamic<'ctx>,
        resource_var: &Dynamic<'ctx>,
        left_attr: &AttributeName,
        right_attr: &AttributeName
    ) -> Result<z3::ast::Bool<'ctx>, String> {
        // 例: crsTaught ] crs の場合
        // ユーザーが教えるコース = リソースのコース
        match (left_attr, right_attr) {
            (AttributeName::CrsTaught, AttributeName::Crs) => {
                // ユーザーが教えるコースとリソースのコースが一致する
                let course_var = Dynamic::new_const(self.context, "course_match", &self.course_sort.sort);
                
                let user_teaches_course = z3::ast::Bool::try_from(
                    self.user_has_taught_course.apply(&[user_var, &course_var])
                ).map_err(|_| "Failed to create user teaches course constraint")?;
                
                let resource_has_course = z3::ast::Bool::try_from(
                    self.resource_has_course.apply(&[resource_var, &course_var])
                ).map_err(|_| "Failed to create resource has course constraint")?;
                
                Ok(user_teaches_course & resource_has_course)
            }
            (AttributeName::CrsTaken, AttributeName::Crs) => {
                // ユーザーが受講したコースとリソースのコースが一致する
                let course_var = Dynamic::new_const(self.context, "course_match", &self.course_sort.sort);
                
                let user_taken_course = z3::ast::Bool::try_from(
                    self.user_has_taken_course.apply(&[user_var, &course_var])
                ).map_err(|_| "Failed to create user taken course constraint")?;
                
                let resource_has_course = z3::ast::Bool::try_from(
                    self.resource_has_course.apply(&[resource_var, &course_var])
                ).map_err(|_| "Failed to create resource has course constraint")?;
                
                Ok(user_taken_course & resource_has_course)
            }
            (AttributeName::Uid, AttributeName::Student) => {
                // ユーザーIDがリソースの学生と一致する
                let student_matches = z3::ast::Bool::try_from(
                    self.resource_has_student.apply(&[resource_var, user_var])
                ).map_err(|_| "Failed to create student match constraint")?;
                
                Ok(student_matches)
            }
            (AttributeName::Department, AttributeName::Departments) => {
                // ユーザーの部署がリソースの部署に含まれる
                // 例: 学部長がその部署の学生の成績表にアクセスする場合
                let department_var = Dynamic::new_const(self.context, "department_match", &self.department_sort.sort);
                
                let user_has_department = z3::ast::Bool::try_from(
                    self.user_has_department.apply(&[user_var, &department_var])
                ).map_err(|_| "Failed to create user department constraint")?;
                
                let resource_has_department = z3::ast::Bool::try_from(
                    self.resource_has_department.apply(&[resource_var, &department_var])
                ).map_err(|_| "Failed to create resource department constraint")?;
                
                Ok(user_has_department & resource_has_department)
            }
            _ => {
                Err(format!("Unsupported attribute equality: {:?} = {:?}", left_attr, right_attr))
            }
        }
    }
    
    /// 属性条件を作成
    fn create_attribute_condition(
        &self,
        user_var: &Dynamic<'ctx>,
        resource_var: &Dynamic<'ctx>,
        attr_name: &AttributeName,
        value: &AttributeValue,
        is_user_context: bool
    ) -> Result<z3::ast::Bool<'ctx>, String> {
        match attr_name {
            AttributeName::Position => {
                if let AttributeValue::Position(pos) = value {
                    let position_const = self.position_mapping.get(pos)
                        .ok_or_else(|| format!("Position not found: {:?}", pos))?;
                    
                    let condition = z3::ast::Bool::try_from(
                        self.user_has_position.apply(&[user_var, position_const])
                    ).map_err(|_| "Failed to create position condition")?;
                    
                    Ok(condition)
                } else {
                    Err(format!("Invalid value type for position: {:?}", value))
                }
            }
            AttributeName::Department => {
                if let AttributeValue::Department(dept) = value {
                    let department_const = self.department_mapping.get(dept)
                        .ok_or_else(|| format!("Department not found: {:?}", dept))?;
                    
                    let condition = if is_user_context {
                        z3::ast::Bool::try_from(
                            self.user_has_department.apply(&[user_var, department_const])
                        ).map_err(|_| "Failed to create user department condition")?
                    } else {
                        z3::ast::Bool::try_from(
                            self.resource_has_department.apply(&[resource_var, department_const])
                        ).map_err(|_| "Failed to create resource department condition")?
                    };
                    
                    Ok(condition)
                } else {
                    Err(format!("Invalid value type for department: {:?}", value))
                }
            }
            AttributeName::Type => {
                if let AttributeValue::ResourceType(rt) = value {
                    let resource_type_const = self.resource_type_mapping.get(rt)
                        .ok_or_else(|| format!("Resource type not found: {:?}", rt))?;
                    
                    let condition = z3::ast::Bool::try_from(
                        self.resource_has_type.apply(&[resource_var, resource_type_const])
                    ).map_err(|_| "Failed to create resource type condition")?;
                    
                    Ok(condition)
                } else {
                    Err(format!("Invalid value type for resource type: {:?}", value))
                }
            }
            AttributeName::IsChair => {
                if let AttributeValue::Boolean(is_chair) = value {
                    let condition = z3::ast::Bool::try_from(
                        self.user_is_chair.apply(&[user_var])
                    ).map_err(|_| "Failed to create chair condition")?;
                    
                    if *is_chair {
                        Ok(condition)
                    } else {
                        Ok(condition.not())
                    }
                } else {
                    Err(format!("Invalid value type for isChair: {:?}", value))
                }
            }
            AttributeName::CrsTaken => {
                if let AttributeValue::Course(course) = value {
                    let course_const = self.course_mapping.get(course)
                        .ok_or_else(|| format!("Course not found: {:?}", course))?;
                    
                    let condition = z3::ast::Bool::try_from(
                        self.user_has_taken_course.apply(&[user_var, course_const])
                    ).map_err(|_| "Failed to create course taken condition")?;
                    
                    Ok(condition)
                } else {
                    Err(format!("Invalid value type for crsTaken: {:?}", value))
                }
            }
            AttributeName::CrsTaught => {
                if let AttributeValue::Course(course) = value {
                    let course_const = self.course_mapping.get(course)
                        .ok_or_else(|| format!("Course not found: {:?}", course))?;
                    
                    let condition = z3::ast::Bool::try_from(
                        self.user_has_taught_course.apply(&[user_var, course_const])
                    ).map_err(|_| "Failed to create course taught condition")?;
                    
                    Ok(condition)
                } else {
                    Err(format!("Invalid value type for crsTaught: {:?}", value))
                }
            }
            _ => {
                Err(format!("Unsupported attribute: {:?}", attr_name))
            }
        }
    }
    
    /// 比較条件を評価
    fn evaluate_comparison_condition(
        &self,
        user_var: &Dynamic<'ctx>,
        resource_var: &Dynamic<'ctx>,
        condition: &Condition
    ) -> Result<z3::ast::Bool<'ctx>, String> {
        // 比較条件は通常、異なる属性間の関係を表現
        self.evaluate_condition(user_var, resource_var, condition, false)
    }
    
    /// アクション条件を評価
    fn evaluate_action_conditions(
        &self,
        user_var: &Dynamic<'ctx>,
        resource_var: &Dynamic<'ctx>,
        actions: &HashSet<Action>
    ) -> Result<Vec<z3::ast::Bool<'ctx>>, String> {
        let mut conditions = Vec::new();
        
        for action in actions {
            let action_const = self.action_mapping.get(action)
                .ok_or_else(|| format!("Action not found: {:?}", action))?;
            
            let condition = z3::ast::Bool::try_from(
                self.user_can_perform_action.apply(&[user_var, resource_var, action_const])
            ).map_err(|_| "Failed to create action condition")?;
            
            conditions.push(condition);
        }
        
        Ok(conditions)
    }
    
    /// アクセス許可を作成
    fn create_access_permission(
        &self,
        user_var: &Dynamic<'ctx>,
        resource_var: &Dynamic<'ctx>,
        actions: &HashSet<Action>
    ) -> Result<z3::ast::Bool<'ctx>, String> {
        let mut permissions = Vec::new();
        
        for action in actions {
            let action_const = self.action_mapping.get(action)
                .ok_or_else(|| format!("Action not found: {:?}", action))?;
            
            let permission = z3::ast::Bool::try_from(
                self.user_can_perform_action.apply(&[user_var, resource_var, action_const])
            ).map_err(|_| "Failed to create access permission")?;
            
            permissions.push(permission);
        }
        
        // 全てのアクションを許可
        self.combine_conditions_with_and(&permissions)
    }
    
    /// 条件をANDで結合
    fn combine_conditions_with_and(&self, conditions: &[z3::ast::Bool<'ctx>]) -> Result<z3::ast::Bool<'ctx>, String> {
        if conditions.is_empty() {
            return Ok(z3::ast::Bool::from_bool(self.context, true));
        }
        
        let mut result = conditions[0].clone();
        for condition in &conditions[1..] {
            result = result & condition.clone();
        }
        
        Ok(result)
    }
    
    /// 条件をORで結合
    fn combine_conditions_with_or(&self, conditions: &[z3::ast::Bool<'ctx>]) -> Result<z3::ast::Bool<'ctx>, String> {
        if conditions.is_empty() {
            return Ok(z3::ast::Bool::from_bool(self.context, false));
        }
        
        let mut result = conditions[0].clone();
        for condition in &conditions[1..] {
            result = result | condition.clone();
        }
        
        Ok(result)
    }
    
    /// 各ルール別にアクセス制御の解析を実行
    pub fn solve_access_control_by_rules(&mut self, rules: &[UniversityRule]) -> Result<Vec<RuleAccessResults>, String> {
        let mut rule_results = Vec::new();
        
        for rule in rules {
            println!("Analyzing Rule {}: {}", rule.id + 1, rule.description);
            
            // 新しいソルバーを作成してルール固有の制約のみを追加
            let rule_solver = Solver::new(self.context);
            
            // 基本的な属性制約を追加（ユーザーとリソースの属性）
            self.add_basic_constraints_to_solver(&rule_solver)?;
            
            // 特定のルールの制約のみを追加
            let rule_constraint = self.generate_single_rule_constraint(rule)?;
            rule_solver.assert(&rule_constraint);
            
            // このルールの下でのアクセス可能な組み合わせを探索
            let access_results = self.find_rule_access_combinations(&rule_solver, rule)?;
            
            rule_results.push(RuleAccessResults {
                rule_id: rule.id,
                rule_description: rule.description.clone(),
                access_combinations: access_results,
            });
        }
        
        Ok(rule_results)
    }
    
    /// ルール固有のアクセス組み合わせを探索
    fn find_rule_access_combinations(&self, solver: &Solver, rule: &UniversityRule) -> Result<Vec<AccessResult>, String> {
        let mut results = Vec::new();
        
        // 解析用の変数を作成
        let u_var = Dynamic::new_const(self.context, "u_rule", &self.user_sort.sort);
        let r_var = Dynamic::new_const(self.context, "r_rule", &self.resource_sort.sort);
        
        // このルールのアクションそれぞれについて確認
        for action in &rule.actions {
            let action_const = self.action_mapping.get(action)
                .ok_or_else(|| format!("Action not found: {:?}", action))?;
            
            // アクセス制御の条件を追加
            let access_constraint = z3::ast::Bool::try_from(
                self.user_can_perform_action.apply(&[&u_var, &r_var, action_const])
            ).map_err(|_| "Failed to create access constraint")?;
            
            solver.assert(&access_constraint);
            
            // 解を探索
            let mut solution_count = 0;
            loop {
                match solver.check() {
                    z3::SatResult::Sat => {
                        let model = solver.get_model()
                            .ok_or_else(|| "Failed to get model")?;
                        
                        let user_val = model.eval(&u_var, true)
                            .ok_or_else(|| "Failed to evaluate user")?;
                        let resource_val = model.eval(&r_var, true)
                            .ok_or_else(|| "Failed to evaluate resource")?;
                        
                        results.push(AccessResult {
                            user: user_val.to_string(),
                            resource: resource_val.to_string(),
                            action: format!("{:?}", action),
                        });
                        
                        // 次の解を探すため、現在の解を禁止
                        let block_constraint = u_var._eq(&user_val) & r_var._eq(&resource_val);
                        solver.assert(&block_constraint.not());
                        
                        solution_count += 1;
                        if solution_count > 50 { // 無限ループを防ぐ
                            break;
                        }
                    }
                    z3::SatResult::Unsat => {
                        break;
                    }
                    z3::SatResult::Unknown => {
                        break;
                    }
                }
            }
            
            // この制約を削除して次のアクションを試す
            solver.pop(1);
        }
        
        Ok(results)
    }
    
    /// 基本的な属性制約をソルバーに追加
    fn add_basic_constraints_to_solver(&self, solver: &Solver) -> Result<(), String> {
        // 実装は簡略化 - 実際にはユーザーとリソースの属性制約を追加
        Ok(())
    }
    
    /// アクセス制御の解析を実行
    pub fn solve_access_control(&mut self) -> Result<Vec<AccessResult>, String> {
        let mut results = Vec::new();
        
        // 解析用の変数を作成
        let u_var = Dynamic::new_const(self.context, "u", &self.user_sort.sort);
        let r_var = Dynamic::new_const(self.context, "r", &self.resource_sort.sort);
        let a_var = Dynamic::new_const(self.context, "a", &self.action_sort.sort);
        
        // アクセス制御の条件を追加
        let access_constraint = z3::ast::Bool::try_from(
            self.user_can_perform_action.apply(&[&u_var, &r_var, &a_var])
        ).map_err(|_| "Failed to create access constraint")?;
        
        self.solver.assert(&access_constraint);
        
        // 解を探索
        let mut solution_count = 0;
        loop {
            match self.solver.check() {
                z3::SatResult::Sat => {
                    let model = self.solver.get_model()
                        .ok_or_else(|| "Failed to get model")?;
                    
                    let user_val = model.eval(&u_var, true)
                        .ok_or_else(|| "Failed to evaluate user")?;
                    let resource_val = model.eval(&r_var, true)
                        .ok_or_else(|| "Failed to evaluate resource")?;
                    let action_val = model.eval(&a_var, true)
                        .ok_or_else(|| "Failed to evaluate action")?;
                    
                    results.push(AccessResult {
                        user: user_val.to_string(),
                        resource: resource_val.to_string(),
                        action: action_val.to_string(),
                    });
                    
                    // 次の解を探すため、現在の解を禁止
                    let block_constraint = u_var._eq(&user_val) & r_var._eq(&resource_val) & a_var._eq(&action_val);
                    self.solver.assert(&block_constraint.not());
                    
                    solution_count += 1;
                    if solution_count > 100 { // 無限ループを防ぐ
                        break;
                    }
                }
                z3::SatResult::Unsat => {
                    break;
                }
                z3::SatResult::Unknown => {
                    return Err("Solver returned unknown".to_string());
                }
            }
        }
        
        Ok(results)
    }
}

/// アクセス制御の結果を表す構造体
#[derive(Debug, Clone)]
pub struct AccessResult {
    pub user: String,
    pub resource: String,
    pub action: String,
}

/// ルール別のアクセス制御結果を表す構造体
#[derive(Debug, Clone)]
pub struct RuleAccessResults {
    pub rule_id: usize,
    pub rule_description: String,
    pub access_combinations: Vec<AccessResult>,
}

impl AccessResult {
    /// 結果を人間が読める形式で表示
    pub fn display(&self) -> String {
        format!("User '{}' can perform '{}' on resource '{}'", 
                self.user, self.action, self.resource)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::UniversityAbacParser;
    
    #[test]
    fn test_abac_solver_creation() {
        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        
        let parser = UniversityAbacParser::new();
        let sample_data = r#"
            userAttrib(alice, position=student, department=cs)
            resourceAttrib(cs101grade, type=gradebook, crs=cs101)
        "#;
        
        let abac_data = parser.parse(sample_data).unwrap();
        let _solver = UniversityAbacSolver::new(&ctx, &abac_data);
        
        // ソルバーが正常に作成されることを確認
        assert_eq!(abac_data.users.len(), 1);
        assert_eq!(abac_data.resources.len(), 1);
    }
} 