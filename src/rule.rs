use std::collections::{HashMap};

pub struct Rule {
    pub name: String,
    pub description: String,
    pub priority: i32,
    pub condition: fn(&mut HashMap<String, String>) -> Result<bool, String>,
    pub action: fn(&mut HashMap<String, String>) -> Result<bool, String>
}

impl Rule {
    // executes the condition and returns the result
    pub fn evaluate(&self, knowledge: &mut HashMap<String, String>) -> Result<bool, String> {
        (self.condition)(knowledge)
    }

    // executes the action and returns the result
    pub fn execute(&self, knowledge: &mut HashMap<String, String>) -> Result<bool, String> {
        (self.action)(knowledge)
    }
}
