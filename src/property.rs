//! Property specifications for model checking.

use crate::StateId;
use std::fmt;

/// Result of checking a property.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PropertyResult {
    /// Property is satisfied in all reachable states.
    Satisfied,
    /// Property is violated. Contains a counterexample trace.
    Violated(Vec<StateId>),
}

impl PropertyResult {
    /// Whether the property is satisfied.
    pub fn is_satisfied(&self) -> bool {
        matches!(self, PropertyResult::Satisfied)
    }

    /// Whether the property is violated.
    pub fn is_violated(&self) -> bool {
        matches!(self, PropertyResult::Violated(_))
    }

    /// Get the counterexample trace, if any.
    pub fn counterexample(&self) -> Option<&[StateId]> {
        match self {
            PropertyResult::Violated(trace) => Some(trace),
            PropertyResult::Satisfied => None,
        }
    }
}

/// A property to check on the model.
#[derive(Clone, Debug)]
pub enum Property {
    /// Invariant: must hold in every reachable state.
    /// The closure takes a variable map and returns true if the property holds.
    Invariant {
        name: String,
        condition: String,
    },
    /// Reachability: can we reach a state satisfying the condition?
    Reachability {
        name: String,
        target: String,
    },
    /// Liveness: eventually a condition must hold on every path.
    Eventually {
        name: String,
        condition: String,
    },
    /// Custom property checked via a predicate on state variable maps.
    Custom {
        name: String,
        description: String,
    },
}

impl Property {
    /// Create a safety (invariant) property.
    pub fn safety(name: &str, condition: &str) -> Self {
        Property::Invariant {
            name: name.to_string(),
            condition: condition.to_string(),
        }
    }

    /// Create a reachability property.
    pub fn reachability(name: &str, target: &str) -> Self {
        Property::Reachability {
            name: name.to_string(),
            target: target.to_string(),
        }
    }

    /// Create a liveness property.
    pub fn liveness(name: &str, condition: &str) -> Self {
        Property::Eventually {
            name: name.to_string(),
            condition: condition.to_string(),
        }
    }

    /// Get the property name.
    pub fn name(&self) -> &str {
        match self {
            Property::Invariant { name, .. } => name,
            Property::Reachability { name, .. } => name,
            Property::Eventually { name, .. } => name,
            Property::Custom { name, .. } => name,
        }
    }
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Property::Invariant { name, condition } => write!(f, "AG({}) [{}]", condition, name),
            Property::Reachability { name, target } => write!(f, "EF({}) [{}]", target, name),
            Property::Eventually { name, condition } => write!(f, "AF({}) [{}]", condition, name),
            Property::Custom { name, description } => write!(f, "{} [{}]", description, name),
        }
    }
}

/// Safety property checker: a predicate over state variables.
pub type SafetyProp = fn(&std::collections::HashMap<String, bool>) -> bool;

/// Liveness property checker: checks that a condition eventually holds on a trace.
pub type LivenessProp = fn(&[&std::collections::HashMap<String, bool>]) -> bool;

/// Evaluates a simple condition string against state variables.
/// Supports: single variable names, "var1 AND var2", "NOT var".
pub fn eval_condition(
    condition: &str,
    vars: &std::collections::HashMap<String, bool>,
) -> bool {
    let cond = condition.trim();

    if let Some(rest) = cond.strip_prefix("NOT ") {
        return !eval_condition(rest.trim(), vars);
    }

    if let Some((left, right)) = cond.split_once(" AND ") {
        return eval_condition(left.trim(), vars) && eval_condition(right.trim(), vars);
    }

    if let Some((left, right)) = cond.split_once(" OR ") {
        return eval_condition(left.trim(), vars) || eval_condition(right.trim(), vars);
    }

    // Single variable
    vars.get(cond).copied().unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_property_result_satisfied() {
        let r = PropertyResult::Satisfied;
        assert!(r.is_satisfied());
        assert!(!r.is_violated());
    }

    #[test]
    fn test_property_result_violated() {
        let r = PropertyResult::Violated(vec![0, 1, 2]);
        assert!(r.is_violated());
        assert_eq!(r.counterexample(), Some(&[0u32, 1, 2][..]));
    }

    #[test]
    fn test_safety_property() {
        let p = Property::safety("mutual_excl", "NOT critical");
        assert_eq!(p.name(), "mutual_excl");
    }

    #[test]
    fn test_reachability_property() {
        let p = Property::reachability("can_finish", "done");
        assert_eq!(p.name(), "can_finish");
    }

    #[test]
    fn test_liveness_property() {
        let p = Property::liveness("progress", "done");
        assert_eq!(p.name(), "progress");
    }

    #[test]
    fn test_eval_single_var() {
        let mut vars = HashMap::new();
        vars.insert("ready".to_string(), true);
        assert!(eval_condition("ready", &vars));
        assert!(!eval_condition("go", &vars));
    }

    #[test]
    fn test_eval_not() {
        let mut vars = HashMap::new();
        vars.insert("error".to_string(), false);
        assert!(eval_condition("NOT error", &vars));
    }

    #[test]
    fn test_eval_and() {
        let mut vars = HashMap::new();
        vars.insert("a".to_string(), true);
        vars.insert("b".to_string(), true);
        assert!(eval_condition("a AND b", &vars));
        vars.insert("b".to_string(), false);
        assert!(!eval_condition("a AND b", &vars));
    }

    #[test]
    fn test_eval_or() {
        let mut vars = HashMap::new();
        vars.insert("a".to_string(), false);
        vars.insert("b".to_string(), true);
        assert!(eval_condition("a OR b", &vars));
    }

    #[test]
    fn test_display_property() {
        let p = Property::safety("test", "ready");
        assert!(format!("{}", p).contains("AG(ready)"));
    }
}
