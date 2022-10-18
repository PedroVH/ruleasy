use std::borrow::Borrow;
use std::collections::HashMap;
use crate::rule::Rule;

/// Defines an engine used to evaluate and execute rules.
pub struct RuleEngine {
    pub knowledge: HashMap<String, String>,
    pub rules: Vec<Rule>
}

impl RuleEngine {

    /// creates a new instance of RuleEngine
    pub fn new(knowledge: HashMap<String, String>, rules: Vec<Rule>) -> RuleEngine {
        RuleEngine {
            knowledge,
            rules
        }
    }

    /// orders rules by their priority, evaluates their conditions, and if true, executes the action.
    pub fn run(&mut self) -> Result<bool, String> {
        // sort by priority
        self.rules.sort_by(|a, b| a.priority.cmp(&b.priority));

        for rule in &self.rules {
            return match rule.borrow().evaluate(&mut self.knowledge) {
                Ok(_) => rule.borrow().execute(&mut self.knowledge),
                Err(msg) => Err(msg)
            };
        }
        Ok(true)
    }
}