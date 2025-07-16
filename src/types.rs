#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Position {
    Applicant,
    Student,
    Faculty,
    Staff,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Department {
    Cs,
    Ee,
    Registrar,
    Admissions,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Course {
    Cs101,
    Cs601,
    Cs602,
    Ee101,
    Ee601,
    Ee602,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Application,
    Gradebook,
    Roster,
    Transcript,
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComparisonOperator {
    Contains,
    ContainedIn,
    Equals,
}