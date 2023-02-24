#[derive(Debug, PartialEq)]
pub struct Validation {
    pub is_valid: bool,
    pub errors: Vec<String>,
}
