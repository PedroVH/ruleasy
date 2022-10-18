mod rule;
mod engine;

pub use self::rule::Rule;
pub use self::engine::RuleEngine;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::engine::RuleEngine;
    use crate::rule::Rule;

    #[test]
    fn engine() {
        // defines knowledge
        let mut knowledge = HashMap::new();
        knowledge.insert("can_run".to_string(), "true".to_string());
        knowledge.insert("ran".to_string(), "false".to_string());

        // defines condition
        let condition = |knw: &mut HashMap<String, String>| {
            Ok(knw.get("can_run").unwrap() == "true")
        };
        // defines action
        let action = |knw: &mut HashMap<String, String>| {
            knw.insert("ran".to_string(), "true".to_string());
            Ok(true)
        };

        // defines the rule
        let rule = Rule {
            name: "test".to_string(),
            description: "this rule is for testing.".to_string(),
            priority: 1,
            condition,
            action
        };

        let rules: Vec<Rule> = Vec::from([rule]);
        let mut engine = RuleEngine::new(knowledge, rules);
        engine.run().unwrap();
        assert_eq!(engine.knowledge.get("ran").unwrap(), &"true".to_string())
    }

}
