use inquire::{Select, error::InquireError};
use std::fmt::Display;

pub fn select_enum<T>(message: &str, options: Vec<T>) -> Result<T, InquireError>
where
    T: Display + Clone,
{
    Select::new(message, options).prompt().map(|v| v.clone())
}
#[derive(Debug, Clone)]
pub enum Group {
    Create,
    Join,
}
impl Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Debug, Clone)]
pub enum Register {
    Register,
    Login,
    Retry,
}
impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
