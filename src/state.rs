//! State and state graph representation.

use std::collections::{HashMap, HashSet};
use std::fmt;

/// Unique identifier for a state.
pub type StateId = u32;

/// A state in the model.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct State {
    /// Unique identifier.
    pub id: StateId,
    /// Human-readable label.
    pub label: String,
    /// Boolean-valued state variables (atomic propositions).
    pub vars: HashMap<String, bool>,
    /// Whether this is an initial state.
    pub initial: bool,
}

impl State {
    /// Create a new state with the given label.
    pub fn new(id: StateId, label: &str) -> Self {
        State {
            id,
            label: label.to_string(),
            vars: HashMap::new(),
            initial: false,
        }
    }

    /// Set a state variable.
    pub fn set_var(&mut self, name: &str, val: bool) -> &mut Self {
        self.vars.insert(name.to_string(), val);
        self
    }

    /// Get a state variable.
    pub fn get_var(&self, name: &str) -> Option<bool> {
        self.vars.get(name).copied()
    }

    /// Check if all given variables are true.
    pub fn satisfies_all(&self, names: &[&str]) -> bool {
        names.iter().all(|&n| self.get_var(n) == Some(true))
    }

    /// Mark this state as initial.
    pub fn mark_initial(&mut self) -> &mut Self {
        self.initial = true;
        self
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{{{}}}", self.label,
            self.vars.iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<_>>().join(", "))
    }
}

/// A directed graph of states with transitions.
#[derive(Clone, Debug)]
pub struct StateGraph {
    /// States indexed by ID.
    states: HashMap<StateId, State>,
    /// Adjacency list: source -> set of targets.
    transitions: HashMap<StateId, HashSet<StateId>>,
    /// Reverse adjacency: target -> set of sources.
    rev_transitions: HashMap<StateId, HashSet<StateId>>,
    /// Initial states.
    initial_states: HashSet<StateId>,
    /// Next available state ID.
    next_id: StateId,
}

impl StateGraph {
    /// Create a new empty state graph.
    pub fn new() -> Self {
        StateGraph {
            states: HashMap::new(),
            transitions: HashMap::new(),
            rev_transitions: HashMap::new(),
            initial_states: HashSet::new(),
            next_id: 0,
        }
    }

    /// Add a new state with the given label, returning its ID.
    pub fn add_state(&mut self, label: &str) -> StateId {
        let id = self.next_id;
        self.next_id += 1;
        let state = State::new(id, label);
        self.states.insert(id, state);
        id
    }

    /// Add a state with specific variables.
    pub fn add_state_with_vars(&mut self, label: &str, vars: Vec<(&str, bool)>) -> StateId {
        let id = self.add_state(label);
        for (name, val) in vars {
            self.states.get_mut(&id).unwrap().set_var(name, val);
        }
        id
    }

    /// Mark a state as initial.
    pub fn mark_initial(&mut self, id: StateId) {
        if let Some(state) = self.states.get_mut(&id) {
            state.initial = true;
        }
        self.initial_states.insert(id);
    }

    /// Add a transition from `from` to `to`.
    pub fn add_transition(&mut self, from: StateId, to: StateId) {
        self.transitions.entry(from).or_default().insert(to);
        self.rev_transitions.entry(to).or_default().insert(from);
    }

    /// Get a state by ID.
    pub fn get_state(&self, id: StateId) -> Option<&State> {
        self.states.get(&id)
    }

    /// Get all states.
    pub fn states(&self) -> Vec<&State> {
        self.states.values().collect()
    }

    /// Get successors of a state.
    pub fn successors(&self, id: StateId) -> Vec<StateId> {
        self.transitions.get(&id).map(|s| s.iter().copied().collect()).unwrap_or_default()
    }

    /// Get predecessors of a state.
    pub fn predecessors(&self, id: StateId) -> Vec<StateId> {
        self.rev_transitions.get(&id).map(|s| s.iter().copied().collect()).unwrap_or_default()
    }

    /// Get initial states.
    pub fn initial_states(&self) -> Vec<StateId> {
        self.initial_states.iter().copied().collect()
    }

    /// Number of states.
    pub fn num_states(&self) -> usize {
        self.states.len()
    }

    /// Number of transitions.
    pub fn num_transitions(&self) -> usize {
        self.transitions.values().map(|s| s.len()).sum()
    }

    /// Check if a transition exists.
    pub fn has_transition(&self, from: StateId, to: StateId) -> bool {
        self.transitions.get(&from).map(|s| s.contains(&to)).unwrap_or(false)
    }

    /// Check if a state exists.
    pub fn has_state(&self, id: StateId) -> bool {
        self.states.contains_key(&id)
    }

    /// Get all state IDs.
    pub fn state_ids(&self) -> Vec<StateId> {
        self.states.keys().copied().collect()
    }
}

impl Default for StateGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_state() {
        let mut s = State::new(0, "init");
        s.set_var("ready", true);
        assert_eq!(s.get_var("ready"), Some(true));
        assert_eq!(s.get_var("missing"), None);
    }

    #[test]
    fn test_state_satisfies() {
        let mut s = State::new(0, "s");
        s.set_var("a", true);
        s.set_var("b", true);
        assert!(s.satisfies_all(&["a", "b"]));
        assert!(!s.satisfies_all(&["a", "c"]));
    }

    #[test]
    fn test_state_graph_add() {
        let mut g = StateGraph::new();
        let s0 = g.add_state("s0");
        let s1 = g.add_state("s1");
        assert_eq!(g.num_states(), 2);
        assert!(g.has_state(s0));
    }

    #[test]
    fn test_transitions() {
        let mut g = StateGraph::new();
        let s0 = g.add_state("s0");
        let s1 = g.add_state("s1");
        g.add_transition(s0, s1);
        assert!(g.has_transition(s0, s1));
        assert!(!g.has_transition(s1, s0));
        assert_eq!(g.num_transitions(), 1);
    }

    #[test]
    fn test_successors_predecessors() {
        let mut g = StateGraph::new();
        let s0 = g.add_state("s0");
        let s1 = g.add_state("s1");
        let s2 = g.add_state("s2");
        g.add_transition(s0, s1);
        g.add_transition(s0, s2);
        let mut succs = g.successors(s0);
        succs.sort();
        assert_eq!(succs, vec![s1, s2]);
        assert_eq!(g.predecessors(s1), vec![s0]);
    }

    #[test]
    fn test_initial_states() {
        let mut g = StateGraph::new();
        let s0 = g.add_state("s0");
        let s1 = g.add_state("s1");
        g.mark_initial(s0);
        assert_eq!(g.initial_states(), vec![s0]);
    }

    #[test]
    fn test_display_state() {
        let mut s = State::new(0, "init");
        s.set_var("x", true);
        let display = format!("{}", s);
        assert!(display.contains("init"));
        assert!(display.contains("x=true"));
    }
}
