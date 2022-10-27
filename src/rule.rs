use std::collections::{HashMap};
use crate::error::RuleError;

pub struct Rule<'a> {
    pub name: String,
    pub description: String,
    pub priority: i32,
    pub condition: &'a dyn Fn(&mut HashMap<String, String>) -> Result<bool, RuleError>,
    pub action: &'a dyn Fn(&mut HashMap<String, String>) -> Result<bool, RuleError>
}

impl <'a> Rule<'a> {
    // executes the condition and returns the result
    pub fn evaluate(&self, knowledge: &mut HashMap<String, String>) -> Result<bool, RuleError> {
        (self.condition)(knowledge)
    }

    // executes the action and returns the result
    pub fn execute(&self, knowledge: &mut HashMap<String, String>) -> Result<bool, RuleError> {
        (self.action)(knowledge)
    }
}
