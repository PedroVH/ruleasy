use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum RuleError {
    Unexpected(Box<dyn Error>),
    Expected(String)
}
impl Display for RuleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_string().as_str())
    }
}
impl Error for RuleError {}