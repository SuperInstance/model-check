//! Transition representation with guards and actions.

use crate::StateId;

/// A labeled transition between states.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transition {
    /// Source state.
    pub from: StateId,
    /// Target state.
    pub to: StateId,
    /// Optional guard condition (variable name that must be true).
    pub guard: Option<String>,
    /// Optional label for the transition.
    pub label: String,
}

impl Transition {
    /// Create a new transition.
    pub fn new(from: StateId, to: StateId) -> Self {
        Transition {
            from,
            to,
            guard: None,
            label: String::new(),
        }
    }

    /// Create a labeled transition.
    pub fn labeled(from: StateId, to: StateId, label: &str) -> Self {
        Transition {
            from,
            to,
            guard: None,
            label: label.to_string(),
        }
    }

    /// Create a guarded transition.
    pub fn guarded(from: StateId, to: StateId, guard: &str) -> Self {
        Transition {
            from,
            to,
            guard: Some(guard.to_string()),
            label: String::new(),
        }
    }

    /// Check if this transition fires given the current variable values.
    pub fn fires(&self, vars: &std::collections::HashMap<String, bool>) -> bool {
        match &self.guard {
            Some(g) => vars.get(g).copied().unwrap_or(false),
            None => true,
        }
    }

    /// Check if this transition is from a given state.
    pub fn is_from(&self, state: StateId) -> bool {
        self.from == state
    }

    /// Check if this transition goes to a given state.
    pub fn is_to(&self, state: StateId) -> bool {
        self.to == state
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_basic_transition() {
        let t = Transition::new(0, 1);
        assert_eq!(t.from, 0);
        assert_eq!(t.to, 1);
        assert!(t.guard.is_none());
    }

    #[test]
    fn test_labeled_transition() {
        let t = Transition::labeled(0, 1, "go");
        assert_eq!(t.label, "go");
    }

    #[test]
    fn test_guarded_transition() {
        let t = Transition::guarded(0, 1, "ready");
        assert_eq!(t.guard, Some("ready".to_string()));
    }

    #[test]
    fn test_fires_no_guard() {
        let t = Transition::new(0, 1);
        let vars = HashMap::new();
        assert!(t.fires(&vars));
    }

    #[test]
    fn test_fires_with_guard() {
        let t = Transition::guarded(0, 1, "ready");
        let mut vars = HashMap::new();
        assert!(!t.fires(&vars));
        vars.insert("ready".to_string(), true);
        assert!(t.fires(&vars));
    }

    #[test]
    fn test_is_from_to() {
        let t = Transition::new(0, 1);
        assert!(t.is_from(0));
        assert!(!t.is_from(1));
        assert!(t.is_to(1));
        assert!(!t.is_to(0));
    }
}
