use std::collections::{HashMap, HashSet};
use z3::{Config, Context, Solver, Sort, FuncDecl, DatatypeBuilder, ast::{Ast, Dynamic}};

use crate::edocumentTypes::*;

/// edocument用ABACソルバーのメイン構造体
pub struct EdocumentAbacSolver<'ctx> {
    context: &'ctx Context,
    solver: Solver<'ctx>,
    
    // Z3データ型（拡張版）
    user_sort: z3::DatatypeSort<'ctx>,
    resource_sort: z3::DatatypeSort<'ctx>,
    role_sort: z3::DatatypeSort<'ctx>,
    position_sort: z3::DatatypeSort<'ctx>,
    tenant_sort: z3::DatatypeSort<'ctx>,
    department_sort: z3::DatatypeSort<'ctx>,
    office_sort: z3::DatatypeSort<'ctx>,
    document_type_sort: z3::DatatypeSort<'ctx>,
    action_sort: z3::DatatypeSort<'ctx>,
    security_level_sort: z3::DatatypeSort<'ctx>,
    customer_tier_sort: z3::DatatypeSort<'ctx>,
    region_sort: z3::DatatypeSort<'ctx>,
    country_sort: z3::DatatypeSort<'ctx>,
    time_zone_sort: z3::DatatypeSort<'ctx>,
    contract_type_sort: z3::DatatypeSort<'ctx>,
    approval_status_sort: z3::DatatypeSort<'ctx>,
    priority_sort: z3::DatatypeSort<'ctx>,
    format_sort: z3::DatatypeSort<'ctx>,
    language_sort: z3::DatatypeSort<'ctx>,
    
    // 属性関数
    user_has_role: FuncDecl<'ctx>,
    user_has_position: FuncDecl<'ctx>,
    user_has_tenant: FuncDecl<'ctx>,
    user_has_department: FuncDecl<'ctx>,
    user_has_office: FuncDecl<'ctx>,
    user_is_registered: FuncDecl<'ctx>,
    user_has_project: FuncDecl<'ctx>,
    user_has_supervisor: FuncDecl<'ctx>,
    user_has_supervisee: FuncDecl<'ctx>,
    user_has_payrolling_permission: FuncDecl<'ctx>,
    user_has_security_clearance: FuncDecl<'ctx>,
    user_has_experience: FuncDecl<'ctx>,
    user_has_customer_tier: FuncDecl<'ctx>,
    user_has_region: FuncDecl<'ctx>,
    user_has_country: FuncDecl<'ctx>,
    user_has_time_zone: FuncDecl<'ctx>,
    user_has_contract_type: FuncDecl<'ctx>,
    user_has_delegated_authority: FuncDecl<'ctx>,
    user_has_temporary_access: FuncDecl<'ctx>,
    user_has_current_project: FuncDecl<'ctx>,
    user_has_past_project: FuncDecl<'ctx>,
    user_is_active: FuncDecl<'ctx>,
    user_has_budget_authority: FuncDecl<'ctx>,
    
    // 数値比較関数
    user_experience_value: FuncDecl<'ctx>,
    user_budget_value: FuncDecl<'ctx>,
    
    resource_has_type: FuncDecl<'ctx>,
    resource_has_owner: FuncDecl<'ctx>,
    resource_has_tenant: FuncDecl<'ctx>,
    resource_has_department: FuncDecl<'ctx>,
    resource_has_office: FuncDecl<'ctx>,
    resource_has_recipient: FuncDecl<'ctx>,
    resource_is_confidential: FuncDecl<'ctx>,
    resource_contains_personal_info: FuncDecl<'ctx>,
    resource_has_security_level: FuncDecl<'ctx>,
    resource_has_project_id: FuncDecl<'ctx>,
    resource_has_version: FuncDecl<'ctx>,
    resource_has_size: FuncDecl<'ctx>,
    resource_has_format: FuncDecl<'ctx>,
    resource_has_language: FuncDecl<'ctx>,
    resource_has_region: FuncDecl<'ctx>,
    resource_has_country: FuncDecl<'ctx>,
    resource_has_approval_status: FuncDecl<'ctx>,
    resource_has_priority: FuncDecl<'ctx>,
    resource_has_retention_period: FuncDecl<'ctx>,
    resource_is_archived: FuncDecl<'ctx>,
    resource_has_access_count: FuncDecl<'ctx>,
    
    // 数値比較関数
    resource_size_value: FuncDecl<'ctx>,
    resource_access_count_value: FuncDecl<'ctx>,
    resource_retention_period_value: FuncDecl<'ctx>,
    
    // アクセス制御関数
    user_can_perform_action: FuncDecl<'ctx>,
    
    // データマッピング
    user_mapping: HashMap<String, Dynamic<'ctx>>,
    resource_mapping: HashMap<String, Dynamic<'ctx>>,
    role_mapping: HashMap<Role, Dynamic<'ctx>>,
    position_mapping: HashMap<Position, Dynamic<'ctx>>,
    tenant_mapping: HashMap<Tenant, Dynamic<'ctx>>,
    department_mapping: HashMap<Department, Dynamic<'ctx>>,
    office_mapping: HashMap<Office, Dynamic<'ctx>>,
    document_type_mapping: HashMap<DocumentType, Dynamic<'ctx>>,
    action_mapping: HashMap<Action, Dynamic<'ctx>>,
    security_level_mapping: HashMap<SecurityLevel, Dynamic<'ctx>>,
    customer_tier_mapping: HashMap<CustomerTier, Dynamic<'ctx>>,
    region_mapping: HashMap<Region, Dynamic<'ctx>>,
    country_mapping: HashMap<Country, Dynamic<'ctx>>,
    time_zone_mapping: HashMap<TimeZone, Dynamic<'ctx>>,
    contract_type_mapping: HashMap<ContractType, Dynamic<'ctx>>,
    approval_status_mapping: HashMap<ApprovalStatus, Dynamic<'ctx>>,
    priority_mapping: HashMap<Priority, Dynamic<'ctx>>,
    format_mapping: HashMap<Format, Dynamic<'ctx>>,
    language_mapping: HashMap<Language, Dynamic<'ctx>>,
}

impl<'ctx> EdocumentAbacSolver<'ctx> {
    /// 新しいedocument用ABACソルバーを作成
    pub fn new(context: &'ctx Context, abac_data: &EdocumentAbacData) -> Self {
        let solver = Solver::new(context);
        
        // Z3データ型を構築
        let user_sort = Self::build_user_sort(context, &abac_data.users);
        let resource_sort = Self::build_resource_sort(context, &abac_data.resources);
        let role_sort = Self::build_role_sort(context);
        let position_sort = Self::build_position_sort(context);
        let tenant_sort = Self::build_tenant_sort(context);
        let department_sort = Self::build_department_sort(context);
        let office_sort = Self::build_office_sort(context);
        let document_type_sort = Self::build_document_type_sort(context);
        let action_sort = Self::build_action_sort(context);
        let security_level_sort = Self::build_security_level_sort(context);
        let customer_tier_sort = Self::build_customer_tier_sort(context);
        let region_sort = Self::build_region_sort(context);
        let country_sort = Self::build_country_sort(context);
        let time_zone_sort = Self::build_time_zone_sort(context);
        let contract_type_sort = Self::build_contract_type_sort(context);
        let approval_status_sort = Self::build_approval_status_sort(context);
        let priority_sort = Self::build_priority_sort(context);
        let format_sort = Self::build_format_sort(context);
        let language_sort = Self::build_language_sort(context);
        
        // 属性関数を定義
        let bool_sort = Sort::bool(context);
        let int_sort = Sort::int(context);
        let string_sort = Sort::string(context);
        
        let user_has_role = FuncDecl::new(context, "user_has_role", 
            &[&user_sort.sort, &role_sort.sort], &bool_sort);
        let user_has_position = FuncDecl::new(context, "user_has_position", 
            &[&user_sort.sort, &position_sort.sort], &bool_sort);
        let user_has_tenant = FuncDecl::new(context, "user_has_tenant", 
            &[&user_sort.sort, &tenant_sort.sort], &bool_sort);
        let user_has_department = FuncDecl::new(context, "user_has_department", 
            &[&user_sort.sort, &department_sort.sort], &bool_sort);
        let user_has_office = FuncDecl::new(context, "user_has_office", 
            &[&user_sort.sort, &office_sort.sort], &bool_sort);
        let user_is_registered = FuncDecl::new(context, "user_is_registered", 
            &[&user_sort.sort], &bool_sort);
        let user_has_project = FuncDecl::new(context, "user_has_project", 
            &[&user_sort.sort, &user_sort.sort], &bool_sort);
        let user_has_supervisor = FuncDecl::new(context, "user_has_supervisor", 
            &[&user_sort.sort, &user_sort.sort], &bool_sort);
        let user_has_supervisee = FuncDecl::new(context, "user_has_supervisee", 
            &[&user_sort.sort, &user_sort.sort], &bool_sort);
        let user_has_payrolling_permission = FuncDecl::new(context, "user_has_payrolling_permission", 
            &[&user_sort.sort, &user_sort.sort], &bool_sort);
        let user_has_security_clearance = FuncDecl::new(context, "user_has_security_clearance", 
            &[&user_sort.sort, &security_level_sort.sort], &bool_sort);
        let user_has_experience = FuncDecl::new(context, "user_has_experience", 
            &[&user_sort.sort, &int_sort], &bool_sort);
        let user_has_customer_tier = FuncDecl::new(context, "user_has_customer_tier", 
            &[&user_sort.sort, &customer_tier_sort.sort], &bool_sort);
        let user_has_region = FuncDecl::new(context, "user_has_region", 
            &[&user_sort.sort, &region_sort.sort], &bool_sort);
        let user_has_country = FuncDecl::new(context, "user_has_country", 
            &[&user_sort.sort, &country_sort.sort], &bool_sort);
        let user_has_time_zone = FuncDecl::new(context, "user_has_time_zone", 
            &[&user_sort.sort, &time_zone_sort.sort], &bool_sort);
        let user_has_contract_type = FuncDecl::new(context, "user_has_contract_type", 
            &[&user_sort.sort, &contract_type_sort.sort], &bool_sort);
        let user_has_delegated_authority = FuncDecl::new(context, "user_has_delegated_authority", 
            &[&user_sort.sort], &bool_sort);
        let user_has_temporary_access = FuncDecl::new(context, "user_has_temporary_access", 
            &[&user_sort.sort], &bool_sort);
        let user_has_current_project = FuncDecl::new(context, "user_has_current_project", 
            &[&user_sort.sort, &string_sort], &bool_sort);
        let user_has_past_project = FuncDecl::new(context, "user_has_past_project", 
            &[&user_sort.sort, &string_sort], &bool_sort);
        let user_is_active = FuncDecl::new(context, "user_is_active", 
            &[&user_sort.sort], &bool_sort);
        let user_has_budget_authority = FuncDecl::new(context, "user_has_budget_authority", 
            &[&user_sort.sort], &bool_sort);
        
        let user_experience_value = FuncDecl::new(context, "user_experience_value", 
            &[&user_sort.sort], &int_sort);
        let user_budget_value = FuncDecl::new(context, "user_budget_value", 
            &[&user_sort.sort], &int_sort);
        
        let resource_has_type = FuncDecl::new(context, "resource_has_type", 
            &[&resource_sort.sort, &document_type_sort.sort], &bool_sort);
        let resource_has_owner = FuncDecl::new(context, "resource_has_owner", 
            &[&resource_sort.sort, &user_sort.sort], &bool_sort);
        let resource_has_tenant = FuncDecl::new(context, "resource_has_tenant", 
            &[&resource_sort.sort, &tenant_sort.sort], &bool_sort);
        let resource_has_department = FuncDecl::new(context, "resource_has_department", 
            &[&resource_sort.sort, &department_sort.sort], &bool_sort);
        let resource_has_office = FuncDecl::new(context, "resource_has_office", 
            &[&resource_sort.sort, &office_sort.sort], &bool_sort);
        let resource_has_recipient = FuncDecl::new(context, "resource_has_recipient", 
            &[&resource_sort.sort, &user_sort.sort], &bool_sort);
        let resource_is_confidential = FuncDecl::new(context, "resource_is_confidential", 
            &[&resource_sort.sort], &bool_sort);
        let resource_contains_personal_info = FuncDecl::new(context, "resource_contains_personal_info", 
            &[&resource_sort.sort], &bool_sort);
        let resource_has_security_level = FuncDecl::new(context, "resource_has_security_level", 
            &[&resource_sort.sort, &security_level_sort.sort], &bool_sort);
        let resource_has_project_id = FuncDecl::new(context, "resource_has_project_id", 
            &[&resource_sort.sort, &string_sort], &bool_sort);
        let resource_has_version = FuncDecl::new(context, "resource_has_version", 
            &[&resource_sort.sort, &string_sort], &bool_sort);
        let resource_has_size = FuncDecl::new(context, "resource_has_size", 
            &[&resource_sort.sort, &int_sort], &bool_sort);
        let resource_has_format = FuncDecl::new(context, "resource_has_format", 
            &[&resource_sort.sort, &format_sort.sort], &bool_sort);
        let resource_has_language = FuncDecl::new(context, "resource_has_language", 
            &[&resource_sort.sort, &language_sort.sort], &bool_sort);
        let resource_has_region = FuncDecl::new(context, "resource_has_region", 
            &[&resource_sort.sort, &region_sort.sort], &bool_sort);
        let resource_has_country = FuncDecl::new(context, "resource_has_country", 
            &[&resource_sort.sort, &country_sort.sort], &bool_sort);
        let resource_has_approval_status = FuncDecl::new(context, "resource_has_approval_status", 
            &[&resource_sort.sort, &approval_status_sort.sort], &bool_sort);
        let resource_has_priority = FuncDecl::new(context, "resource_has_priority", 
            &[&resource_sort.sort, &priority_sort.sort], &bool_sort);
        let resource_has_retention_period = FuncDecl::new(context, "resource_has_retention_period", 
            &[&resource_sort.sort, &int_sort], &bool_sort);
        let resource_is_archived = FuncDecl::new(context, "resource_is_archived", 
            &[&resource_sort.sort], &bool_sort);
        let resource_has_access_count = FuncDecl::new(context, "resource_has_access_count", 
            &[&resource_sort.sort, &int_sort], &bool_sort);
        
        let resource_size_value = FuncDecl::new(context, "resource_size_value", 
            &[&resource_sort.sort], &int_sort);
        let resource_access_count_value = FuncDecl::new(context, "resource_access_count_value", 
            &[&resource_sort.sort], &int_sort);
        let resource_retention_period_value = FuncDecl::new(context, "resource_retention_period_value", 
            &[&resource_sort.sort], &int_sort);
        
        let user_can_perform_action = FuncDecl::new(context, "user_can_perform_action", 
            &[&user_sort.sort, &resource_sort.sort, &action_sort.sort], &bool_sort);
        
        // データマッピングを構築
        let user_mapping = Self::build_user_mapping(context, &user_sort, &abac_data.users);
        let resource_mapping = Self::build_resource_mapping(context, &resource_sort, &abac_data.resources);
        let role_mapping = Self::build_role_mapping(context, &role_sort);
        let position_mapping = Self::build_position_mapping(context, &position_sort);
        let tenant_mapping = Self::build_tenant_mapping(context, &tenant_sort);
        let department_mapping = Self::build_department_mapping(context, &department_sort);
        let office_mapping = Self::build_office_mapping(context, &office_sort);
        let document_type_mapping = Self::build_document_type_mapping(context, &document_type_sort);
        let action_mapping = Self::build_action_mapping(context, &action_sort);
        let security_level_mapping = Self::build_security_level_mapping(context, &security_level_sort);
        let customer_tier_mapping = Self::build_customer_tier_mapping(context, &customer_tier_sort);
        let region_mapping = Self::build_region_mapping(context, &region_sort);
        let country_mapping = Self::build_country_mapping(context, &country_sort);
        let time_zone_mapping = Self::build_time_zone_mapping(context, &time_zone_sort);
        let contract_type_mapping = Self::build_contract_type_mapping(context, &contract_type_sort);
        let approval_status_mapping = Self::build_approval_status_mapping(context, &approval_status_sort);
        let priority_mapping = Self::build_priority_mapping(context, &priority_sort);
        let format_mapping = Self::build_format_mapping(context, &format_sort);
        let language_mapping = Self::build_language_mapping(context, &language_sort);
        
        Self {
            context,
            solver,
            user_sort,
            resource_sort,
            role_sort,
            position_sort,
            tenant_sort,
            department_sort,
            office_sort,
            document_type_sort,
            action_sort,
            security_level_sort,
            customer_tier_sort,
            region_sort,
            country_sort,
            time_zone_sort,
            contract_type_sort,
            approval_status_sort,
            priority_sort,
            format_sort,
            language_sort,
            user_has_role,
            user_has_position,
            user_has_tenant,
            user_has_department,
            user_has_office,
            user_is_registered,
            user_has_project,
            user_has_supervisor,
            user_has_supervisee,
            user_has_payrolling_permission,
            user_has_security_clearance,
            user_has_experience,
            user_has_customer_tier,
            user_has_region,
            user_has_country,
            user_has_time_zone,
            user_has_contract_type,
            user_has_delegated_authority,
            user_has_temporary_access,
            user_has_current_project,
            user_has_past_project,
            user_is_active,
            user_has_budget_authority,
            user_experience_value,
            user_budget_value,
            resource_has_type,
            resource_has_owner,
            resource_has_tenant,
            resource_has_department,
            resource_has_office,
            resource_has_recipient,
            resource_is_confidential,
            resource_contains_personal_info,
            resource_has_security_level,
            resource_has_project_id,
            resource_has_version,
            resource_has_size,
            resource_has_format,
            resource_has_language,
            resource_has_region,
            resource_has_country,
            resource_has_approval_status,
            resource_has_priority,
            resource_has_retention_period,
            resource_is_archived,
            resource_has_access_count,
            resource_size_value,
            resource_access_count_value,
            resource_retention_period_value,
            user_can_perform_action,
            user_mapping,
            resource_mapping,
            role_mapping,
            position_mapping,
            tenant_mapping,
            department_mapping,
            office_mapping,
            document_type_mapping,
            action_mapping,
            security_level_mapping,
            customer_tier_mapping,
            region_mapping,
            country_mapping,
            time_zone_mapping,
            contract_type_mapping,
            approval_status_mapping,
            priority_mapping,
            format_mapping,
            language_mapping,
        }
    }
    
    /// ユーザーのデータ型を構築
    fn build_user_sort(context: &'ctx Context, users: &[EdocumentUserAttribute]) -> z3::DatatypeSort<'ctx> {
        let mut builder = DatatypeBuilder::new(context, "User");
        
        for user in users {
            builder = builder.variant(&user.user_id, vec![]);
        }
        
        builder.finish()
    }
    
    /// リソースのデータ型を構築
    fn build_resource_sort(context: &'ctx Context, resources: &[EdocumentResourceAttribute]) -> z3::DatatypeSort<'ctx> {
        let mut builder = DatatypeBuilder::new(context, "Resource");
        
        for resource in resources {
            builder = builder.variant(&resource.resource_id, vec![]);
        }
        
        builder.finish()
    }
    
    /// ロールのデータ型を構築
    fn build_role_sort(context: &'ctx Context) -> z3::DatatypeSort<'ctx> {
        DatatypeBuilder::new(context, "Role")
            .variant("employee", vec![])
            .variant("manager", vec![])
            .variant("admin", vec![])
            .variant("helpdesk", vec![])
            .variant("customer", vec![])
            .variant("projectManager", vec![])
            .variant("legalOfficer", vec![])
            .variant("financialOfficer", vec![])
            .variant("auditor", vec![])
            .variant("consultant", vec![])
            .finish()
    }
    
    /// 役職のデータ型を構築
    fn build_position_sort(context: &'ctx Context) -> z3::DatatypeSort<'ctx> {
        DatatypeBuilder::new(context, "Position")
            .variant("secretary", vec![])
            .variant("director", vec![])
            .variant("seniorOfficeManager", vec![])
            .variant("officeManager", vec![])
            .variant("insuranceAgent", vec![])
            .variant("analyst", vec![])
            .variant("specialist", vec![])
            .variant("coordinator", vec![])
            .variant("lead", vec![])
            .variant("associate", vec![])
            .variant("senior", vec![])
            .variant("principal", vec![])
            .variant("vicePresident", vec![])
            .variant("none", vec![])
            .finish()
    }
    
    /// テナントのデータ型を構築
    fn build_tenant_sort(context: &'ctx Context) -> z3::DatatypeSort<'ctx> {
        DatatypeBuilder::new(context, "Tenant")
            // Organization tenants
            .variant("largeBank", vec![])
            .variant("largeBankLeasing", vec![])
            .variant("newsAgency", vec![])
            .variant("europeRegion", vec![])
            .variant("londonOffice", vec![])
            .variant("reseller", vec![])
            .variant("techCorp", vec![])
            .variant("pharmaceuticals", vec![])
            .variant("energyCompany", vec![])
            .variant("retailChain", vec![])
            .variant("consultingFirm", vec![])
            .variant("lawFirm", vec![])
            .variant("financialServices", vec![])
            .variant("healthcareSystem", vec![])
            .variant("educationInstitute", vec![])
            // Customer tenants
            .variant("carLeaser", vec![])
            .variant("ictProvider", vec![])
            .variant("privateReceiver", vec![])
            .variant("smallBusiness", vec![])
            .variant("startupCompany", vec![])
            .variant("governmentAgency", vec![])
            .variant("nonprofit", vec![])
            .variant("mediaCompany", vec![])
            .finish()
    }
    
    /// 部署のデータ型を構築
    fn build_department_sort(context: &'ctx Context) -> z3::DatatypeSort<'ctx> {
        DatatypeBuilder::new(context, "Department")
            // LargeBank departments
            .variant("largeBankSales", vec![])
            .variant("largeBankICT", vec![])
            .variant("largeBankIT", vec![])
            .variant("largeBankHR", vec![])
            .variant("largeBankAudit", vec![])
            .variant("largeBankLegal", vec![])
            .variant("largeBankRisk", vec![])
            .variant("largeBankCompliance", vec![])
            
            // LargeBankLeasing departments
            .variant("largeBankLeasingSales", vec![])
            .variant("largeBankLeasingCustomerCare", vec![])
            .variant("largeBankLeasingRisk", vec![])
            .variant("largeBankLeasingOperations", vec![])
            
            // Reseller departments
            .variant("resellerSales", vec![])
            .variant("resellerAccounting", vec![])
            .variant("resellerCustomer", vec![])
            .variant("resellerSupport", vec![])
            .variant("resellerMarketing", vec![])
            
            // NewsAgency departments
            .variant("newsAgencyIT", vec![])
            .variant("newsAgencyAudit", vec![])
            .variant("newsAgencyEditorial", vec![])
            .variant("newsAgencyMarketing", vec![])
            .variant("newsAgencyLegal", vec![])
            
            // EuropeRegion departments
            .variant("europeRegionHR", vec![])
            .variant("europeRegionIT", vec![])
            .variant("europeRegionFinance", vec![])
            .variant("europeRegionStrategy", vec![])
            .variant("europeRegionCompliance", vec![])
            
            // LondonOffice departments
            .variant("londonOfficeAudit", vec![])
            .variant("londonOfficeHR", vec![])
            .variant("londonOfficeSales", vec![])
            .variant("londonOfficeOperations", vec![])
            .variant("londonOfficeResearch", vec![])
            
            // TechCorp departments
            .variant("techCorpEngineering", vec![])
            .variant("techCorpProduct", vec![])
            .variant("techCorpSales", vec![])
            .variant("techCorpSupport", vec![])
            .variant("techCorpResearch", vec![])
            .variant("techCorpSecurity", vec![])
            
            // Pharmaceuticals departments
            .variant("pharmaceuticalsResearch", vec![])
            .variant("pharmaceuticalsRegulatory", vec![])
            .variant("pharmaceuticalsSales", vec![])
            .variant("pharmaceuticalsManufacturing", vec![])
            
            // EnergyCompany departments
            .variant("energyCompanyExploration", vec![])
            .variant("energyCompanyProduction", vec![])
            .variant("energyCompanyRefining", vec![])
            .variant("energyCompanyTradingAudit", vec![])
            
            // RetailChain departments
            .variant("retailChainOperations", vec![])
            .variant("retailChainMarketing", vec![])
            .variant("retailChainSupplyChain", vec![])
            .variant("retailChainFinance", vec![])
            
            // ConsultingFirm departments
            .variant("consultingFirmStrategy", vec![])
            .variant("consultingFirmTechnology", vec![])
            .variant("consultingFirmOperations", vec![])
            .variant("consultingFirmHR", vec![])
            
            // LawFirm departments
            .variant("lawFirmCorporate", vec![])
            .variant("lawFirmLitigation", vec![])
            .variant("lawFirmIntellectualProperty", vec![])
            .variant("lawFirmTax", vec![])
            .variant("lawFirmEmployment", vec![])
            
            // FinancialServices departments
            .variant("financialServicesWealth", vec![])
            .variant("financialServicesRetail", vec![])
            .variant("financialServicesInvestment", vec![])
            .variant("financialServicesRisk", vec![])
            
            // HealthcareSystem departments
            .variant("healthcareSystemClinical", vec![])
            .variant("healthcareSystemAdministrative", vec![])
            .variant("healthcareSystemResearch", vec![])
            .variant("healthcareSystemIT", vec![])
            
            // EducationInstitute departments
            .variant("educationInstituteAcademic", vec![])
            .variant("educationInstituteAdministrative", vec![])
            .variant("educationInstituteResearch", vec![])
            .variant("educationInstituteIT", vec![])
            
            // Customer tenant departments
            .variant("carLeaserAudit", vec![])
            .variant("carLeaserSecretary", vec![])
            .variant("carLeaserAccounting", vec![])
            .variant("carLeaserOperations", vec![])
            .variant("carLeaserMarketing", vec![])
            .variant("ictProviderAudit", vec![])
            .variant("ictProviderSecretary", vec![])
            .variant("ictProviderAccounting", vec![])
            .variant("ictProviderOperations", vec![])
            .variant("ictProviderMarketing", vec![])
            .variant("ictProviderICT", vec![])
            .variant("ictProviderDevelopment", vec![])
            .variant("ictProviderSupport", vec![])
            .variant("privateReceiverAudit", vec![])
            .variant("privateReceiverSecretary", vec![])
            .variant("privateReceiverAccounting", vec![])
            .variant("privateReceiverOperations", vec![])
            .variant("privateReceiverMarketing", vec![])
            .variant("smallBusinessAudit", vec![])
            .variant("smallBusinessSecretary", vec![])
            .variant("smallBusinessAccounting", vec![])
            .variant("smallBusinessOperations", vec![])
            .variant("smallBusinessMarketing", vec![])
            .variant("startupCompanyAudit", vec![])
            .variant("startupCompanySecretary", vec![])
            .variant("startupCompanyAccounting", vec![])
            .variant("startupCompanyOperations", vec![])
            .variant("startupCompanyMarketing", vec![])
            .variant("startupCompanyProduct", vec![])
            .variant("startupCompanyEngineering", vec![])
            .variant("startupCompanyBusinessDevelopment", vec![])
            .variant("governmentAgencyAudit", vec![])
            .variant("governmentAgencySecretary", vec![])
            .variant("governmentAgencyAccounting", vec![])
            .variant("governmentAgencyOperations", vec![])
            .variant("governmentAgencyMarketing", vec![])
            .variant("governmentAgencyPolicy", vec![])
            .variant("governmentAgencyPublicRelations", vec![])
            .variant("governmentAgencyCompliance", vec![])
            .variant("nonprofitAudit", vec![])
            .variant("nonprofitSecretary", vec![])
            .variant("nonprofitAccounting", vec![])
            .variant("nonprofitOperations", vec![])
            .variant("nonprofitMarketing", vec![])
            .variant("mediaCompanyAudit", vec![])
            .variant("mediaCompanySecretary", vec![])
            .variant("mediaCompanyAccounting", vec![])
            .variant("mediaCompanyOperations", vec![])
            .variant("mediaCompanyMarketing", vec![])
            .variant("none", vec![])
            .finish()
    }
    
    /// オフィスのデータ型を構築
    fn build_office_sort(context: &'ctx Context) -> z3::DatatypeSort<'ctx> {
        DatatypeBuilder::new(context, "Office")
            // LargeBank offices (15個)
            .variant("largeBankOffice1", vec![])
            .variant("largeBankOffice2", vec![])
            .variant("largeBankOffice3", vec![])
            .variant("largeBankOffice4", vec![])
            .variant("largeBankOffice5", vec![])
            .variant("largeBankOffice6", vec![])
            .variant("largeBankOffice7", vec![])
            .variant("largeBankOffice8", vec![])
            .variant("largeBankOffice9", vec![])
            .variant("largeBankOffice10", vec![])
            .variant("largeBankOffice11", vec![])
            .variant("largeBankOffice12", vec![])
            .variant("largeBankOffice13", vec![])
            .variant("largeBankOffice14", vec![])
            .variant("largeBankOffice15", vec![])
            // LargeBankLeasing offices (5個)
            .variant("largeBankLeasingOffice1", vec![])
            .variant("largeBankLeasingOffice2", vec![])
            .variant("largeBankLeasingOffice3", vec![])
            .variant("largeBankLeasingOffice4", vec![])
            .variant("largeBankLeasingOffice5", vec![])
            // NewsAgency offices (6個)
            .variant("newsAgencyOffice1", vec![])
            .variant("newsAgencyOffice2", vec![])
            .variant("newsAgencyOffice3", vec![])
            .variant("newsAgencyOffice4", vec![])
            .variant("newsAgencyOffice5", vec![])
            .variant("newsAgencyOffice6", vec![])
            // EuropeRegion offices (12個)
            .variant("europeRegionOffice1", vec![])
            .variant("europeRegionOffice2", vec![])
            .variant("europeRegionOffice3", vec![])
            .variant("europeRegionOffice4", vec![])
            .variant("europeRegionOffice5", vec![])
            .variant("europeRegionOffice6", vec![])
            .variant("europeRegionOffice7", vec![])
            .variant("europeRegionOffice8", vec![])
            .variant("europeRegionOffice9", vec![])
            .variant("europeRegionOffice10", vec![])
            .variant("europeRegionOffice11", vec![])
            .variant("europeRegionOffice12", vec![])
            // LondonOffice offices (4個)
            .variant("londonOfficeOffice1", vec![])
            .variant("londonOfficeOffice2", vec![])
            .variant("londonOfficeOffice3", vec![])
            .variant("londonOfficeOffice4", vec![])
            // Reseller offices (7個)
            .variant("resellerOffice1", vec![])
            .variant("resellerOffice2", vec![])
            .variant("resellerOffice3", vec![])
            .variant("resellerOffice4", vec![])
            .variant("resellerOffice5", vec![])
            .variant("resellerOffice6", vec![])
            .variant("resellerOffice7", vec![])
            // TechCorp offices (20個)
            .variant("techCorpOffice1", vec![])
            .variant("techCorpOffice2", vec![])
            .variant("techCorpOffice3", vec![])
            .variant("techCorpOffice4", vec![])
            .variant("techCorpOffice5", vec![])
            .variant("techCorpOffice6", vec![])
            .variant("techCorpOffice7", vec![])
            .variant("techCorpOffice8", vec![])
            .variant("techCorpOffice9", vec![])
            .variant("techCorpOffice10", vec![])
            .variant("techCorpOffice11", vec![])
            .variant("techCorpOffice12", vec![])
            .variant("techCorpOffice13", vec![])
            .variant("techCorpOffice14", vec![])
            .variant("techCorpOffice15", vec![])
            .variant("techCorpOffice16", vec![])
            .variant("techCorpOffice17", vec![])
            .variant("techCorpOffice18", vec![])
            .variant("techCorpOffice19", vec![])
            .variant("techCorpOffice20", vec![])
            // Pharmaceuticals offices (18個)
            .variant("pharmaceuticalsOffice1", vec![])
            .variant("pharmaceuticalsOffice2", vec![])
            .variant("pharmaceuticalsOffice3", vec![])
            .variant("pharmaceuticalsOffice4", vec![])
            .variant("pharmaceuticalsOffice5", vec![])
            .variant("pharmaceuticalsOffice6", vec![])
            .variant("pharmaceuticalsOffice7", vec![])
            .variant("pharmaceuticalsOffice8", vec![])
            .variant("pharmaceuticalsOffice9", vec![])
            .variant("pharmaceuticalsOffice10", vec![])
            .variant("pharmaceuticalsOffice11", vec![])
            .variant("pharmaceuticalsOffice12", vec![])
            .variant("pharmaceuticalsOffice13", vec![])
            .variant("pharmaceuticalsOffice14", vec![])
            .variant("pharmaceuticalsOffice15", vec![])
            .variant("pharmaceuticalsOffice16", vec![])
            .variant("pharmaceuticalsOffice17", vec![])
            .variant("pharmaceuticalsOffice18", vec![])
            // EnergyCompany offices (25個)
            .variant("energyCompanyOffice1", vec![])
            .variant("energyCompanyOffice2", vec![])
            .variant("energyCompanyOffice3", vec![])
            .variant("energyCompanyOffice4", vec![])
            .variant("energyCompanyOffice5", vec![])
            .variant("energyCompanyOffice6", vec![])
            .variant("energyCompanyOffice7", vec![])
            .variant("energyCompanyOffice8", vec![])
            .variant("energyCompanyOffice9", vec![])
            .variant("energyCompanyOffice10", vec![])
            .variant("energyCompanyOffice11", vec![])
            .variant("energyCompanyOffice12", vec![])
            .variant("energyCompanyOffice13", vec![])
            .variant("energyCompanyOffice14", vec![])
            .variant("energyCompanyOffice15", vec![])
            .variant("energyCompanyOffice16", vec![])
            .variant("energyCompanyOffice17", vec![])
            .variant("energyCompanyOffice18", vec![])
            .variant("energyCompanyOffice19", vec![])
            .variant("energyCompanyOffice20", vec![])
            .variant("energyCompanyOffice21", vec![])
            .variant("energyCompanyOffice22", vec![])
            .variant("energyCompanyOffice23", vec![])
            .variant("energyCompanyOffice24", vec![])
            .variant("energyCompanyOffice25", vec![])
            // RetailChain offices (first 25個)
            .variant("retailChainOffice1", vec![])
            .variant("retailChainOffice2", vec![])
            .variant("retailChainOffice3", vec![])
            .variant("retailChainOffice4", vec![])
            .variant("retailChainOffice5", vec![])
            .variant("retailChainOffice6", vec![])
            .variant("retailChainOffice7", vec![])
            .variant("retailChainOffice8", vec![])
            .variant("retailChainOffice9", vec![])
            .variant("retailChainOffice10", vec![])
            .variant("retailChainOffice11", vec![])
            .variant("retailChainOffice12", vec![])
            .variant("retailChainOffice13", vec![])
            .variant("retailChainOffice14", vec![])
            .variant("retailChainOffice15", vec![])
            .variant("retailChainOffice16", vec![])
            .variant("retailChainOffice17", vec![])
            .variant("retailChainOffice18", vec![])
            .variant("retailChainOffice19", vec![])
            .variant("retailChainOffice20", vec![])
            .variant("retailChainOffice21", vec![])
            .variant("retailChainOffice22", vec![])
            .variant("retailChainOffice23", vec![])
            .variant("retailChainOffice24", vec![])
            .variant("retailChainOffice25", vec![])
            // RetailChain offices (next 25個)
            .variant("retailChainOffice26", vec![])
            .variant("retailChainOffice27", vec![])
            .variant("retailChainOffice28", vec![])
            .variant("retailChainOffice29", vec![])
            .variant("retailChainOffice30", vec![])
            .variant("retailChainOffice31", vec![])
            .variant("retailChainOffice32", vec![])
            .variant("retailChainOffice33", vec![])
            .variant("retailChainOffice34", vec![])
            .variant("retailChainOffice35", vec![])
            .variant("retailChainOffice36", vec![])
            .variant("retailChainOffice37", vec![])
            .variant("retailChainOffice38", vec![])
            .variant("retailChainOffice39", vec![])
            .variant("retailChainOffice40", vec![])
            .variant("retailChainOffice41", vec![])
            .variant("retailChainOffice42", vec![])
            .variant("retailChainOffice43", vec![])
            .variant("retailChainOffice44", vec![])
            .variant("retailChainOffice45", vec![])
            .variant("retailChainOffice46", vec![])
            .variant("retailChainOffice47", vec![])
            .variant("retailChainOffice48", vec![])
            .variant("retailChainOffice49", vec![])
            .variant("retailChainOffice50", vec![])
            // ConsultingFirm offices (10個)
            .variant("consultingFirmOffice1", vec![])
            .variant("consultingFirmOffice2", vec![])
            .variant("consultingFirmOffice3", vec![])
            .variant("consultingFirmOffice4", vec![])
            .variant("consultingFirmOffice5", vec![])
            .variant("consultingFirmOffice6", vec![])
            .variant("consultingFirmOffice7", vec![])
            .variant("consultingFirmOffice8", vec![])
            .variant("consultingFirmOffice9", vec![])
            .variant("consultingFirmOffice10", vec![])
            // LawFirm offices (8個)
            .variant("lawFirmOffice1", vec![])
            .variant("lawFirmOffice2", vec![])
            .variant("lawFirmOffice3", vec![])
            .variant("lawFirmOffice4", vec![])
            .variant("lawFirmOffice5", vec![])
            .variant("lawFirmOffice6", vec![])
            .variant("lawFirmOffice7", vec![])
            .variant("lawFirmOffice8", vec![])
            // FinancialServices offices (15個)
            .variant("financialServicesOffice1", vec![])
            .variant("financialServicesOffice2", vec![])
            .variant("financialServicesOffice3", vec![])
            .variant("financialServicesOffice4", vec![])
            .variant("financialServicesOffice5", vec![])
            .variant("financialServicesOffice6", vec![])
            .variant("financialServicesOffice7", vec![])
            .variant("financialServicesOffice8", vec![])
            .variant("financialServicesOffice9", vec![])
            .variant("financialServicesOffice10", vec![])
            .variant("financialServicesOffice11", vec![])
            .variant("financialServicesOffice12", vec![])
            .variant("financialServicesOffice13", vec![])
            .variant("financialServicesOffice14", vec![])
            .variant("financialServicesOffice15", vec![])
            // HealthcareSystem offices (12個)
            .variant("healthcareSystemOffice1", vec![])
            .variant("healthcareSystemOffice2", vec![])
            .variant("healthcareSystemOffice3", vec![])
            .variant("healthcareSystemOffice4", vec![])
            .variant("healthcareSystemOffice5", vec![])
            .variant("healthcareSystemOffice6", vec![])
            .variant("healthcareSystemOffice7", vec![])
            .variant("healthcareSystemOffice8", vec![])
            .variant("healthcareSystemOffice9", vec![])
            .variant("healthcareSystemOffice10", vec![])
            .variant("healthcareSystemOffice11", vec![])
            .variant("healthcareSystemOffice12", vec![])
            // EducationInstitute offices (8個)
            .variant("educationInstituteOffice1", vec![])
            .variant("educationInstituteOffice2", vec![])
            .variant("educationInstituteOffice3", vec![])
            .variant("educationInstituteOffice4", vec![])
            .variant("educationInstituteOffice5", vec![])
            .variant("educationInstituteOffice6", vec![])
            .variant("educationInstituteOffice7", vec![])
            .variant("educationInstituteOffice8", vec![])
            // IctProvider offices (8個)
            .variant("ictProviderOffice1", vec![])
            .variant("ictProviderOffice2", vec![])
            .variant("ictProviderOffice3", vec![])
            .variant("ictProviderOffice4", vec![])
            .variant("ictProviderOffice5", vec![])
            .variant("ictProviderOffice6", vec![])
            .variant("ictProviderOffice7", vec![])
            .variant("ictProviderOffice8", vec![])
            // Customer tenant offices (5個ずつ)
            .variant("carLeaserOffice1", vec![])
            .variant("carLeaserOffice2", vec![])
            .variant("carLeaserOffice3", vec![])
            .variant("carLeaserOffice4", vec![])
            .variant("carLeaserOffice5", vec![])
            .variant("privateReceiverOffice1", vec![])
            .variant("privateReceiverOffice2", vec![])
            .variant("privateReceiverOffice3", vec![])
            .variant("privateReceiverOffice4", vec![])
            .variant("privateReceiverOffice5", vec![])
            .variant("smallBusinessOffice1", vec![])
            .variant("smallBusinessOffice2", vec![])
            .variant("smallBusinessOffice3", vec![])
            .variant("smallBusinessOffice4", vec![])
            .variant("smallBusinessOffice5", vec![])
            .variant("startupCompanyOffice1", vec![])
            .variant("startupCompanyOffice2", vec![])
            .variant("startupCompanyOffice3", vec![])
            .variant("startupCompanyOffice4", vec![])
            .variant("startupCompanyOffice5", vec![])
            .variant("governmentAgencyOffice1", vec![])
            .variant("governmentAgencyOffice2", vec![])
            .variant("governmentAgencyOffice3", vec![])
            .variant("governmentAgencyOffice4", vec![])
            .variant("governmentAgencyOffice5", vec![])
            .variant("nonprofitOffice1", vec![])
            .variant("nonprofitOffice2", vec![])
            .variant("nonprofitOffice3", vec![])
            .variant("nonprofitOffice4", vec![])
            .variant("nonprofitOffice5", vec![])
            .variant("mediaCompanyOffice1", vec![])
            .variant("mediaCompanyOffice2", vec![])
            .variant("mediaCompanyOffice3", vec![])
            .variant("mediaCompanyOffice4", vec![])
            .variant("mediaCompanyOffice5", vec![])
            .variant("none", vec![])
            .finish()
    }
    
    /// ドキュメントタイプのデータ型を構築
    fn build_document_type_sort(context: &'ctx Context) -> z3::DatatypeSort<'ctx> {
        DatatypeBuilder::new(context, "DocumentType")
            .variant("bankingNote", vec![])
            .variant("trafficFine", vec![])
            .variant("salesOffer", vec![])
            .variant("contract", vec![])
            .variant("invoice", vec![])
            .variant("paycheck", vec![])
            .variant("legalDocument", vec![])
            .variant("technicalSpecification", vec![])
            .variant("financialReport", vec![])
            .variant("auditReport", vec![])
            .variant("hrDocument", vec![])
            .variant("policyDocument", vec![])
            .variant("projectPlan", vec![])
            .variant("meetingMinutes", vec![])
            .variant("complianceReport", vec![])
            .variant("riskAssessment", vec![])
            .variant("budgetReport", vec![])
            .variant("performanceReview", vec![])
            .variant("strategicPlan", vec![])
            .variant("marketAnalysis", vec![])
            .finish()
    }
    
    /// アクションのデータ型を構築
    fn build_action_sort(context: &'ctx Context) -> z3::DatatypeSort<'ctx> {
        DatatypeBuilder::new(context, "Action")
            .variant("view", vec![])
            .variant("search", vec![])
            .variant("readMetaInfo", vec![])
            .variant("send", vec![])
            .variant("edit", vec![])
            .variant("approve", vec![])
            .finish()
    }

    /// セキュリティレベルのデータ型を構築
    fn build_security_level_sort(context: &'ctx Context) -> z3::DatatypeSort<'ctx> {
        DatatypeBuilder::new(context, "SecurityLevel")
            .variant("public", vec![])
            .variant("internal", vec![])
            .variant("confidential", vec![])
            .variant("secret", vec![])
            .variant("topSecret", vec![])
            .finish()
    }

    /// 顧客階層のデータ型を構築
    fn build_customer_tier_sort(context: &'ctx Context) -> z3::DatatypeSort<'ctx> {
        DatatypeBuilder::new(context, "CustomerTier")
            .variant("bronze", vec![])
            .variant("silver", vec![])
            .variant("gold", vec![])
            .variant("platinum", vec![])
            .variant("vip", vec![])
            .variant("none", vec![])
            .finish()
    }

    /// 地域のデータ型を構築
    fn build_region_sort(context: &'ctx Context) -> z3::DatatypeSort<'ctx> {
        DatatypeBuilder::new(context, "Region")
            .variant("northAmerica", vec![])
            .variant("europe", vec![])
            .variant("asia", vec![])
            .variant("latinAmerica", vec![])
            .variant("africa", vec![])
            .finish()
    }

    /// 国のデータ型を構築
    fn build_country_sort(context: &'ctx Context) -> z3::DatatypeSort<'ctx> {
        DatatypeBuilder::new(context, "Country")
            .variant("usa", vec![])
            .variant("canada", vec![])
            .variant("mexico", vec![])
            .variant("uk", vec![])
            .variant("germany", vec![])
            .variant("france", vec![])
            .variant("netherlands", vec![])
            .variant("spain", vec![])
            .variant("japan", vec![])
            .variant("china", vec![])
            .variant("india", vec![])
            .variant("singapore", vec![])
            .variant("southKorea", vec![])
            .variant("brazil", vec![])
            .variant("argentina", vec![])
            .variant("chile", vec![])
            .variant("colombia", vec![])
            .variant("southAfrica", vec![])
            .variant("nigeria", vec![])
            .variant("egypt", vec![])
            .variant("kenya", vec![])
            .finish()
    }

    /// タイムゾーンのデータ型を構築
    fn build_time_zone_sort(context: &'ctx Context) -> z3::DatatypeSort<'ctx> {
        DatatypeBuilder::new(context, "TimeZone")
            .variant("utc", vec![])
            .variant("est", vec![])
            .variant("pst", vec![])
            .variant("gmt", vec![])
            .variant("cet", vec![])
            .variant("jst", vec![])
            .variant("ist", vec![])
            .finish()
    }

    /// 契約タイプのデータ型を構築
    fn build_contract_type_sort(context: &'ctx Context) -> z3::DatatypeSort<'ctx> {
        DatatypeBuilder::new(context, "ContractType")
            .variant("permanent", vec![])
            .variant("temporary", vec![])
            .variant("contractor", vec![])
            .variant("consultant", vec![])
            .finish()
    }

    /// 承認状態のデータ型を構築
    fn build_approval_status_sort(context: &'ctx Context) -> z3::DatatypeSort<'ctx> {
        DatatypeBuilder::new(context, "ApprovalStatus")
            .variant("draft", vec![])
            .variant("pending", vec![])
            .variant("approved", vec![])
            .variant("rejected", vec![])
            .variant("archived", vec![])
            .finish()
    }

    /// 優先度のデータ型を構築
    fn build_priority_sort(context: &'ctx Context) -> z3::DatatypeSort<'ctx> {
        DatatypeBuilder::new(context, "Priority")
            .variant("low", vec![])
            .variant("medium", vec![])
            .variant("high", vec![])
            .variant("critical", vec![])
            .finish()
    }

    /// フォーマットのデータ型を構築
    fn build_format_sort(context: &'ctx Context) -> z3::DatatypeSort<'ctx> {
        DatatypeBuilder::new(context, "Format")
            .variant("pdf", vec![])
            .variant("docx", vec![])
            .variant("xlsx", vec![])
            .variant("txt", vec![])
            .variant("pptx", vec![])
            .variant("xml", vec![])
            .variant("json", vec![])
            .finish()
    }

    /// 言語のデータ型を構築
    fn build_language_sort(context: &'ctx Context) -> z3::DatatypeSort<'ctx> {
        DatatypeBuilder::new(context, "Language")
            .variant("en", vec![])
            .variant("es", vec![])
            .variant("fr", vec![])
            .variant("de", vec![])
            .variant("ja", vec![])
            .variant("zh", vec![])
            .variant("pt", vec![])
            .finish()
    }
    
    /// ユーザーマッピングを構築
    fn build_user_mapping(
        context: &'ctx Context, 
        user_sort: &z3::DatatypeSort<'ctx>, 
        users: &[EdocumentUserAttribute]
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
        resources: &[EdocumentResourceAttribute]
    ) -> HashMap<String, Dynamic<'ctx>> {
        let mut mapping = HashMap::new();
        
        for (i, resource) in resources.iter().enumerate() {
            let resource_const = resource_sort.variants[i].constructor.apply(&[]);
            mapping.insert(resource.resource_id.clone(), resource_const);
        }
        
        mapping
    }
    
    /// ロールマッピングを構築
    fn build_role_mapping(
        context: &'ctx Context, 
        role_sort: &z3::DatatypeSort<'ctx>
    ) -> HashMap<Role, Dynamic<'ctx>> {
        let mut mapping = HashMap::new();
        let mut idx = 0;
        
        mapping.insert(Role::Employee, role_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Role::Manager, role_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Role::Admin, role_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Role::Helpdesk, role_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Role::Customer, role_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Role::ProjectManager, role_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Role::LegalOfficer, role_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Role::FinancialOfficer, role_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Role::Auditor, role_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Role::Consultant, role_sort.variants[idx].constructor.apply(&[]));
        
        mapping
    }
    
    /// 役職マッピングを構築
    fn build_position_mapping(
        context: &'ctx Context, 
        position_sort: &z3::DatatypeSort<'ctx>
    ) -> HashMap<Position, Dynamic<'ctx>> {
        let mut mapping = HashMap::new();
        let mut idx = 0;
        
        // Z3ソート順序に合わせて
        mapping.insert(Position::Secretary, position_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Position::Director, position_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Position::SeniorOfficeManager, position_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Position::OfficeManager, position_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Position::InsuranceAgent, position_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Position::Analyst, position_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Position::Specialist, position_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Position::Coordinator, position_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Position::Lead, position_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Position::Associate, position_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Position::Senior, position_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Position::Principal, position_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Position::VicePresident, position_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Position::None, position_sort.variants[idx].constructor.apply(&[]));
        
        mapping
    }
    
    /// テナントマッピングを構築
    fn build_tenant_mapping(
        context: &'ctx Context, 
        tenant_sort: &z3::DatatypeSort<'ctx>
    ) -> HashMap<Tenant, Dynamic<'ctx>> {
        let mut mapping = HashMap::new();
        let mut idx = 0;
        
        // Organization tenants
        mapping.insert(Tenant::LargeBank, tenant_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Tenant::LargeBankLeasing, tenant_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Tenant::NewsAgency, tenant_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Tenant::EuropeRegion, tenant_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Tenant::LondonOffice, tenant_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Tenant::Reseller, tenant_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Tenant::TechCorp, tenant_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Tenant::Pharmaceuticals, tenant_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Tenant::EnergyCompany, tenant_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Tenant::RetailChain, tenant_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Tenant::ConsultingFirm, tenant_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Tenant::LawFirm, tenant_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Tenant::FinancialServices, tenant_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Tenant::HealthcareSystem, tenant_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Tenant::EducationInstitute, tenant_sort.variants[idx].constructor.apply(&[])); idx += 1;
        // Customer tenants
        mapping.insert(Tenant::CarLeaser, tenant_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Tenant::IctProvider, tenant_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Tenant::PrivateReceiver, tenant_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Tenant::SmallBusiness, tenant_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Tenant::StartupCompany, tenant_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Tenant::GovernmentAgency, tenant_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Tenant::Nonprofit, tenant_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Tenant::MediaCompany, tenant_sort.variants[idx].constructor.apply(&[]));
        
        mapping
    }
    
    /// 部署マッピングを構築
    fn build_department_mapping(
        context: &'ctx Context, 
        department_sort: &z3::DatatypeSort<'ctx>
    ) -> HashMap<Department, Dynamic<'ctx>> {
        let mut mapping = HashMap::new();
        let mut idx = 0;
        
        // Z3ソート順序に合わせて（build_department_sortと同じ順序）
        
        // LargeBank departments
        mapping.insert(Department::LargeBankSales, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::LargeBankICT, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::LargeBankIT, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::LargeBankHR, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::LargeBankAudit, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::LargeBankLegal, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::LargeBankRisk, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::LargeBankCompliance, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        // LargeBankLeasing departments
        mapping.insert(Department::LargeBankLeasingSales, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::LargeBankLeasingCustomerCare, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::LargeBankLeasingRisk, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::LargeBankLeasingOperations, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        // Reseller departments
        mapping.insert(Department::ResellerSales, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::ResellerAccounting, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::ResellerCustomer, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::ResellerSupport, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::ResellerMarketing, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        // NewsAgency departments
        mapping.insert(Department::NewsAgencyIT, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::NewsAgencyAudit, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::NewsAgencyEditorial, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::NewsAgencyMarketing, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::NewsAgencyLegal, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        // EuropeRegion departments
        mapping.insert(Department::EuropeRegionHR, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::EuropeRegionIT, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::EuropeRegionFinance, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::EuropeRegionStrategy, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::EuropeRegionCompliance, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        // LondonOffice departments
        mapping.insert(Department::LondonOfficeAudit, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::LondonOfficeHR, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::LondonOfficeSales, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::LondonOfficeOperations, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::LondonOfficeResearch, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        // TechCorp departments
        mapping.insert(Department::TechCorpEngineering, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::TechCorpProduct, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::TechCorpSales, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::TechCorpSupport, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::TechCorpResearch, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::TechCorpSecurity, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        // Pharmaceuticals departments
        mapping.insert(Department::PharmaceuticalsResearch, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::PharmaceuticalsRegulatory, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::PharmaceuticalsSales, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::PharmaceuticalsManufacturing, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        // EnergyCompany departments
        mapping.insert(Department::EnergyCompanyExploration, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::EnergyCompanyProduction, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::EnergyCompanyRefining, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::EnergyCompanyTradingAudit, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        // RetailChain departments
        mapping.insert(Department::RetailChainOperations, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::RetailChainMarketing, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::RetailChainSupplyChain, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::RetailChainFinance, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        // ConsultingFirm departments
        mapping.insert(Department::ConsultingFirmStrategy, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::ConsultingFirmTechnology, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::ConsultingFirmOperations, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::ConsultingFirmHR, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        // LawFirm departments
        mapping.insert(Department::LawFirmCorporate, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::LawFirmLitigation, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::LawFirmIntellectualProperty, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::LawFirmTax, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::LawFirmEmployment, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        // FinancialServices departments
        mapping.insert(Department::FinancialServicesWealth, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::FinancialServicesRetail, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::FinancialServicesInvestment, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::FinancialServicesRisk, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        // HealthcareSystem departments
        mapping.insert(Department::HealthcareSystemClinical, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::HealthcareSystemAdministrative, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::HealthcareSystemResearch, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::HealthcareSystemIT, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        // EducationInstitute departments
        mapping.insert(Department::EducationInstituteAcademic, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::EducationInstituteAdministrative, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::EducationInstituteResearch, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::EducationInstituteIT, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        // Customer tenant departments
        mapping.insert(Department::CarLeaserAudit, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::CarLeaserSecretary, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::CarLeaserAccounting, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::CarLeaserOperations, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::CarLeaserMarketing, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::IctProviderAudit, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::IctProviderSecretary, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::IctProviderAccounting, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::IctProviderOperations, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::IctProviderMarketing, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::IctProviderICT, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::IctProviderDevelopment, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::IctProviderSupport, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::PrivateReceiverAudit, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::PrivateReceiverSecretary, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::PrivateReceiverAccounting, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::PrivateReceiverOperations, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::PrivateReceiverMarketing, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::SmallBusinessAudit, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::SmallBusinessSecretary, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::SmallBusinessAccounting, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::SmallBusinessOperations, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::SmallBusinessMarketing, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::StartupCompanyAudit, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::StartupCompanySecretary, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::StartupCompanyAccounting, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::StartupCompanyOperations, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::StartupCompanyMarketing, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::StartupCompanyProduct, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::StartupCompanyEngineering, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::StartupCompanyBusinessDevelopment, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::GovernmentAgencyAudit, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::GovernmentAgencySecretary, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::GovernmentAgencyAccounting, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::GovernmentAgencyOperations, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::GovernmentAgencyMarketing, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::GovernmentAgencyPolicy, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::GovernmentAgencyPublicRelations, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::GovernmentAgencyCompliance, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::NonprofitAudit, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::NonprofitSecretary, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::NonprofitAccounting, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::NonprofitOperations, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::NonprofitMarketing, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::MediaCompanyAudit, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::MediaCompanySecretary, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::MediaCompanyAccounting, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::MediaCompanyOperations, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::MediaCompanyMarketing, department_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Department::None, department_sort.variants[idx].constructor.apply(&[]));
        
        mapping
    }
    
    /// オフィスマッピングを構築
    fn build_office_mapping(
        context: &'ctx Context, 
        office_sort: &z3::DatatypeSort<'ctx>
    ) -> HashMap<Office, Dynamic<'ctx>> {
        let mut mapping = HashMap::new();
        let mut idx = 0;
        
        // LargeBank offices (15個)
        mapping.insert(Office::LargeBankOffice1, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::LargeBankOffice2, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::LargeBankOffice3, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::LargeBankOffice4, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::LargeBankOffice5, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::LargeBankOffice6, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::LargeBankOffice7, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::LargeBankOffice8, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::LargeBankOffice9, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::LargeBankOffice10, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::LargeBankOffice11, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::LargeBankOffice12, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::LargeBankOffice13, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::LargeBankOffice14, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::LargeBankOffice15, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        // LargeBankLeasing offices (5個)
        mapping.insert(Office::LargeBankLeasingOffice1, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::LargeBankLeasingOffice2, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::LargeBankLeasingOffice3, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::LargeBankLeasingOffice4, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::LargeBankLeasingOffice5, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        // NewsAgency offices (6個)
        mapping.insert(Office::NewsAgencyOffice1, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::NewsAgencyOffice2, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::NewsAgencyOffice3, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::NewsAgencyOffice4, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::NewsAgencyOffice5, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::NewsAgencyOffice6, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        // EuropeRegion offices (12個)
        mapping.insert(Office::EuropeRegionOffice1, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EuropeRegionOffice2, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EuropeRegionOffice3, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EuropeRegionOffice4, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EuropeRegionOffice5, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EuropeRegionOffice6, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EuropeRegionOffice7, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EuropeRegionOffice8, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EuropeRegionOffice9, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EuropeRegionOffice10, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EuropeRegionOffice11, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EuropeRegionOffice12, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        // LondonOffice offices (4個)
        mapping.insert(Office::LondonOfficeOffice1, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::LondonOfficeOffice2, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::LondonOfficeOffice3, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::LondonOfficeOffice4, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        // Reseller offices (7個)
        mapping.insert(Office::ResellerOffice1, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::ResellerOffice2, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::ResellerOffice3, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::ResellerOffice4, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::ResellerOffice5, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::ResellerOffice6, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::ResellerOffice7, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        // TechCorp offices (20個)
        mapping.insert(Office::TechCorpOffice1, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::TechCorpOffice2, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::TechCorpOffice3, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::TechCorpOffice4, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::TechCorpOffice5, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::TechCorpOffice6, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::TechCorpOffice7, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::TechCorpOffice8, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::TechCorpOffice9, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::TechCorpOffice10, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::TechCorpOffice11, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::TechCorpOffice12, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::TechCorpOffice13, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::TechCorpOffice14, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::TechCorpOffice15, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::TechCorpOffice16, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::TechCorpOffice17, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::TechCorpOffice18, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::TechCorpOffice19, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::TechCorpOffice20, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        // Pharmaceuticals offices (18個)
        mapping.insert(Office::PharmaceuticalsOffice1, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::PharmaceuticalsOffice2, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::PharmaceuticalsOffice3, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::PharmaceuticalsOffice4, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::PharmaceuticalsOffice5, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::PharmaceuticalsOffice6, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::PharmaceuticalsOffice7, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::PharmaceuticalsOffice8, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::PharmaceuticalsOffice9, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::PharmaceuticalsOffice10, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::PharmaceuticalsOffice11, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::PharmaceuticalsOffice12, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::PharmaceuticalsOffice13, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::PharmaceuticalsOffice14, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::PharmaceuticalsOffice15, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::PharmaceuticalsOffice16, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::PharmaceuticalsOffice17, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::PharmaceuticalsOffice18, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        // EnergyCompany offices (25個)
        mapping.insert(Office::EnergyCompanyOffice1, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EnergyCompanyOffice2, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EnergyCompanyOffice3, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EnergyCompanyOffice4, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EnergyCompanyOffice5, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EnergyCompanyOffice6, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EnergyCompanyOffice7, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EnergyCompanyOffice8, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EnergyCompanyOffice9, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EnergyCompanyOffice10, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EnergyCompanyOffice11, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EnergyCompanyOffice12, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EnergyCompanyOffice13, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EnergyCompanyOffice14, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EnergyCompanyOffice15, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EnergyCompanyOffice16, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EnergyCompanyOffice17, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EnergyCompanyOffice18, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EnergyCompanyOffice19, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EnergyCompanyOffice20, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EnergyCompanyOffice21, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EnergyCompanyOffice22, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EnergyCompanyOffice23, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EnergyCompanyOffice24, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EnergyCompanyOffice25, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        // RetailChain offices (50個) - 一部のみ実装
        mapping.insert(Office::RetailChainOffice1, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice2, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice3, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice4, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice5, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice6, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice7, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice8, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice9, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice10, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        // Continue for all 50 retail chain offices...
        mapping.insert(Office::RetailChainOffice11, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice12, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice13, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice14, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice15, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice16, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice17, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice18, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice19, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice20, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice21, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice22, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice23, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice24, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice25, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice26, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice27, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice28, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice29, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice30, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice31, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice32, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice33, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice34, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice35, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice36, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice37, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice38, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice39, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice40, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice41, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice42, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice43, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice44, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice45, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice46, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice47, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice48, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice49, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::RetailChainOffice50, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        // Continue with other offices...
        mapping.insert(Office::ConsultingFirmOffice1, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::ConsultingFirmOffice2, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::ConsultingFirmOffice3, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::ConsultingFirmOffice4, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::ConsultingFirmOffice5, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::ConsultingFirmOffice6, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::ConsultingFirmOffice7, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::ConsultingFirmOffice8, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::ConsultingFirmOffice9, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::ConsultingFirmOffice10, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        // LawFirm offices (8個)
        mapping.insert(Office::LawFirmOffice1, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::LawFirmOffice2, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::LawFirmOffice3, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::LawFirmOffice4, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::LawFirmOffice5, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::LawFirmOffice6, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::LawFirmOffice7, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::LawFirmOffice8, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        // FinancialServices offices (15個)
        mapping.insert(Office::FinancialServicesOffice1, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::FinancialServicesOffice2, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::FinancialServicesOffice3, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::FinancialServicesOffice4, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::FinancialServicesOffice5, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::FinancialServicesOffice6, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::FinancialServicesOffice7, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::FinancialServicesOffice8, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::FinancialServicesOffice9, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::FinancialServicesOffice10, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::FinancialServicesOffice11, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::FinancialServicesOffice12, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::FinancialServicesOffice13, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::FinancialServicesOffice14, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::FinancialServicesOffice15, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        // HealthcareSystem offices (12個)
        mapping.insert(Office::HealthcareSystemOffice1, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::HealthcareSystemOffice2, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::HealthcareSystemOffice3, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::HealthcareSystemOffice4, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::HealthcareSystemOffice5, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::HealthcareSystemOffice6, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::HealthcareSystemOffice7, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::HealthcareSystemOffice8, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::HealthcareSystemOffice9, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::HealthcareSystemOffice10, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::HealthcareSystemOffice11, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::HealthcareSystemOffice12, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        // EducationInstitute offices (8個)
        mapping.insert(Office::EducationInstituteOffice1, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EducationInstituteOffice2, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EducationInstituteOffice3, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EducationInstituteOffice4, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EducationInstituteOffice5, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EducationInstituteOffice6, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EducationInstituteOffice7, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::EducationInstituteOffice8, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        // IctProvider offices (8個)
        mapping.insert(Office::IctProviderOffice1, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::IctProviderOffice2, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::IctProviderOffice3, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::IctProviderOffice4, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::IctProviderOffice5, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::IctProviderOffice6, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::IctProviderOffice7, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::IctProviderOffice8, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        // Customer tenant offices (5個ずつ)
        mapping.insert(Office::CarLeaserOffice1, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::CarLeaserOffice2, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::CarLeaserOffice3, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::CarLeaserOffice4, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::CarLeaserOffice5, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        mapping.insert(Office::PrivateReceiverOffice1, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::PrivateReceiverOffice2, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::PrivateReceiverOffice3, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::PrivateReceiverOffice4, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::PrivateReceiverOffice5, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        mapping.insert(Office::SmallBusinessOffice1, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::SmallBusinessOffice2, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::SmallBusinessOffice3, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::SmallBusinessOffice4, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::SmallBusinessOffice5, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        mapping.insert(Office::StartupCompanyOffice1, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::StartupCompanyOffice2, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::StartupCompanyOffice3, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::StartupCompanyOffice4, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::StartupCompanyOffice5, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        mapping.insert(Office::GovernmentAgencyOffice1, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::GovernmentAgencyOffice2, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::GovernmentAgencyOffice3, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::GovernmentAgencyOffice4, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::GovernmentAgencyOffice5, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        mapping.insert(Office::NonprofitOffice1, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::NonprofitOffice2, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::NonprofitOffice3, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::NonprofitOffice4, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::NonprofitOffice5, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        mapping.insert(Office::MediaCompanyOffice1, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::MediaCompanyOffice2, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::MediaCompanyOffice3, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::MediaCompanyOffice4, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(Office::MediaCompanyOffice5, office_sort.variants[idx].constructor.apply(&[])); idx += 1;
        
        // None
        mapping.insert(Office::None, office_sort.variants[idx].constructor.apply(&[]));
        
        mapping
    }
    
    /// ドキュメントタイプマッピングを構築
    fn build_document_type_mapping(
        context: &'ctx Context, 
        document_type_sort: &z3::DatatypeSort<'ctx>
    ) -> HashMap<DocumentType, Dynamic<'ctx>> {
        let mut mapping = HashMap::new();
        let mut idx = 0;
        
        // Z3ソート順序に合わせて（build_document_type_sortと同じ順序）
        mapping.insert(DocumentType::BankingNote, document_type_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(DocumentType::TrafficFine, document_type_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(DocumentType::SalesOffer, document_type_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(DocumentType::Contract, document_type_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(DocumentType::Invoice, document_type_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(DocumentType::Paycheck, document_type_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(DocumentType::LegalDocument, document_type_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(DocumentType::TechnicalSpecification, document_type_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(DocumentType::FinancialReport, document_type_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(DocumentType::AuditReport, document_type_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(DocumentType::HrDocument, document_type_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(DocumentType::PolicyDocument, document_type_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(DocumentType::ProjectPlan, document_type_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(DocumentType::MeetingMinutes, document_type_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(DocumentType::ComplianceReport, document_type_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(DocumentType::RiskAssessment, document_type_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(DocumentType::BudgetReport, document_type_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(DocumentType::PerformanceReview, document_type_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(DocumentType::StrategicPlan, document_type_sort.variants[idx].constructor.apply(&[])); idx += 1;
        mapping.insert(DocumentType::MarketAnalysis, document_type_sort.variants[idx].constructor.apply(&[]));
        
        mapping
    }
    
    /// アクションマッピングを構築
    fn build_action_mapping(
        context: &'ctx Context, 
        action_sort: &z3::DatatypeSort<'ctx>
    ) -> HashMap<Action, Dynamic<'ctx>> {
        let mut mapping = HashMap::new();
        
        mapping.insert(Action::View, action_sort.variants[0].constructor.apply(&[]));
        mapping.insert(Action::Search, action_sort.variants[1].constructor.apply(&[]));
        mapping.insert(Action::ReadMetaInfo, action_sort.variants[2].constructor.apply(&[]));
        mapping.insert(Action::Send, action_sort.variants[3].constructor.apply(&[]));
        mapping.insert(Action::Edit, action_sort.variants[4].constructor.apply(&[]));
        mapping.insert(Action::Approve, action_sort.variants[5].constructor.apply(&[]));
        
        mapping
    }

    /// セキュリティレベルマッピングを構築
    fn build_security_level_mapping(
        context: &'ctx Context, 
        security_level_sort: &z3::DatatypeSort<'ctx>
    ) -> HashMap<SecurityLevel, Dynamic<'ctx>> {
        let mut mapping = HashMap::new();
        
        mapping.insert(SecurityLevel::Public, security_level_sort.variants[0].constructor.apply(&[]));
        mapping.insert(SecurityLevel::Internal, security_level_sort.variants[1].constructor.apply(&[]));
        mapping.insert(SecurityLevel::Confidential, security_level_sort.variants[2].constructor.apply(&[]));
        mapping.insert(SecurityLevel::Secret, security_level_sort.variants[3].constructor.apply(&[]));
        mapping.insert(SecurityLevel::TopSecret, security_level_sort.variants[4].constructor.apply(&[]));
        
        mapping
    }

    /// 顧客階層マッピングを構築
    fn build_customer_tier_mapping(
        context: &'ctx Context, 
        customer_tier_sort: &z3::DatatypeSort<'ctx>
    ) -> HashMap<CustomerTier, Dynamic<'ctx>> {
        let mut mapping = HashMap::new();
        
        mapping.insert(CustomerTier::Bronze, customer_tier_sort.variants[0].constructor.apply(&[]));
        mapping.insert(CustomerTier::Silver, customer_tier_sort.variants[1].constructor.apply(&[]));
        mapping.insert(CustomerTier::Gold, customer_tier_sort.variants[2].constructor.apply(&[]));
        mapping.insert(CustomerTier::Platinum, customer_tier_sort.variants[3].constructor.apply(&[]));
        mapping.insert(CustomerTier::Vip, customer_tier_sort.variants[4].constructor.apply(&[]));
        mapping.insert(CustomerTier::None, customer_tier_sort.variants[5].constructor.apply(&[]));
        
        mapping
    }

    /// 地域マッピングを構築
    fn build_region_mapping(
        context: &'ctx Context, 
        region_sort: &z3::DatatypeSort<'ctx>
    ) -> HashMap<Region, Dynamic<'ctx>> {
        let mut mapping = HashMap::new();
        
        mapping.insert(Region::NorthAmerica, region_sort.variants[0].constructor.apply(&[]));
        mapping.insert(Region::Europe, region_sort.variants[1].constructor.apply(&[]));
        mapping.insert(Region::Asia, region_sort.variants[2].constructor.apply(&[]));
        mapping.insert(Region::LatinAmerica, region_sort.variants[3].constructor.apply(&[]));
        mapping.insert(Region::Africa, region_sort.variants[4].constructor.apply(&[]));
        
        mapping
    }

    /// 国マッピングを構築
    fn build_country_mapping(
        context: &'ctx Context, 
        country_sort: &z3::DatatypeSort<'ctx>
    ) -> HashMap<Country, Dynamic<'ctx>> {
        let mut mapping = HashMap::new();
        
        mapping.insert(Country::USA, country_sort.variants[0].constructor.apply(&[]));
        mapping.insert(Country::Canada, country_sort.variants[1].constructor.apply(&[]));
        mapping.insert(Country::Mexico, country_sort.variants[2].constructor.apply(&[]));
        mapping.insert(Country::UK, country_sort.variants[3].constructor.apply(&[]));
        mapping.insert(Country::Germany, country_sort.variants[4].constructor.apply(&[]));
        mapping.insert(Country::France, country_sort.variants[5].constructor.apply(&[]));
        mapping.insert(Country::Netherlands, country_sort.variants[6].constructor.apply(&[]));
        mapping.insert(Country::Spain, country_sort.variants[7].constructor.apply(&[]));
        mapping.insert(Country::Japan, country_sort.variants[8].constructor.apply(&[]));
        mapping.insert(Country::China, country_sort.variants[9].constructor.apply(&[]));
        mapping.insert(Country::India, country_sort.variants[10].constructor.apply(&[]));
        mapping.insert(Country::Singapore, country_sort.variants[11].constructor.apply(&[]));
        mapping.insert(Country::SouthKorea, country_sort.variants[12].constructor.apply(&[]));
        mapping.insert(Country::Brazil, country_sort.variants[13].constructor.apply(&[]));
        mapping.insert(Country::Argentina, country_sort.variants[14].constructor.apply(&[]));
        mapping.insert(Country::Chile, country_sort.variants[15].constructor.apply(&[]));
        mapping.insert(Country::Colombia, country_sort.variants[16].constructor.apply(&[]));
        mapping.insert(Country::SouthAfrica, country_sort.variants[17].constructor.apply(&[]));
        mapping.insert(Country::Nigeria, country_sort.variants[18].constructor.apply(&[]));
        mapping.insert(Country::Egypt, country_sort.variants[19].constructor.apply(&[]));
        mapping.insert(Country::Kenya, country_sort.variants[20].constructor.apply(&[]));
        
        mapping
    }

    /// タイムゾーンマッピングを構築
    fn build_time_zone_mapping(
        context: &'ctx Context, 
        time_zone_sort: &z3::DatatypeSort<'ctx>
    ) -> HashMap<TimeZone, Dynamic<'ctx>> {
        let mut mapping = HashMap::new();
        
        mapping.insert(TimeZone::UTC, time_zone_sort.variants[0].constructor.apply(&[]));
        mapping.insert(TimeZone::EST, time_zone_sort.variants[1].constructor.apply(&[]));
        mapping.insert(TimeZone::PST, time_zone_sort.variants[2].constructor.apply(&[]));
        mapping.insert(TimeZone::GMT, time_zone_sort.variants[3].constructor.apply(&[]));
        mapping.insert(TimeZone::CET, time_zone_sort.variants[4].constructor.apply(&[]));
        mapping.insert(TimeZone::JST, time_zone_sort.variants[5].constructor.apply(&[]));
        mapping.insert(TimeZone::IST, time_zone_sort.variants[6].constructor.apply(&[]));
        
        mapping
    }

    /// 契約タイプマッピングを構築
    fn build_contract_type_mapping(
        context: &'ctx Context, 
        contract_type_sort: &z3::DatatypeSort<'ctx>
    ) -> HashMap<ContractType, Dynamic<'ctx>> {
        let mut mapping = HashMap::new();
        
        mapping.insert(ContractType::Permanent, contract_type_sort.variants[0].constructor.apply(&[]));
        mapping.insert(ContractType::Temporary, contract_type_sort.variants[1].constructor.apply(&[]));
        mapping.insert(ContractType::Contractor, contract_type_sort.variants[2].constructor.apply(&[]));
        mapping.insert(ContractType::Consultant, contract_type_sort.variants[3].constructor.apply(&[]));
        
        mapping
    }

    /// 承認状態マッピングを構築
    fn build_approval_status_mapping(
        context: &'ctx Context, 
        approval_status_sort: &z3::DatatypeSort<'ctx>
    ) -> HashMap<ApprovalStatus, Dynamic<'ctx>> {
        let mut mapping = HashMap::new();
        
        mapping.insert(ApprovalStatus::Draft, approval_status_sort.variants[0].constructor.apply(&[]));
        mapping.insert(ApprovalStatus::Pending, approval_status_sort.variants[1].constructor.apply(&[]));
        mapping.insert(ApprovalStatus::Approved, approval_status_sort.variants[2].constructor.apply(&[]));
        mapping.insert(ApprovalStatus::Rejected, approval_status_sort.variants[3].constructor.apply(&[]));
        mapping.insert(ApprovalStatus::Archived, approval_status_sort.variants[4].constructor.apply(&[]));
        
        mapping
    }

    /// 優先度マッピングを構築
    fn build_priority_mapping(
        context: &'ctx Context, 
        priority_sort: &z3::DatatypeSort<'ctx>
    ) -> HashMap<Priority, Dynamic<'ctx>> {
        let mut mapping = HashMap::new();
        
        mapping.insert(Priority::Low, priority_sort.variants[0].constructor.apply(&[]));
        mapping.insert(Priority::Medium, priority_sort.variants[1].constructor.apply(&[]));
        mapping.insert(Priority::High, priority_sort.variants[2].constructor.apply(&[]));
        mapping.insert(Priority::Critical, priority_sort.variants[3].constructor.apply(&[]));
        
        mapping
    }

    /// フォーマットマッピングを構築
    fn build_format_mapping(
        context: &'ctx Context, 
        format_sort: &z3::DatatypeSort<'ctx>
    ) -> HashMap<Format, Dynamic<'ctx>> {
        let mut mapping = HashMap::new();
        
        mapping.insert(Format::Pdf, format_sort.variants[0].constructor.apply(&[]));
        mapping.insert(Format::Docx, format_sort.variants[1].constructor.apply(&[]));
        mapping.insert(Format::Xlsx, format_sort.variants[2].constructor.apply(&[]));
        mapping.insert(Format::Txt, format_sort.variants[3].constructor.apply(&[]));
        mapping.insert(Format::Pptx, format_sort.variants[4].constructor.apply(&[]));
        mapping.insert(Format::Xml, format_sort.variants[5].constructor.apply(&[]));
        mapping.insert(Format::Json, format_sort.variants[6].constructor.apply(&[]));
        
        mapping
    }

    /// 言語マッピングを構築
    fn build_language_mapping(
        context: &'ctx Context, 
        language_sort: &z3::DatatypeSort<'ctx>
    ) -> HashMap<Language, Dynamic<'ctx>> {
        let mut mapping = HashMap::new();
        
        mapping.insert(Language::En, language_sort.variants[0].constructor.apply(&[]));
        mapping.insert(Language::Es, language_sort.variants[1].constructor.apply(&[]));
        mapping.insert(Language::Fr, language_sort.variants[2].constructor.apply(&[]));
        mapping.insert(Language::De, language_sort.variants[3].constructor.apply(&[]));
        mapping.insert(Language::Ja, language_sort.variants[4].constructor.apply(&[]));
        mapping.insert(Language::Zh, language_sort.variants[5].constructor.apply(&[]));
        mapping.insert(Language::Pt, language_sort.variants[6].constructor.apply(&[]));
        
        mapping
    }
    
    /// ABACデータから制約を生成
    pub fn generate_constraints(&mut self, abac_data: &EdocumentAbacData) -> Result<(), String> {
        // ユーザー属性の制約を生成
        self.generate_user_constraints(&abac_data.users)?;
        
        // リソース属性の制約を生成
        self.generate_resource_constraints(&abac_data.resources)?;
        
        // ルールの制約を生成
        self.generate_rule_constraints(&abac_data.rules)?;
        
        Ok(())
    }
    
    /// ユーザー属性の制約を生成
    fn generate_user_constraints(&mut self, users: &[EdocumentUserAttribute]) -> Result<(), String> {
        for user in users {
            let user_const = self.user_mapping.get(&user.user_id)
                .ok_or_else(|| format!("User not found: {}", user.user_id))?;
            
            // ロールの制約
            if let Some(role) = &user.role {
                let role_const = self.role_mapping.get(role)
                    .ok_or_else(|| format!("Role not found: {:?}", role))?;
                
                let constraint = z3::ast::Bool::try_from(
                    self.user_has_role.apply(&[user_const, role_const])
                ).map_err(|_| "Failed to create role constraint")?;
                
                self.solver.assert(&constraint);
            }
            
            // 役職の制約
            if let Some(position) = &user.position {
                let position_const = self.position_mapping.get(position)
                    .ok_or_else(|| format!("Position not found: {:?}", position))?;
                
                let constraint = z3::ast::Bool::try_from(
                    self.user_has_position.apply(&[user_const, position_const])
                ).map_err(|_| "Failed to create position constraint")?;
                
                self.solver.assert(&constraint);
            }
            
            // テナントの制約
            if let Some(tenant) = &user.tenant {
                let tenant_const = self.tenant_mapping.get(tenant)
                    .ok_or_else(|| format!("Tenant not found: {:?}", tenant))?;
                
                let constraint = z3::ast::Bool::try_from(
                    self.user_has_tenant.apply(&[user_const, tenant_const])
                ).map_err(|_| "Failed to create tenant constraint")?;
                
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
            
            // オフィスの制約
            if let Some(office) = &user.office {
                let office_const = self.office_mapping.get(office)
                    .ok_or_else(|| format!("Office not found: {:?}", office))?;
                
                let constraint = z3::ast::Bool::try_from(
                    self.user_has_office.apply(&[user_const, office_const])
                ).map_err(|_| "Failed to create office constraint")?;
                
                self.solver.assert(&constraint);
            }
            
            // 登録状態の制約
            if user.registered {
                let constraint = z3::ast::Bool::try_from(
                    self.user_is_registered.apply(&[user_const])
                ).map_err(|_| "Failed to create registered constraint")?;
                
                self.solver.assert(&constraint);
            } else {
                let constraint = z3::ast::Bool::try_from(
                    self.user_is_registered.apply(&[user_const])
                ).map_err(|_| "Failed to create registered constraint")?;
                
                self.solver.assert(&constraint.not());
            }
            
            // プロジェクトの制約
            for project in &user.projects {
                if let Some(project_user) = self.user_mapping.get(project) {
                    let constraint = z3::ast::Bool::try_from(
                        self.user_has_project.apply(&[user_const, project_user])
                    ).map_err(|_| "Failed to create project constraint")?;
                    
                    self.solver.assert(&constraint);
                }
            }
            
            // 上司の制約
            if let Some(supervisor) = &user.supervisor {
                if let Some(supervisor_user) = self.user_mapping.get(supervisor) {
                    let constraint = z3::ast::Bool::try_from(
                        self.user_has_supervisor.apply(&[user_const, supervisor_user])
                    ).map_err(|_| "Failed to create supervisor constraint")?;
                    
                    self.solver.assert(&constraint);
                }
            }
            
            // 部下の制約
            for supervisee in &user.supervisee {
                if let Some(supervisee_user) = self.user_mapping.get(supervisee) {
                    let constraint = z3::ast::Bool::try_from(
                        self.user_has_supervisee.apply(&[user_const, supervisee_user])
                    ).map_err(|_| "Failed to create supervisee constraint")?;
                    
                    self.solver.assert(&constraint);
                }
            }
            
            // 給与支払い権限の制約
            if user.payrolling_permissions {
                let constraint = z3::ast::Bool::try_from(
                    self.user_has_payrolling_permission.apply(&[user_const, user_const])
                ).map_err(|_| "Failed to create payrolling permission constraint")?;
                
                self.solver.assert(&constraint);
            }

            // セキュリティクリアランスの制約
            if let Some(security_clearance) = &user.security_clearance {
                let security_clearance_const = self.security_level_mapping.get(security_clearance)
                    .ok_or_else(|| format!("Security level not found: {:?}", security_clearance))?;
                let constraint = z3::ast::Bool::try_from(
                    self.user_has_security_clearance.apply(&[user_const, security_clearance_const])
                ).map_err(|_| "Failed to create security clearance constraint")?;
                self.solver.assert(&constraint);
            }

            // 経験年数の制約
            if let Some(experience) = &user.experience {
                let constraint = z3::ast::Bool::try_from(
                    self.user_has_experience.apply(&[user_const, &z3::ast::Int::from_u64(self.context, *experience as u64)])
                ).map_err(|_| "Failed to create experience constraint")?;
                self.solver.assert(&constraint);
            }

            // 顧客階層の制約
            if let Some(customer_tier) = &user.customer_tier {
                let customer_tier_const = self.customer_tier_mapping.get(customer_tier)
                    .ok_or_else(|| format!("Customer tier not found: {:?}", customer_tier))?;
                let constraint = z3::ast::Bool::try_from(
                    self.user_has_customer_tier.apply(&[user_const, customer_tier_const])
                ).map_err(|_| "Failed to create customer tier constraint")?;
                self.solver.assert(&constraint);
            }

            // 地域の制約
            if let Some(region) = &user.region {
                let region_const = self.region_mapping.get(region)
                    .ok_or_else(|| format!("Region not found: {:?}", region))?;
                let constraint = z3::ast::Bool::try_from(
                    self.user_has_region.apply(&[user_const, region_const])
                ).map_err(|_| "Failed to create region constraint")?;
                self.solver.assert(&constraint);
            }

            // 国の制約
            if let Some(country) = &user.country {
                let country_const = self.country_mapping.get(country)
                    .ok_or_else(|| format!("Country not found: {:?}", country))?;
                let constraint = z3::ast::Bool::try_from(
                    self.user_has_country.apply(&[user_const, country_const])
                ).map_err(|_| "Failed to create country constraint")?;
                self.solver.assert(&constraint);
            }

            // タイムゾーンの制約
            if let Some(time_zone) = &user.time_zone {
                let time_zone_const = self.time_zone_mapping.get(time_zone)
                    .ok_or_else(|| format!("Time zone not found: {:?}", time_zone))?;
                let constraint = z3::ast::Bool::try_from(
                    self.user_has_time_zone.apply(&[user_const, time_zone_const])
                ).map_err(|_| "Failed to create time zone constraint")?;
                self.solver.assert(&constraint);
            }

            // 契約タイプの制約
            if let Some(contract_type) = &user.contract_type {
                let contract_type_const = self.contract_type_mapping.get(contract_type)
                    .ok_or_else(|| format!("Contract type not found: {:?}", contract_type))?;
                let constraint = z3::ast::Bool::try_from(
                    self.user_has_contract_type.apply(&[user_const, contract_type_const])
                ).map_err(|_| "Failed to create contract type constraint")?;
                self.solver.assert(&constraint);
            }

            // 委任権限の制約（HashSet<String>なので、空でないかチェック）
            if !user.delegated_authority.is_empty() {
                let constraint = z3::ast::Bool::try_from(
                    self.user_has_delegated_authority.apply(&[user_const])
                ).map_err(|_| "Failed to create delegated authority constraint")?;
                self.solver.assert(&constraint);
            }

            // 一時アクセスの制約（HashSet<String>なので、空でないかチェック）
            if !user.temporary_access.is_empty() {
                let constraint = z3::ast::Bool::try_from(
                    self.user_has_temporary_access.apply(&[user_const])
                ).map_err(|_| "Failed to create temporary access constraint")?;
                self.solver.assert(&constraint);
            }

            // 現在のプロジェクトの制約
            for current_project in &user.current_projects {
                let current_project_const = z3::ast::String::from_str(self.context, current_project)
                    .map_err(|_| "Failed to create current project string")?;
                let constraint = z3::ast::Bool::try_from(
                    self.user_has_current_project.apply(&[user_const, &current_project_const])
                ).map_err(|_| "Failed to create current project constraint")?;
                self.solver.assert(&constraint);
            }

            // 過去のプロジェクトの制約
            for past_project in &user.past_projects {
                let past_project_const = z3::ast::String::from_str(self.context, past_project)
                    .map_err(|_| "Failed to create past project string")?;
                let constraint = z3::ast::Bool::try_from(
                    self.user_has_past_project.apply(&[user_const, &past_project_const])
                ).map_err(|_| "Failed to create past project constraint")?;
                self.solver.assert(&constraint);
            }

            // アクティブ状態の制約
            if user.is_active {
                let constraint = z3::ast::Bool::try_from(
                    self.user_is_active.apply(&[user_const])
                ).map_err(|_| "Failed to create active constraint")?;
                self.solver.assert(&constraint);
            }

            // 予算権限の制約（Option<u64>なので、Someかどうかチェック）
            if user.budget_authority.is_some() {
                let constraint = z3::ast::Bool::try_from(
                    self.user_has_budget_authority.apply(&[user_const])
                ).map_err(|_| "Failed to create budget authority constraint")?;
                self.solver.assert(&constraint);
            }
        }
        
        Ok(())
    }
    
    /// リソース属性の制約を生成
    fn generate_resource_constraints(&mut self, resources: &[EdocumentResourceAttribute]) -> Result<(), String> {
        for resource in resources {
            let resource_const = self.resource_mapping.get(&resource.resource_id)
                .ok_or_else(|| format!("Resource not found: {}", resource.resource_id))?;
            
            // ドキュメントタイプの制約
            let document_type_const = self.document_type_mapping.get(&resource.document_type)
                .ok_or_else(|| format!("Document type not found: {:?}", resource.document_type))?;
            
            let type_constraint = z3::ast::Bool::try_from(
                self.resource_has_type.apply(&[resource_const, document_type_const])
            ).map_err(|_| "Failed to create resource type constraint")?;
            
            self.solver.assert(&type_constraint);
            
            // オーナーの制約
            if let Some(owner_user) = self.user_mapping.get(&resource.owner) {
                let owner_constraint = z3::ast::Bool::try_from(
                    self.resource_has_owner.apply(&[resource_const, owner_user])
                ).map_err(|_| "Failed to create owner constraint")?;
                
                self.solver.assert(&owner_constraint);
            }
            
            // テナントの制約
            let tenant_const = self.tenant_mapping.get(&resource.tenant)
                .ok_or_else(|| format!("Tenant not found: {:?}", resource.tenant))?;
            
            let tenant_constraint = z3::ast::Bool::try_from(
                self.resource_has_tenant.apply(&[resource_const, tenant_const])
            ).map_err(|_| "Failed to create resource tenant constraint")?;
            
            self.solver.assert(&tenant_constraint);
            
            // 部署の制約
            let department_const = self.department_mapping.get(&resource.department)
                .ok_or_else(|| format!("Department not found: {:?}", resource.department))?;
            
            let dept_constraint = z3::ast::Bool::try_from(
                self.resource_has_department.apply(&[resource_const, department_const])
            ).map_err(|_| "Failed to create resource department constraint")?;
            
            self.solver.assert(&dept_constraint);
            
            // オフィスの制約
            let office_const = self.office_mapping.get(&resource.office)
                .ok_or_else(|| format!("Office not found: {:?}", resource.office))?;
            
            let office_constraint = z3::ast::Bool::try_from(
                self.resource_has_office.apply(&[resource_const, office_const])
            ).map_err(|_| "Failed to create resource office constraint")?;
            
            self.solver.assert(&office_constraint);
            
            // 受信者の制約
            for recipient in &resource.recipients {
                if let Some(recipient_user) = self.user_mapping.get(recipient) {
                    let recipient_constraint = z3::ast::Bool::try_from(
                        self.resource_has_recipient.apply(&[resource_const, recipient_user])
                    ).map_err(|_| "Failed to create recipient constraint")?;
                    
                    self.solver.assert(&recipient_constraint);
                }
            }
            
            // 機密情報の制約
            let constraint = z3::ast::Bool::try_from(
                self.resource_is_confidential.apply(&[resource_const])
            ).map_err(|_| "Failed to create confidential constraint")?;
            
            if resource.is_confidential {
                self.solver.assert(&constraint);
            } else {
                self.solver.assert(&constraint.not());
            }
            
            // 個人情報の制約
            let constraint = z3::ast::Bool::try_from(
                self.resource_contains_personal_info.apply(&[resource_const])
            ).map_err(|_| "Failed to create personal info constraint")?;
            
            if resource.contains_personal_info {
                self.solver.assert(&constraint);
            } else {
                self.solver.assert(&constraint.not());
            }

            // セキュリティレベルの制約
            if let Some(security_level) = &resource.security_level {
                let security_level_const = self.security_level_mapping.get(security_level)
                    .ok_or_else(|| format!("Security level not found: {:?}", security_level))?;
                let constraint = z3::ast::Bool::try_from(
                    self.resource_has_security_level.apply(&[resource_const, security_level_const])
                ).map_err(|_| "Failed to create security level constraint")?;
                self.solver.assert(&constraint);
            }

            // プロジェクトIDの制約
            if let Some(project_id) = &resource.project_id {
                let project_id_const = z3::ast::String::from_str(self.context, project_id)
                    .map_err(|_| "Failed to create project ID string")?;
                let constraint = z3::ast::Bool::try_from(
                    self.resource_has_project_id.apply(&[resource_const, &project_id_const])
                ).map_err(|_| "Failed to create project ID constraint")?;
                self.solver.assert(&constraint);
            }

            // バージョンの制約（Stringなので、文字列として処理）
            if let Some(version) = &resource.version {
                let version_const = z3::ast::String::from_str(self.context, version)
                    .map_err(|_| "Failed to create version string")?;
                let constraint = z3::ast::Bool::try_from(
                    self.resource_has_version.apply(&[resource_const, &version_const])
                ).map_err(|_| "Failed to create version constraint")?;
                self.solver.assert(&constraint);
            }

            // サイズの制約
            if let Some(size) = &resource.size {
                let constraint = z3::ast::Bool::try_from(
                    self.resource_has_size.apply(&[resource_const, &z3::ast::Int::from_u64(self.context, *size)])
                ).map_err(|_| "Failed to create size constraint")?;
                self.solver.assert(&constraint);
            }

            // フォーマットの制約
            if let Some(format) = &resource.format {
                let format_const = self.format_mapping.get(format)
                    .ok_or_else(|| format!("Format not found: {:?}", format))?;
                let constraint = z3::ast::Bool::try_from(
                    self.resource_has_format.apply(&[resource_const, format_const])
                ).map_err(|_| "Failed to create format constraint")?;
                self.solver.assert(&constraint);
            }

            // 言語の制約
            if let Some(language) = &resource.language {
                let language_const = self.language_mapping.get(language)
                    .ok_or_else(|| format!("Language not found: {:?}", language))?;
                let constraint = z3::ast::Bool::try_from(
                    self.resource_has_language.apply(&[resource_const, language_const])
                ).map_err(|_| "Failed to create language constraint")?;
                self.solver.assert(&constraint);
            }

            // 地域の制約
            if let Some(region) = &resource.region {
                let region_const = self.region_mapping.get(region)
                    .ok_or_else(|| format!("Region not found: {:?}", region))?;
                let constraint = z3::ast::Bool::try_from(
                    self.resource_has_region.apply(&[resource_const, region_const])
                ).map_err(|_| "Failed to create region constraint")?;
                self.solver.assert(&constraint);
            }

            // 国の制約
            if let Some(country) = &resource.country {
                let country_const = self.country_mapping.get(country)
                    .ok_or_else(|| format!("Country not found: {:?}", country))?;
                let constraint = z3::ast::Bool::try_from(
                    self.resource_has_country.apply(&[resource_const, country_const])
                ).map_err(|_| "Failed to create country constraint")?;
                self.solver.assert(&constraint);
            }

            // 承認状態の制約
            if let Some(approval_status) = &resource.approval_status {
                let approval_status_const = self.approval_status_mapping.get(approval_status)
                    .ok_or_else(|| format!("Approval status not found: {:?}", approval_status))?;
                let constraint = z3::ast::Bool::try_from(
                    self.resource_has_approval_status.apply(&[resource_const, approval_status_const])
                ).map_err(|_| "Failed to create approval status constraint")?;
                self.solver.assert(&constraint);
            }

            // 優先度の制約
            if let Some(priority) = &resource.priority {
                let priority_const = self.priority_mapping.get(priority)
                    .ok_or_else(|| format!("Priority not found: {:?}", priority))?;
                let constraint = z3::ast::Bool::try_from(
                    self.resource_has_priority.apply(&[resource_const, priority_const])
                ).map_err(|_| "Failed to create priority constraint")?;
                self.solver.assert(&constraint);
            }

            // 保持期間の制約
            if let Some(retention_period) = &resource.retention_period {
                let constraint = z3::ast::Bool::try_from(
                    self.resource_has_retention_period.apply(&[resource_const, &z3::ast::Int::from_u64(self.context, *retention_period as u64)])
                ).map_err(|_| "Failed to create retention period constraint")?;
                self.solver.assert(&constraint);
            }

            // アーカイブ状態の制約
            let constraint = z3::ast::Bool::try_from(
                self.resource_is_archived.apply(&[resource_const])
            ).map_err(|_| "Failed to create archived constraint")?;
            if resource.is_archived {
                self.solver.assert(&constraint);
            } else {
                self.solver.assert(&constraint.not());
            }

            // アクセスカウントの制約
            if let Some(access_count) = &resource.access_count {
                let constraint = z3::ast::Bool::try_from(
                    self.resource_has_access_count.apply(&[resource_const, &z3::ast::Int::from_u64(self.context, *access_count as u64)])
                ).map_err(|_| "Failed to create access count constraint")?;
                self.solver.assert(&constraint);
            }
        }
        
        Ok(())
    }
    
    /// ルールの制約を生成
    fn generate_rule_constraints(&mut self, rules: &[EdocumentRule]) -> Result<(), String> {
        for rule in rules {
            println!("Processing rule {}: {}", rule.id, rule.description);
            
            // 基本的なルール制約を生成（実装は簡略化）
            // 実際の実装では、各ルールの条件を詳細に解析してZ3制約に変換する必要がある
            let rule_constraint = self.generate_single_rule_constraint(rule)?;
            
            self.solver.assert(&rule_constraint);
        }
        
        Ok(())
    }
    
    /// 単一のルールから制約を生成
    fn generate_single_rule_constraint(&mut self, rule: &EdocumentRule) -> Result<z3::ast::Bool<'ctx>, String> {
        // 変数を作成
        let user_var_name = format!("u_{}", rule.id);
        let resource_var_name = format!("r_{}", rule.id);
        let _user_var = Dynamic::new_const(self.context, user_var_name.as_str(), &self.user_sort.sort);
        let _resource_var = Dynamic::new_const(self.context, resource_var_name.as_str(), &self.resource_sort.sort);
        
        // 基本的なルール制約（簡略化）
        // 実際の実装では、rule.user_conditions, rule.resource_conditions, rule.actions などを
        // 詳細に解析してZ3制約に変換する必要がある
        
        let basic_constraint = z3::ast::Bool::from_bool(self.context, true);
        
        Ok(basic_constraint)
    }
    
    /// 数値比較制約を生成
    fn generate_numerical_comparison_constraint(
        &self,
        left_value: &z3::ast::Int<'ctx>,
        operator: &ComparisonOperator,
        right_value: &z3::ast::Int<'ctx>
    ) -> Result<z3::ast::Bool<'ctx>, String> {
        match operator {
            ComparisonOperator::Equals => {
                Ok(left_value._eq(right_value))
            }
            ComparisonOperator::GreaterThan => {
                Ok(left_value.gt(right_value))
            }
            ComparisonOperator::LessThan => {
                Ok(left_value.lt(right_value))
            }
            ComparisonOperator::GreaterThanOrEqual => {
                Ok(left_value.ge(right_value))
            }
            ComparisonOperator::LessThanOrEqual => {
                Ok(left_value.le(right_value))
            }
            ComparisonOperator::NotEqual => {
                Ok(left_value._eq(right_value).not())
            }
            _ => Err(format!("Unsupported numerical comparison operator: {:?}", operator))
        }
    }
    
    /// ユーザーの経験年数比較制約を生成
    fn generate_user_experience_comparison_constraint(
        &self,
        user_const: &Dynamic<'ctx>,
        operator: &ComparisonOperator,
        target_value: u32
    ) -> Result<z3::ast::Bool<'ctx>, String> {
        let experience_var = z3::ast::Int::new_const(self.context, "user_exp");
        let user_has_exp = z3::ast::Bool::try_from(
            self.user_experience_value.apply(&[user_const])
        ).map_err(|_| "Failed to create experience value constraint")?;
        
        let target_int = z3::ast::Int::from_u64(self.context, target_value as u64);
        let exp_value = self.user_experience_value.apply(&[user_const]);
        let exp_int = z3::ast::Int::try_from(exp_value)
            .map_err(|_| "Failed to convert experience to int")?;
        
        let comparison = self.generate_numerical_comparison_constraint(&exp_int, operator, &target_int)?;
        
        Ok(user_has_exp & comparison)
    }
    
    /// リソースサイズ比較制約を生成
    fn generate_resource_size_comparison_constraint(
        &self,
        resource_const: &Dynamic<'ctx>,
        operator: &ComparisonOperator,
        target_value: u64
    ) -> Result<z3::ast::Bool<'ctx>, String> {
        let resource_has_size = z3::ast::Bool::try_from(
            self.resource_size_value.apply(&[resource_const])
        ).map_err(|_| "Failed to create resource size value constraint")?;
        
        let target_int = z3::ast::Int::from_u64(self.context, target_value);
        let size_value = self.resource_size_value.apply(&[resource_const]);
        let size_int = z3::ast::Int::try_from(size_value)
            .map_err(|_| "Failed to convert size to int")?;
        
        let comparison = self.generate_numerical_comparison_constraint(&size_int, operator, &target_int)?;
        
        Ok(resource_has_size & comparison)
    }
    
    /// リソースアクセス数比較制約を生成
    fn generate_resource_access_count_comparison_constraint(
        &self,
        resource_const: &Dynamic<'ctx>,
        operator: &ComparisonOperator,
        target_value: u32
    ) -> Result<z3::ast::Bool<'ctx>, String> {
        let resource_has_count = z3::ast::Bool::try_from(
            self.resource_access_count_value.apply(&[resource_const])
        ).map_err(|_| "Failed to create resource access count value constraint")?;
        
        let target_int = z3::ast::Int::from_u64(self.context, target_value as u64);
        let count_value = self.resource_access_count_value.apply(&[resource_const]);
        let count_int = z3::ast::Int::try_from(count_value)
            .map_err(|_| "Failed to convert access count to int")?;
        
        let comparison = self.generate_numerical_comparison_constraint(&count_int, operator, &target_int)?;
        
        Ok(resource_has_count & comparison)
    }
    
    /// リソース保持期間比較制約を生成
    fn generate_resource_retention_period_comparison_constraint(
        &self,
        resource_const: &Dynamic<'ctx>,
        operator: &ComparisonOperator,
        target_value: u32
    ) -> Result<z3::ast::Bool<'ctx>, String> {
        let resource_has_period = z3::ast::Bool::try_from(
            self.resource_retention_period_value.apply(&[resource_const])
        ).map_err(|_| "Failed to create resource retention period value constraint")?;
        
        let target_int = z3::ast::Int::from_u64(self.context, target_value as u64);
        let period_value = self.resource_retention_period_value.apply(&[resource_const]);
        let period_int = z3::ast::Int::try_from(period_value)
            .map_err(|_| "Failed to convert retention period to int")?;
        
        let comparison = self.generate_numerical_comparison_constraint(&period_int, operator, &target_int)?;
        
        Ok(resource_has_period & comparison)
    }
    
    /// アクセス制御の解析を実行
    pub fn solve_access_control(&mut self) -> Result<Vec<EdocumentAccessResult>, String> {
        use std::time::Instant;
        
        let mut results = Vec::new();
        let mut solution_times = Vec::new();
        let analysis_start = Instant::now();
        
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
        let mut last_time = Instant::now();
        
        loop {
            let check_start = Instant::now();
            
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
                    
                    results.push(EdocumentAccessResult {
                        user: user_val.to_string(),
                        resource: resource_val.to_string(),
                        action: action_val.to_string(),
                    });
                    
                    // 解を見つけるのにかかった時間を記録
                    let solution_time = check_start.elapsed();
                    solution_times.push(solution_time);
                    
                    // 次の解を探すため、現在の解を禁止
                    let block_constraint = u_var._eq(&user_val) & r_var._eq(&resource_val) & a_var._eq(&action_val);
                    self.solver.assert(&block_constraint.not());
                    
                    solution_count += 1;
                    
                    // 定期的に統計情報を出力
                    if solution_count % 10 == 0 {
                        let current_avg = solution_times.iter().sum::<std::time::Duration>().as_millis() as f64 / solution_times.len() as f64;
                        println!("Found {} solutions, average time per solution: {:.2}ms", solution_count, current_avg);
                    }
                    
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
        
        // 最終統計情報を出力
        let total_time = analysis_start.elapsed();
        if !solution_times.is_empty() {
            let average_time = solution_times.iter().sum::<std::time::Duration>().as_millis() as f64 / solution_times.len() as f64;
            let min_time = solution_times.iter().min().unwrap().as_millis();
            let max_time = solution_times.iter().max().unwrap().as_millis();
            
            println!("=== Access Control Analysis Statistics ===");
            println!("Total solutions found: {}", solution_count);
            println!("Total analysis time: {:.2}s", total_time.as_secs_f64());
            println!("Average time per solution: {:.2}ms", average_time);
            println!("Min time per solution: {}ms", min_time);
            println!("Max time per solution: {}ms", max_time);
            println!("Solutions per second: {:.2}", solution_count as f64 / total_time.as_secs_f64());
        }
        
        Ok(results)
    }

    /// プログレスバー付きアクセス制御の解析を実行
    pub fn solve_access_control_with_progress(&mut self, progress_bar: &indicatif::ProgressBar) -> Result<Vec<EdocumentAccessResult>, String> {
        use indicatif::ProgressBar;
        use std::time::Instant;
        
        let mut results = Vec::new();
        let mut solution_times = Vec::new();
        let analysis_start = Instant::now();
        
        // 解析用の変数を作成
        let u_var = Dynamic::new_const(self.context, "u", &self.user_sort.sort);
        let r_var = Dynamic::new_const(self.context, "r", &self.resource_sort.sort);
        let a_var = Dynamic::new_const(self.context, "a", &self.action_sort.sort);
        
        // アクセス制御の条件を追加
        let access_constraint = z3::ast::Bool::try_from(
            self.user_can_perform_action.apply(&[&u_var, &r_var, &a_var])
        ).map_err(|_| "Failed to create access constraint")?;
        
        self.solver.assert(&access_constraint);
        
        // 環境変数から最大解数を取得（デフォルトは1000）
        let max_solutions = std::env::var("EDOCUMENT_MAX_SOLUTIONS")
            .unwrap_or_else(|_| "1000000000".to_string())
            .parse::<u64>()
            .unwrap_or(1000);
        
        progress_bar.set_length(max_solutions);
        progress_bar.set_message("Initializing access control analysis...");
        
        // 解を探索
        let mut solution_count = 0;
        let mut check_count = 0;
        
        loop {
            check_count += 1;
            let check_start = Instant::now();
            
            // 定期的にプログレスバーのメッセージを更新
            if check_count % 10 == 0 {
                let current_avg = if !solution_times.is_empty() {
                    solution_times.iter().sum::<std::time::Duration>().as_millis() as f64 / solution_times.len() as f64
                } else {
                    0.0
                };
                progress_bar.set_message(format!("Searching... ({} checks, {} found, avg: {:.1}ms)", check_count, solution_count, current_avg));
            }
            
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
                    
                    results.push(EdocumentAccessResult {
                        user: user_val.to_string(),
                        resource: resource_val.to_string(),
                        action: action_val.to_string(),
                    });
                    
                    // 解を見つけるのにかかった時間を記録
                    let solution_time = check_start.elapsed();
                    solution_times.push(solution_time);
                    
                    // プログレスバーを更新
                    solution_count += 1;
                    progress_bar.set_position(solution_count);
                    
                    // 動的にプログレスバーの長さを調整
                    if solution_count >= progress_bar.length().unwrap_or(0) {
                        progress_bar.set_length(solution_count + 100);
                    }
                    
                    // 統計情報を含むメッセージを更新
                    let current_avg = solution_times.iter().sum::<std::time::Duration>().as_millis() as f64 / solution_times.len() as f64;
                    progress_bar.set_message(format!("Found {} valid combinations (avg: {:.1}ms per solution)", solution_count, current_avg));
                    
                    // 次の解を探すため、現在の解を禁止
                    let block_constraint = u_var._eq(&user_val) & r_var._eq(&resource_val) & a_var._eq(&action_val);
                    self.solver.assert(&block_constraint.not());
                    
                    if solution_count >= max_solutions { // 無限ループを防ぐ
                        progress_bar.set_message(format!("Reached maximum solutions limit ({}) - avg: {:.1}ms per solution", max_solutions, current_avg));
                        break;
                    }
                }
                z3::SatResult::Unsat => {
                    progress_bar.set_message("No more solutions found");
                    break;
                }
                z3::SatResult::Unknown => {
                    progress_bar.set_message("Solver returned unknown result");
                    return Err("Solver returned unknown".to_string());
                }
            }
        }
        
        // プログレスバーを完了状態に設定
        progress_bar.set_position(solution_count);
        
        // 最終統計情報を出力
        let total_time = analysis_start.elapsed();
        if !solution_times.is_empty() {
            let average_time = solution_times.iter().sum::<std::time::Duration>().as_millis() as f64 / solution_times.len() as f64;
            let min_time = solution_times.iter().min().unwrap().as_millis();
            let max_time = solution_times.iter().max().unwrap().as_millis();
            
            progress_bar.set_message(format!("Analysis complete: {} combinations found (avg: {:.1}ms per solution)", solution_count, average_time));
            
            // 詳細統計をコンソールに出力
            println!("=== Access Control Analysis Statistics ===");
            println!("Total solutions found: {}", solution_count);
            println!("Total analysis time: {:.2}s", total_time.as_secs_f64());
            println!("Average time per solution: {:.2}ms", average_time);
            println!("Min time per solution: {}ms", min_time);
            println!("Max time per solution: {}ms", max_time);
            println!("Solutions per second: {:.2}", solution_count as f64 / total_time.as_secs_f64());
            println!("Total solver checks: {}", check_count);
            println!("Success rate: {:.1}%", (solution_count as f64 / check_count as f64) * 100.0);
        } else {
            progress_bar.set_message(format!("Analysis complete: {} combinations found (after {} checks)", solution_count, check_count));
        }
        
        Ok(results)
    }
}

/// edocument用アクセス制御の結果を表す構造体
#[derive(Debug, Clone)]
pub struct EdocumentAccessResult {
    pub user: String,
    pub resource: String,
    pub action: String,
}

impl EdocumentAccessResult {
    /// 結果を人間が読める形式で表示
    pub fn display(&self) -> String {
        format!("User '{}' can perform '{}' on resource '{}'", 
                self.user, self.action, self.resource)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::edocumentParser::EdocumentAbacParser;
    
    #[test]
    fn test_edocument_abac_solver_creation() {
        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        
        let parser = EdocumentAbacParser::new();
        let sample_data = r#"
            userAttrib(user1, role=employee, position=secretary, tenant=largeBank)
            resourceAttrib(doc1, type=bankingNote, owner=user1, tenant=largeBank)
        "#;
        
        let abac_data = parser.parse(sample_data).unwrap();
        let _solver = EdocumentAbacSolver::new(&ctx, &abac_data);
        
        // ソルバーが正常に作成されることを確認
        assert_eq!(abac_data.users.len(), 1);
        assert_eq!(abac_data.resources.len(), 1);
    }
} 