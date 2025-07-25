use crate::edocument_types::*;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::{HashMap, HashSet};
use serde_json;

// Helper function to get a random element from a slice
fn random_choice<T>(slice: &[T]) -> Option<&T> {
    let mut rng = rand::thread_rng();
    slice.choose(&mut rng)
}

// Python's probability function
fn probability(p: f64) -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_bool(p)
}

pub fn generate_edocument_data_with_clearance(
    n_users: usize,
    n_documents: usize,
    n_helpdesk_operators: usize,
    n_application_admins: usize,
    n_customers: usize,
) -> EdocumentAbacData {
    let mut users: Vec<EdocumentUserAttribute> = Vec::new();
    let mut documents: Vec<EdocumentResourceAttribute> = Vec::new();

    // Definitions from python script
    let tenants = vec![
        Tenant::LargeBank, Tenant::LargeBankLeasing, Tenant::NewsAgency,
        Tenant::EuropeRegion, Tenant::LondonOffice, Tenant::Reseller,
    ];
    let customer_tenants = vec![
        Tenant::CarLeaser, Tenant::IctProvider, Tenant::PrivateReceiver,
    ];
    let positions = vec![
        Position::Secretary, Position::Director, Position::SeniorOfficeManager,
        Position::OfficeManager, Position::InsuranceAgent,
    ];
    let document_types = vec![
        DocumentType::Invoice, DocumentType::Contract, DocumentType::Paycheck,
        DocumentType::BankingNote, DocumentType::SalesOffer, DocumentType::TrafficFine,
    ];

    let mut departments: HashMap<Tenant, Vec<String>> = HashMap::new();
    departments.insert(Tenant::LargeBank, vec!["largeBankSales".to_string(), "largeBankICT".to_string(), "largeBankHR".to_string(), "largeBankIT".to_string(), "largeBankAudit".to_string()]);
    departments.insert(Tenant::LargeBankLeasing, vec!["largeBankLeasingCustomerCare".to_string(), "largeBankLeasingSales".to_string()]);
    // ... Add all other departments as in the python script

    let mut offices: HashMap<Tenant, usize> = HashMap::new();
    offices.insert(Tenant::LargeBank, 10);
    offices.insert(Tenant::LargeBankLeasing, 2);
    offices.insert(Tenant::IctProvider, 5);
    offices.insert(Tenant::NewsAgency, 0);
    offices.insert(Tenant::EuropeRegion, 0);
    offices.insert(Tenant::LondonOffice, 0);
    offices.insert(Tenant::Reseller, 0);
    offices.insert(Tenant::CarLeaser, 0);
    offices.insert(Tenant::PrivateReceiver, 0);


    // Generate Users
    for i in 0..n_users {
        let mut user = EdocumentUserAttribute::new(format!("user{}", i));
        let tenant = random_choice(&tenants).unwrap().clone();
        
        user.tenant = Some(tenant.clone());
        user.role = Some(Role::Employee);
        user.position = random_choice(&positions).map(|p| p.clone());
        user.department = departments.get(&tenant).and_then(|deps| random_choice(deps)).map(|d| d.clone());
        
        if offices.get(&tenant).unwrap_or(&0) > &0 {
            let office_num = rand::thread_rng().gen_range(1..=*offices.get(&tenant).unwrap());
            user.office = Some(format!("{:?}Office{}", tenant, office_num));
        }

        user.payrolling_permissions = Some(probability(0.1));
        user.registered = Some(probability(0.8));
        user.clearance_level = Some(rand::thread_rng().gen_range(1..=5));
        users.push(user);
    }
    
    // Generate Documents
    for i in 0..n_documents {
        let owner = random_choice(&users).unwrap();
        let doc_type = random_choice(&document_types).unwrap();
        let doc_type_str = match doc_type {
            DocumentType::Invoice => "invoice",
            DocumentType::Contract => "contract",
            DocumentType::Paycheck => "paycheck",
            DocumentType::BankingNote => "bankingNote",
            DocumentType::SalesOffer => "salesOffer",
            DocumentType::TrafficFine => "trafficFine",
            DocumentType::None => "none",
        };
        let mut doc = EdocumentResourceAttribute::new(format!("doc{}", i), doc_type_str).unwrap();

        doc.owner = Some(owner.user_id.clone());
        doc.tenant = owner.tenant.clone();
        doc.department = owner.department.clone();
        doc.office = owner.office.clone();
        doc.is_confidential = Some(probability(0.6));
        doc.contains_personal_info = Some(probability(0.2));
        doc.security_level = Some(rand::thread_rng().gen_range(1..=5));
        
        // Simplified recipient logic
        let mut recipients = HashSet::new();
        for _ in 0..5 {
            if let Some(user) = random_choice(&users) {
                recipients.insert(user.user_id.clone());
            }
        }
        doc.recipients = recipients;

        documents.push(doc);
    }


    // Generate Rules
    let mut rules: Vec<EdocumentRule> = Vec::new();

    // Combined Rule: GreaterThanOrEqual AND ContainedIn AND Equal (user) AND Equal (resource)
    rules.push(EdocumentRule {
        id: 1,
        description: "Combined Rule: clearance_level >= security_level AND uid in recipients AND role = employee AND type = invoice.".to_string(),
        user_conditions: vec![
            Condition {
                left: AttributeExpression::AttributeName(AttributeName::Role),
                operator: ComparisonOperator::Equals,
                right: AttributeExpression::AttributeValue(AttributeValue::Role(Role::Employee)),
            },
        ],
        resource_conditions: vec![
            Condition {
                left: AttributeExpression::AttributeName(AttributeName::Type),
                operator: ComparisonOperator::Equals,
                right: AttributeExpression::AttributeValue(AttributeValue::ResourceType(DocumentType::Invoice)),
            },
        ],
        actions: vec![Action::View].into_iter().collect(),
        comparison_conditions: vec![
            // clearance_level >= security_level
            Condition {
                left: AttributeExpression::AttributeName(AttributeName::ClearanceLevel),
                operator: ComparisonOperator::GreaterThanOrEqual,
                right: AttributeExpression::AttributeName(AttributeName::SecurityLevel),
            },
            // uid ContainedIn recipients
            Condition {
                left: AttributeExpression::AttributeName(AttributeName::Uid),
                operator: ComparisonOperator::ContainedIn,
                right: AttributeExpression::AttributeName(AttributeName::Recipients),
            },
            // department = department (user's department equals resource's department)
            Condition {
                left: AttributeExpression::AttributeName(AttributeName::Department),
                operator: ComparisonOperator::Equals,
                right: AttributeExpression::AttributeName(AttributeName::Department),
            },
        ],
    });

    // A simple rule allowing owners to view their own documents (as a fallback/additional rule)
    rules.push(EdocumentRule {
        id: 2,
        description: "Owner can view their own document.".to_string(),
        user_conditions: vec![],
        resource_conditions: vec![],
        actions: vec![Action::View].into_iter().collect(),
        comparison_conditions: vec![
            Condition {
                left: AttributeExpression::AttributeName(AttributeName::Uid),
                operator: ComparisonOperator::Equals,
                right: AttributeExpression::AttributeName(AttributeName::Owner),
            }
        ],
    });

    EdocumentAbacData {
        users,
        resources: documents,
        rules,
    }
}

pub fn generate_and_save_json() {
    println!("Generating realistic e-document data...");
    let data = generate_edocument_data_with_clearance(10000000, 10000000, 30, 30, 40);
    let json_data = serde_json::to_string_pretty(&data).unwrap();
    let output_path = "output/edocument_with_clearance.json";
    std::fs::create_dir_all("output").unwrap();
    std::fs::write(output_path, json_data).unwrap();
    println!("Generated data and saved to {}", output_path);
}
