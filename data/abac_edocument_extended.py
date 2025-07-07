import random
from collections import defaultdict
from datetime import datetime, timedelta
import json
import os

# Set random seed for reproducibility
random.seed(42)

# probability helper
def probability(p):
    return random.random() < p

# Enhanced params for scale - Large Dataset
nUsers = 10000
nDocuments = 10000
nHelpdeskOperators = 200
nApplicationAdmins = 150
nCustomers = 500
nProjectManagers = 300
nLegalOfficers = 100
nFinancialOfficers = 150
nAuditors = 200
nConsultants = 250

# Enhanced roles
roles = ["employee", "manager", "admin", "helpdesk", "customer", "projectManager", 
         "legalOfficer", "financialOfficer", "auditor", "consultant"]

# Enhanced positions
positions = ["secretary", "director", "seniorOfficeManager", "officeManager", 
             "insuranceAgent", "analyst", "specialist", "coordinator", "lead", 
             "associate", "senior", "principal", "vicePresident"]

# Enhanced document types
documentTypes = ["invoice", "contract", "paycheck", "bankingNote", "salesOffer", 
                "trafficFine", "legalDocument", "technicalSpecification", 
                "financialReport", "auditReport", "hrDocument", "policyDocument",
                "projectPlan", "meetingMinutes", "complianceReport", "riskAssessment",
                "budgetReport", "performanceReview", "strategicPlan", "marketAnalysis"]

# Geographic attributes
regions = ["NorthAmerica", "Europe", "Asia", "LatinAmerica", "Africa"]
countries = {
    "NorthAmerica": ["USA", "Canada", "Mexico"],
    "Europe": ["UK", "Germany", "France", "Netherlands", "Spain"],
    "Asia": ["Japan", "China", "India", "Singapore", "SouthKorea"],
    "LatinAmerica": ["Brazil", "Argentina", "Chile", "Colombia"],
    "Africa": ["SouthAfrica", "Nigeria", "Egypt", "Kenya"]
}

# Security levels
securityLevels = ["public", "internal", "confidential", "secret", "topSecret"]

# Customer tiers
customerTiers = ["bronze", "silver", "gold", "platinum", "vip"]

# Project phases
projectPhases = ["initiation", "planning", "execution", "monitoring", "closure"]

# Time periods
timeZones = ["UTC", "EST", "PST", "GMT", "CET", "JST", "IST"]

# Enhanced tenants/organizations
tenants = ["largeBank", "largeBankLeasing", "newsAgency", "europeRegion", 
           "londonOffice", "reseller", "techCorp", "pharmaceuticals", 
           "energyCompany", "retailChain", "consultingFirm", "lawFirm",
           "financialServices", "healthcareSystem", "educationInstitute"]

customerTenants = ["carLeaser", "ictProvider", "privateReceiver", "smallBusiness",
                  "startupCompany", "governmentAgency", "nonprofit", "mediaCompany"]

# Enhanced departments structure
departments = {
    "largeBank": ["largeBankSales", "largeBankICT", "largeBankHR", "largeBankIT", 
                  "largeBankAudit", "largeBankLegal", "largeBankRisk", "largeBankCompliance"],
    "largeBankLeasing": ["largeBankLeasingCustomerCare", "largeBankLeasingSales",
                        "largeBankLeasingRisk", "largeBankLeasingOperations"],
    "newsAgency": ["newsAgencyAudit", "newsAgencyIT", "newsAgencyEditorial", 
                   "newsAgencyMarketing", "newsAgencyLegal"],
    "europeRegion": ["europeRegionIT", "europeRegionHR", "europeRegionFinance",
                     "europeRegionStrategy", "europeRegionCompliance"],
    "londonOffice": ["londonOfficeAudit", "londonOfficeHR", "londonOfficeSales",
                     "londonOfficeOperations", "londonOfficeResearch"],
    "reseller": ["resellerSales", "resellerCustomer", "resellerAccounting",
                 "resellerSupport", "resellerMarketing"],
    "techCorp": ["techCorpEngineering", "techCorpProduct", "techCorpSales",
                 "techCorpSupport", "techCorpResearch", "techCorpSecurity"],
    "pharmaceuticals": ["pharmaceuticalsResearch", "pharmaceuticalsRegulatory",
                       "pharmaceuticalsSales", "pharmaceuticalsManufacturing"],
    "energyCompany": ["energyCompanyExploration", "energyCompanyProduction",
                      "energyCompanyRefining", "energyCompanyTradingAudit"],
    "retailChain": ["retailChainOperations", "retailChainMarketing", 
                    "retailChainSupplyChain", "retailChainFinance"],
    "consultingFirm": ["consultingFirmStrategy", "consultingFirmTechnology",
                       "consultingFirmOperations", "consultingFirmHR"],
    "lawFirm": ["lawFirmCorporate", "lawFirmLitigation", "lawFirmIntellectualProperty",
                "lawFirmTax", "lawFirmEmployment"],
    "financialServices": ["financialServicesWealth", "financialServicesRetail",
                         "financialServicesInvestment", "financialServicesRisk"],
    "healthcareSystem": ["healthcareSystemClinical", "healthcareSystemAdministrative",
                        "healthcareSystemResearch", "healthcareSystemIT"],
    "educationInstitute": ["educationInstituteAcademic", "educationInstituteAdministrative",
                          "educationInstituteResearch", "educationInstituteIT"]
}

# Enhanced customer tenant departments
def get_customer_departments(tenant):
    base_departments = ["Audit", "Secretary", "Accounting", "Operations", "Marketing"]
    if tenant == "ictProvider":
        base_departments.extend(["ICT", "Development", "Support"])
    elif tenant == "startupCompany":
        base_departments.extend(["Product", "Engineering", "BusinessDevelopment"])
    elif tenant == "governmentAgency":
        base_departments.extend(["Policy", "PublicRelations", "Compliance"])
    return [f"{tenant}{dept}" for dept in base_departments]

for customerTenant in customerTenants:
    departments[customerTenant] = get_customer_departments(customerTenant)

# Enhanced office structure
offices = {
    "largeBank": 15,
    "largeBankLeasing": 5,
    "ictProvider": 8,
    "newsAgency": 6,
    "europeRegion": 12,
    "londonOffice": 4,
    "reseller": 7,
    "techCorp": 20,
    "pharmaceuticals": 18,
    "energyCompany": 25,
    "retailChain": 50,
    "consultingFirm": 10,
    "lawFirm": 8,
    "financialServices": 15,
    "healthcareSystem": 12,
    "educationInstitute": 8
}

for customerTenant in customerTenants:
    offices[customerTenant] = random.randint(1, 5)

# Enhanced Organization class
class Organization:
    def __init__(self, orgId, departments, offices):
        self.orgId = orgId
        self.departments = departments
        self.offices = offices
        self.region = random.choice(regions)
        self.country = random.choice(countries[self.region])
        self.foundedYear = random.randint(1950, 2020)
        self.size = random.choice(["small", "medium", "large", "enterprise"])
        self.industry = self.get_industry()
        
    def get_industry(self):
        industry_mapping = {
            "largeBank": "financial", "largeBankLeasing": "financial",
            "newsAgency": "media", "techCorp": "technology",
            "pharmaceuticals": "healthcare", "energyCompany": "energy",
            "retailChain": "retail", "consultingFirm": "consulting",
            "lawFirm": "legal", "financialServices": "financial",
            "healthcareSystem": "healthcare", "educationInstitute": "education"
        }
        return industry_mapping.get(self.orgId, "other")

# Enhanced User class
class User:
    def __init__(self, userId, role, organization, department=None, office=None, position=None):
        self.userId = userId
        self.role = role
        self.organization = organization
        self.department = department if department else "none"
        self.office = office if office else "none"
        self.position = position if position else "none"
        self.projects = set()
        self.supervisor = None
        self.supervisee = set()
        self.payrollingPermissions = False
        self.registered = False
        
        # Enhanced attributes
        self.securityClearance = random.choice(securityLevels)
        self.experience = random.randint(0, 30)  # years of experience
        self.customerTier = random.choice(customerTiers) if role == "customer" else "none"
        self.region = organization.region
        self.country = organization.country
        self.city = self.get_city()
        self.timeZone = random.choice(timeZones)
        self.workingHours = self.get_working_hours()
        self.temporaryAccess = set()  # temporary access to resources
        self.delegatedAuthority = set()  # users who delegated authority to this user
        self.currentProjects = set()
        self.pastProjects = set()
        self.certifications = self.get_certifications()
        self.isActive = probability(0.95)
        self.lastLogin = self.get_last_login()
        self.contractType = random.choice(["permanent", "temporary", "contractor", "consultant"])
        self.budgetAuthority = random.randint(0, 10000000) if role in ["manager", "director"] else 0
        
    def get_city(self):
        city_mapping = {
            "USA": ["NewYork", "LosAngeles", "Chicago", "Houston", "Phoenix"],
            "Canada": ["Toronto", "Vancouver", "Montreal", "Calgary", "Ottawa"],
            "UK": ["London", "Manchester", "Birmingham", "Edinburgh", "Glasgow"],
            "Germany": ["Berlin", "Munich", "Frankfurt", "Hamburg", "Cologne"],
            "France": ["Paris", "Lyon", "Marseille", "Toulouse", "Nice"],
            "Japan": ["Tokyo", "Osaka", "Yokohama", "Nagoya", "Sapporo"],
            "China": ["Beijing", "Shanghai", "Guangzhou", "Shenzhen", "Chengdu"]
        }
        return random.choice(city_mapping.get(self.country, ["DefaultCity"]))
    
    def get_working_hours(self):
        start_hour = random.randint(7, 10)
        end_hour = start_hour + random.randint(8, 10)
        return f"{start_hour:02d}:00-{end_hour:02d}:00"
    
    def get_certifications(self):
        all_certs = ["PMP", "CISSP", "CPA", "MBA", "PhD", "JD", "CFA", "FRM", "CISA", "CRISC"]
        return set(random.sample(all_certs, random.randint(0, 3)))
    
    def get_last_login(self):
        days_ago = random.randint(0, 30)
        return (datetime.now() - timedelta(days=days_ago)).strftime("%Y-%m-%d")

    def __str__(self):
        attributes = [
            f"role={self.role}",
            f"position={self.position}",
            f"tenant={self.organization.orgId}",
            f"department={self.department}",
            f"office={self.office}",
            f"registered={'True' if self.registered else 'False'}",
            f"projects={{{' '.join(self.projects)}}}",
            f"supervisor={self.supervisor if self.supervisor else 'none'}",
            f"supervisee={{{' '.join(self.supervisee)}}}",
            f"payrollingPermissions={self.payrollingPermissions}",
            f"securityClearance={self.securityClearance}",
            f"experience={self.experience}",
            f"customerTier={self.customerTier}",
            f"region={self.region}",
            f"country={self.country}",
            f"city={self.city}",
            f"timeZone={self.timeZone}",
            f"workingHours={self.workingHours}",
            f"temporaryAccess={{{' '.join(self.temporaryAccess)}}}",
            f"delegatedAuthority={{{' '.join(self.delegatedAuthority)}}}",
            f"currentProjects={{{' '.join(self.currentProjects)}}}",
            f"pastProjects={{{' '.join(self.pastProjects)}}}",
            f"certifications={{{' '.join(self.certifications)}}}",
            f"isActive={'True' if self.isActive else 'False'}",
            f"lastLogin={self.lastLogin}",
            f"contractType={self.contractType}",
            f"budgetAuthority={self.budgetAuthority}"
        ]
        return f"userAttrib({self.userId}, {', '.join(attributes)})"

# Enhanced Document class
class Document:
    def __init__(self, docId, docType, owner, confidential=False, containsPersonalInfo=False):
        self.docId = docId
        self.docType = docType
        self.owner = owner
        self.confidential = confidential
        self.containsPersonalInfo = containsPersonalInfo
        self.tenant = owner.organization.orgId
        self.department = owner.department
        self.office = f"{self.tenant}Office{random.randint(1, offices[self.tenant])}" if offices[self.tenant] > 0 else "none"
        self.recipients = set()
        
        # Enhanced attributes
        self.securityLevel = random.choice(securityLevels)
        self.createdDate = self.get_created_date()
        self.expiryDate = self.get_expiry_date()
        self.projectId = self.get_project_id()
        self.version = f"{random.randint(1, 10)}.{random.randint(0, 9)}"
        self.size = random.randint(1, 10000)  # KB
        self.format = random.choice(["pdf", "docx", "xlsx", "txt", "pptx", "xml", "json"])
        self.language = random.choice(["en", "es", "fr", "de", "ja", "zh", "pt"])
        self.region = owner.region
        self.country = owner.country
        self.approvalStatus = random.choice(["draft", "pending", "approved", "rejected", "archived"])
        self.reviewers = set()
        self.approvers = set()
        self.relatedDocuments = set()
        self.tags = self.get_tags()
        self.complianceRequirements = self.get_compliance_requirements()
        self.retentionPeriod = random.randint(1, 10)  # years
        self.isArchived = probability(0.1)
        self.lastModified = self.get_last_modified()
        self.accessCount = random.randint(0, 1000)
        self.priority = random.choice(["low", "medium", "high", "critical"])
        
        # Recipients will be set later when all users are generated
        
    def get_created_date(self):
        days_ago = random.randint(0, 365)
        return (datetime.now() - timedelta(days=days_ago)).strftime("%Y-%m-%d")
    
    def get_expiry_date(self):
        if probability(0.7):  # 70% of documents have expiry
            days_ahead = random.randint(30, 3650)  # 1 month to 10 years
            return (datetime.now() + timedelta(days=days_ahead)).strftime("%Y-%m-%d")
        return "none"
    
    def get_project_id(self):
        if probability(0.6):  # 60% of documents are project-related
            return f"proj{random.randint(1, 200)}"
        return "none"
    
    def get_last_modified(self):
        days_ago = random.randint(0, 60)
        return (datetime.now() - timedelta(days=days_ago)).strftime("%Y-%m-%d")
    
    def get_tags(self):
        all_tags = ["financial", "legal", "technical", "strategic", "operational", 
                   "compliance", "audit", "hr", "marketing", "research", "development"]
        return set(random.sample(all_tags, random.randint(1, 4)))
    
    def get_compliance_requirements(self):
        requirements = ["GDPR", "HIPAA", "SOX", "PCI-DSS", "ISO27001", "NIST", "COBIT"]
        return set(random.sample(requirements, random.randint(0, 3)))
    
    def set_recipients(self, all_users):
        if self.office != "none":
            office_users = [user for user in all_users if user.office == self.office]
            if office_users:
                self.recipients.update(office_users[:random.randint(1, min(5, len(office_users)))])
        
        # Add tenant users
        tenant_users = [user for user in all_users if user.organization.orgId == self.tenant]
        if tenant_users:
            self.recipients.update(random.sample(tenant_users, min(random.randint(1, 8), len(tenant_users))))
        
        # Add department users
        dept_users = [user for user in all_users if user.department == self.department]
        if dept_users:
            self.recipients.update(random.sample(dept_users, min(random.randint(1, 5), len(dept_users))))

    def __str__(self):
        attributes = [
            f"type={self.docType}",
            f"owner={self.owner.userId}",
            f"tenant={self.tenant}",
            f"department={self.department}",
            f"office={self.office}",
            f"recipients={{{' '.join(user.userId for user in self.recipients)}}}",
            f"isConfidential={'True' if self.confidential else 'False'}",
            f"containsPersonalInfo={'True' if self.containsPersonalInfo else 'False'}",
            f"securityLevel={self.securityLevel}",
            f"createdDate={self.createdDate}",
            f"expiryDate={self.expiryDate}",
            f"projectId={self.projectId}",
            f"version={self.version}",
            f"size={self.size}",
            f"format={self.format}",
            f"language={self.language}",
            f"region={self.region}",
            f"country={self.country}",
            f"approvalStatus={self.approvalStatus}",
            f"reviewers={{{' '.join(self.reviewers)}}}",
            f"approvers={{{' '.join(self.approvers)}}}",
            f"relatedDocuments={{{' '.join(self.relatedDocuments)}}}",
            f"tags={{{' '.join(self.tags)}}}",
            f"complianceRequirements={{{' '.join(self.complianceRequirements)}}}",
            f"retentionPeriod={self.retentionPeriod}",
            f"isArchived={'True' if self.isArchived else 'False'}",
            f"lastModified={self.lastModified}",
            f"accessCount={self.accessCount}",
            f"priority={self.priority}"
        ]
        return f"resourceAttrib({self.docId}, {', '.join(attributes)})"

# Generate organizations
organizations = {}
for tenant in tenants:
    organizations[tenant] = Organization(tenant, departments[tenant], offices[tenant])
for customerTenant in customerTenants:
    organizations[customerTenant] = Organization(customerTenant, departments[customerTenant], offices[customerTenant])

# Generate users
print("Generating users...")
users = []
users_by_department = defaultdict(list)
users_by_role = defaultdict(list)

# Generate regular employees
for i in range(nUsers):
    tenant = random.choice(tenants)
    
    valid_positions = positions if offices[tenant] > 0 else [p for p in positions if p not in ["secretary", "director"]]
    position = random.choice(valid_positions)
    
    office = "none"
    if offices[tenant] > 0 or position in ["secretary", "director"]:
        office = f"{tenant}Office{random.randint(1, offices[tenant])}" if offices[tenant] > 0 else "none"

    department = random.choice(departments[tenant])
    user = User(f"user{i}", "employee", organizations[tenant], department, office, position)
    users.append(user)
    users_by_department[(tenant, department)].append(user)
    users_by_role["employee"].append(user)
    
    if (i + 1) % 1000 == 0:
        print(f"Generated {i + 1}/{nUsers} users")

# Generate specialized roles
specialized_roles = [
    ("hdop", "helpdesk", nHelpdeskOperators),
    ("admin", "admin", nApplicationAdmins),
    ("pm", "projectManager", nProjectManagers),
    ("legal", "legalOfficer", nLegalOfficers),
    ("finance", "financialOfficer", nFinancialOfficers),
    ("audit", "auditor", nAuditors),
    ("cons", "consultant", nConsultants)
]

for prefix, role, count in specialized_roles:
    for i in range(count):
        tenant = random.choice(tenants)
        department = random.choice(departments[tenant])
        user = User(f"{prefix}{i}", role, organizations[tenant], department)
        users.append(user)
        users_by_role[role].append(user)

# Generate customers
for i in range(nCustomers):
    tenant = random.choice(customerTenants)
    department = random.choice(departments[tenant])
    user = User(f"cstmr{i}", "customer", organizations[tenant], department)
    users.append(user)
    users_by_role["customer"].append(user)

# Enhanced supervisor logic
position_hierarchy = {
    "secretary": 1, "associate": 2, "analyst": 3, "specialist": 4, "coordinator": 5,
    "officeManager": 6, "lead": 7, "senior": 8, "seniorOfficeManager": 9,
    "principal": 10, "director": 11, "vicePresident": 12
}

for (tenant, department), dept_users in users_by_department.items():
    dept_users.sort(key=lambda u: position_hierarchy.get(u.position, 0), reverse=True)
    for i, user in enumerate(dept_users):
        if i > 0:
            user.supervisor = dept_users[i - 1].userId
            dept_users[i - 1].supervisee.add(user.userId)

# Update registered attribute
for user in users:
    if user.supervisee or user.role in ["manager", "director", "projectManager"]:
        user.registered = True
    else:
        user.registered = probability(0.7)

# Assign enhanced permissions
for (tenant, department), dept_users in users_by_department.items():
    if dept_users:
        random.choice(dept_users).payrollingPermissions = True
        for user in dept_users:
            if user.role in ["manager", "admin", "financialOfficer"] or probability(0.3):
                user.payrollingPermissions = True

# Generate projects and assign them
print("Generating projects...")
projects = []
num_projects = 1000  # Increased for larger dataset
for i in range(num_projects):
    project_id = f"proj{i}"
    project_owner = random.choice(users)
    project_phase = random.choice(projectPhases)
    project_budget = random.randint(10000, 10000000)
    
    # Assign project to users (optimized for large dataset)
    project_team_size = random.randint(3, 20)
    project_team = random.sample(users, min(project_team_size, len(users)))
    
    for user in project_team:
        user.currentProjects.add(project_id)
        user.projects.add(project_id)
    
    if (i + 1) % 100 == 0:
        print(f"Generated {i + 1}/{num_projects} projects")

# Generate past projects
for user in users:
    if probability(0.6):  # 60% of users have past projects
        past_project_count = random.randint(1, 5)
        for _ in range(past_project_count):
            past_project = f"pastProj{random.randint(1, 500)}"
            user.pastProjects.add(past_project)

# Generate temporary access
for user in users:
    if probability(0.2):  # 20% of users have temporary access
        temp_access_count = random.randint(1, 3)
        for _ in range(temp_access_count):
            temp_resource = f"tempRes{random.randint(1, 100)}"
            user.temporaryAccess.add(temp_resource)

# Generate delegation relationships
for user in users:
    if user.role in ["manager", "director", "projectManager"] and probability(0.3):
        potential_delegates = [u for u in users if u.department == user.department and u != user]
        if potential_delegates:
            delegate = random.choice(potential_delegates)
            delegate.delegatedAuthority.add(user.userId)

# Generate documents
print("Generating documents...")
documents = []
for i in range(nDocuments):
    owner = random.choice(users)
    docType = random.choice(documentTypes)
    confidential = probability(0.4)
    containsPersonalInfo = probability(0.3)
    doc = Document(f"doc{i}", docType, owner, confidential, containsPersonalInfo)
    documents.append(doc)
    
    if (i + 1) % 1000 == 0:
        print(f"Generated {i + 1}/{nDocuments} documents")

# Set document recipients after all users are generated
print("Setting document recipients...")
for i, doc in enumerate(documents):
    doc.set_recipients(users)
    if (i + 1) % 1000 == 0:
        print(f"Set recipients for {i + 1}/{nDocuments} documents")

# Set document relationships
print("Setting document relationships...")
for i, doc in enumerate(documents):
    # Assign to owner's projects
    if doc.owner.projects:
        doc.owner.projects.add(doc.docId)
    
    # Set reviewers and approvers
    if doc.approvalStatus in ["pending", "approved"]:
        potential_reviewers = [u for u in users if u.department == doc.department and u != doc.owner]
        if potential_reviewers:
            doc.reviewers = set(random.sample([u.userId for u in potential_reviewers], 
                                            min(random.randint(1, 3), len(potential_reviewers))))
        
        potential_approvers = [u for u in users if u.role in ["manager", "director", "admin"] 
                              and u.department == doc.department]
        if potential_approvers:
            doc.approvers = set(random.sample([u.userId for u in potential_approvers], 
                                            min(random.randint(1, 2), len(potential_approvers))))
    
    # Set related documents (reduced probability for large dataset)
    if probability(0.1):  # 10% of documents have related documents
        related_docs = random.sample([d.docId for d in documents if d != doc], 
                                   min(random.randint(1, 2), len(documents) - 1))
        doc.relatedDocuments = set(related_docs)
    
    if (i + 1) % 1000 == 0:
        print(f"Set relationships for {i + 1}/{nDocuments} documents")

# Output enhanced ABAC file
filename = "edocument_extended_large.abac"
print(f"Writing {filename}...")
with open(filename, "w", encoding="utf-8") as f:
    f.write("# Enhanced ABAC policy for comprehensive document management system.\n")
    f.write("# Generated with extended attributes, roles, and complex relationships.\n")
    f.write("# Large dataset: 10,000 users and 10,000 documents\n\n")
    
    f.write("#------------------------------------------------------------\n")
    f.write("# User Attribute Data\n")
    f.write("#------------------------------------------------------------\n\n")
    for i, user in enumerate(users):
        f.write(str(user) + "\n")
        if (i + 1) % 1000 == 0:
            print(f"Written {i + 1}/{len(users)} users")
    
    f.write("\n#------------------------------------------------------------\n")
    f.write("# Resource Attribute Data\n")
    f.write("#------------------------------------------------------------\n\n")
    for i, doc in enumerate(documents):
        f.write(str(doc) + "\n")
        if (i + 1) % 1000 == 0:
            print(f"Written {i + 1}/{len(documents)} documents")

    f.write("\n#------------------------------------------------------------\n")
    f.write("# Enhanced ABAC Rules\n")
    f.write("#------------------------------------------------------------\n\n")

    # Original rules (enhanced)
    f.write("# 1. Unregistered customers can only view documents sent to them\n")
    f.write("rule(role [ {customer}, registered [ {False}; ; {view}; uid [ recipients)\n\n")

    f.write("# 2. Helpdesk members can search and view meta-information of documents\n")
    f.write("rule(role [ {helpdesk}; ; {search readMetaInfo}; uid [ recipients)\n\n")

    f.write("# 3. Helpdesk can read non-confidential documents in their tenant\n")
    f.write("rule(role [ {helpdesk}; isConfidential [ {False}; {view}; tenant = tenant)\n\n")

    f.write("# 4. Application admins can view non-confidential documents\n")
    f.write("rule(role [ {admin}; isConfidential [ {False}; {view}; )\n\n")

    # Enhanced rules with new attributes
    f.write("# 5. Supervisors can read documents from their supervisees\n")
    f.write("rule(role [ {employee}, registered [ {True}; ; {view}; supervisee ] owner)\n\n")

    f.write("# 6. Project members can read documents related to their current projects\n")
    f.write("rule(role [ {employee projectManager}; ; {view}; currentProjects ] projectId)\n\n")

    f.write("# 7. Users with security clearance can access documents of same or lower level\n")
    f.write("rule(securityClearance [ {secret topSecret}; securityLevel [ {public internal confidential}; {view}; )\n\n")

    f.write("# 8. Financial officers can access all financial documents\n")
    f.write("rule(role [ {financialOfficer}; tags ] {financial}; {view send edit}; )\n\n")

    f.write("# 9. Legal officers can access all legal documents\n")
    f.write("rule(role [ {legalOfficer}; tags ] {legal}; {view send edit}; )\n\n")

    f.write("# 10. Auditors can read all documents except those containing personal info\n")
    f.write("rule(role [ {auditor}; containsPersonalInfo [ {False}; {view}; )\n\n")

    f.write("# 11. Regional managers can access documents from their region\n")
    f.write("rule(role [ {manager}, position [ {director vicePresident}; ; {view}; region = region)\n\n")

    f.write("# 12. Users can only access documents during their working hours\n")
    f.write("rule(isActive [ {True}; ; {view}; workingHours = currentTime)\n\n")

    f.write("# 13. Consultants can only access documents they are explicitly recipients of\n")
    f.write("rule(role [ {consultant}; ; {view}; uid [ recipients)\n\n")

    f.write("# 14. Users with temporary access can view specific resources\n")
    f.write("rule(; ; {view}; temporaryAccess ] rid)\n\n")

    f.write("# 15. Delegated authority allows access to supervisor's documents\n")
    f.write("rule(; ; {view}; delegatedAuthority ] owner)\n\n")

    f.write("# 16. Project managers can access all documents in their projects\n")
    f.write("rule(role [ {projectManager}; ; {view send edit}; currentProjects ] projectId)\n\n")

    f.write("# 17. Users with budget authority can approve financial documents\n")
    f.write("rule(budgetAuthority > 100000; tags ] {financial}; {approve}; )\n\n")

    f.write("# 18. Document reviewers can edit documents in review status\n")
    f.write("rule(approvalStatus [ {pending}; ; {edit}; uid [ reviewers)\n\n")

    f.write("# 19. Document approvers can approve documents\n")
    f.write("rule(approvalStatus [ {pending}; ; {approve}; uid [ approvers)\n\n")

    f.write("# 20. Users can access documents in same language and region\n")
    f.write("rule(; language = language, region = region; {view}; )\n\n")

    f.write("# 21. High-priority documents require high security clearance\n")
    f.write("rule(securityClearance [ {secret topSecret}; priority [ {high critical}; {view}; )\n\n")

    f.write("# 22. Archived documents can only be accessed by admins and auditors\n")
    f.write("rule(role [ {admin auditor}; isArchived [ {True}; {view}; )\n\n")

    f.write("# 23. Users with relevant certifications can access technical documents\n")
    f.write("rule(certifications ] {PMP CISSP}; tags ] {technical}; {view edit}; )\n\n")

    f.write("# 24. Country-specific compliance requirements\n")
    f.write("rule(country [ {USA}; complianceRequirements ] {SOX}; {view}; )\n\n")

    f.write("# 25. Time-based access for contractors\n")
    f.write("rule(contractType [ {contractor consultant}, lastLogin <= 30; ; {view}; )\n\n")

    f.write("# 26. Experience-based access to strategic documents\n")
    f.write("rule(experience >= 10; tags ] {strategic}; {view}; )\n\n")

    f.write("# 27. Department-specific document type access\n")
    f.write("rule(department [ {techCorpEngineering}; type [ {technicalSpecification}; {view edit send}; )\n\n")

    f.write("# 28. Customer tier-based access\n")
    f.write("rule(role [ {customer}, customerTier [ {gold platinum vip}; ; {view}; )\n\n")

    f.write("# 29. Version control access\n")
    f.write("rule(role [ {employee}; ; {view}; owner = uid)\n\n")

    f.write("# 30. Cross-department collaboration\n")
    f.write("rule(role [ {employee}, currentProjects ] projectId; ; {view}; )\n\n")

    # Statistics
    f.write("\n#------------------------------------------------------------\n")
    f.write("# Statistics\n")
    f.write("#------------------------------------------------------------\n\n")
    f.write(f"# Total Users: {len(users)}\n")
    f.write(f"# Total Documents: {len(documents)}\n")
    f.write(f"# Total Organizations: {len(organizations)}\n")
    f.write(f"# Total Rules: 30\n")
    f.write(f"# Document Types: {len(documentTypes)}\n")
    f.write(f"# User Roles: {len(roles)}\n")
    f.write(f"# Security Levels: {len(securityLevels)}\n")
    f.write(f"# Regions: {len(regions)}\n")

print(f"\n✅ Enhanced ABAC policy generated and saved as '{filename}'.")
print(f"📊 Generated {len(users)} users, {len(documents)} documents, and 30 enhanced rules.")
print(f"📁 File size: {os.path.getsize(filename) / 1024 / 1024:.1f} MB")
print("\n🚀 Enhanced features include:")
print("- Geographic attributes (region, country, city)")
print("- Time-based attributes (working hours, login times)")
print("- Security clearance levels")
print("- Project management attributes")
print("- Delegation and temporary access")
print("- Document lifecycle management")
print("- Compliance requirements")
print("- Enhanced role-based permissions")
print("\n🔍 This large dataset is ideal for testing ABAC policy analysis at scale!") 