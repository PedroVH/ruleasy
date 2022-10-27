use std::collections::{HashMap};
use crate::error::RuleError;

pub struct Rule {
    pub name: String,
    pub description: String,
    pub priority: i32,
    pub condition: fn(&mut HashMap<String, String>) -> Result<bool, RuleError>,
    pub action: fn(&mut HashMap<String, String>) -> Result<bool, RuleError>
}

impl Rule {
    // executes the condition and returns the result
    pub fn evaluate(&self, knowledge: &mut HashMap<String, String>) -> Result<bool, RuleError> {
        (self.condition)(knowledge)
    }

    // executes the action and returns the result
    pub fn execute(&self, knowledge: &mut HashMap<String, String>) -> Result<bool, RuleError> {
        (self.action)(knowledge)
    }
}
