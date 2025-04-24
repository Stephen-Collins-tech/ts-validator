use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum ViolationKind {
    DirectAccess,
    Alias,
    IndirectAccess,
}

#[derive(Debug, Serialize)]
pub struct Violation {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub kind: ViolationKind,
    pub message: String,
}
