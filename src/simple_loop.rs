use crate::university_types::UniversityAbacData;

pub fn simple_loop(abac_data: UniversityAbacData) -> Result<(), Box<dyn std::error::Error>> {
    let users = abac_data.users;
    let resources = abac_data.resources;
    let rules = abac_data.rules;

    for rule in rules {
        let user_conditions = rule.user_conditions;
        let resource_conditions = rule.resource_conditions;
        let comparison_conditions = rule.comparison_conditions;

        for user in users {
            for user_condition in user_conditions {
                
            }
        }
    }
    
    Ok(())
}