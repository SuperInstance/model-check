//! Explicit-state exploration strategies.

use crate::{StateGraph, StateId};
use std::collections::{HashMap, HashSet, VecDeque};
/// State exploration strategies.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SearchStrategy {
    /// Breadth-first search.
    Bfs,
    /// Depth-first search.
    Dfs,
}

/// Result of state exploration.
#[derive(Clone, Debug)]
pub struct ExplorationResult {
    /// All reachable states.
    pub reachable: HashSet<StateId>,
    /// States in BFS/DFS order.
    pub visit_order: Vec<StateId>,
    /// BFS layers (only for BFS).
    pub layers: Vec<Vec<StateId>>,
    /// Number of transitions traversed.
    pub transitions_traversed: usize,
}

impl ExplorationResult {
    /// Whether a state is reachable.
    pub fn is_reachable(&self, id: StateId) -> bool {
        self.reachable.contains(&id)
    }

    /// Number of reachable states.
    pub fn num_reachable(&self) -> usize {
        self.reachable.len()
    }
}

/// Explores the state space of a model.
#[derive(Clone, Debug)]
pub struct StateExplorer {
    graph: StateGraph,
}

impl StateExplorer {
    /// Create a new explorer for the given state graph.
    pub fn new(graph: StateGraph) -> Self {
        StateExplorer { graph }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &StateGraph {
        &self.graph
    }

    /// Explore all reachable states from initial states using BFS.
    pub fn explore_bfs(&self) -> ExplorationResult {
        self.explore(SearchStrategy::Bfs)
    }

    /// Explore all reachable states from initial states using DFS.
    pub fn explore_dfs(&self) -> ExplorationResult {
        self.explore(SearchStrategy::Dfs)
    }

    /// Explore with a given strategy.
    pub fn explore(&self, strategy: SearchStrategy) -> ExplorationResult {
        match strategy {
            SearchStrategy::Bfs => self.bfs(),
            SearchStrategy::Dfs => self.dfs(),
        }
    }

    fn bfs(&self) -> ExplorationResult {
        let mut reachable = HashSet::new();
        let mut visit_order = Vec::new();
        let mut layers = Vec::new();
        let mut transitions_traversed = 0;

        let mut queue: VecDeque<StateId> = VecDeque::new();
        for &id in &self.graph.initial_states() {
            if !reachable.contains(&id) {
                reachable.insert(id);
                queue.push_back(id);
            }
        }

        let current_layer: Vec<StateId> = queue.iter().copied().collect();
        if !current_layer.is_empty() {
            layers.push(current_layer.clone());
        }

        while !queue.is_empty() {
            let layer_size = queue.len();
            let mut next_layer = Vec::new();

            for _ in 0..layer_size {
                let current = queue.pop_front().unwrap();
                visit_order.push(current);

                for &succ in &self.graph.successors(current) {
                    transitions_traversed += 1;
                    if !reachable.contains(&succ) {
                        reachable.insert(succ);
                        queue.push_back(succ);
                        next_layer.push(succ);
                    }
                }
            }

            if !next_layer.is_empty() {
                layers.push(next_layer);
            }
        }

        ExplorationResult {
            reachable,
            visit_order,
            layers,
            transitions_traversed,
        }
    }

    fn dfs(&self) -> ExplorationResult {
        let mut reachable = HashSet::new();
        let mut visit_order = Vec::new();
        let mut transitions_traversed = 0;

        // Use explicit stack for iterative DFS
        let mut stack: Vec<StateId> = self.graph.initial_states();
        stack.reverse(); // Process in order

        while let Some(current) = stack.pop() {
            if reachable.contains(&current) {
                continue;
            }
            reachable.insert(current);
            visit_order.push(current);

            let mut succs = self.graph.successors(current);
            succs.reverse();
            for succ in succs {
                transitions_traversed += 1;
                if !reachable.contains(&succ) {
                    stack.push(succ);
                }
            }
        }

        ExplorationResult {
            reachable,
            visit_order,
            layers: Vec::new(),
            transitions_traversed,
        }
    }

    /// Find a path from any initial state to a target state (BFS shortest path).
    pub fn find_path_to(&self, target: StateId) -> Option<Vec<StateId>> {
        let mut visited = HashSet::new();
        let mut parent: std::collections::HashMap<StateId, StateId> = HashMap::new();
        let mut queue: VecDeque<StateId> = VecDeque::new();

        for &id in &self.graph.initial_states() {
            visited.insert(id);
            queue.push_back(id);
            if id == target {
                return Some(vec![id]);
            }
        }

        while let Some(current) = queue.pop_front() {
            for &succ in &self.graph.successors(current) {
                if !visited.contains(&succ) {
                    visited.insert(succ);
                    parent.insert(succ, current);
                    if succ == target {
                        // Reconstruct path
                        let mut path = vec![target];
                        let mut node = target;
                        while let Some(&p) = parent.get(&node) {
                            path.push(p);
                            node = p;
                        }
                        path.reverse();
                        return Some(path);
                    }
                    queue.push_back(succ);
                }
            }
        }

        None
    }

    /// Find all states that satisfy a predicate.
    pub fn find_states<F>(&self, predicate: F) -> Vec<StateId>
    where
        F: Fn(&std::collections::HashMap<String, bool>) -> bool,
    {
        self.graph
            .states()
            .iter()
            .filter(|s| predicate(&s.vars))
            .map(|s| s.id)
            .collect()
    }

    /// Compute the set of dead states (states with no outgoing transitions).
    pub fn dead_states(&self) -> Vec<StateId> {
        self.graph
            .state_ids()
            .into_iter()
            .filter(|&id| self.graph.successors(id).is_empty())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_linear_graph() -> StateGraph {
        let mut g = StateGraph::new();
        let s0 = g.add_state("s0");
        let s1 = g.add_state("s1");
        let s2 = g.add_state("s2");
        g.mark_initial(s0);
        g.add_transition(s0, s1);
        g.add_transition(s1, s2);
        g
    }

    #[test]
    fn test_bfs_exploration() {
        let graph = make_linear_graph();
        let explorer = StateExplorer::new(graph);
        let result = explorer.explore_bfs();
        assert_eq!(result.num_reachable(), 3);
        assert_eq!(result.visit_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_dfs_exploration() {
        let graph = make_linear_graph();
        let explorer = StateExplorer::new(graph);
        let result = explorer.explore_dfs();
        assert_eq!(result.num_reachable(), 3);
        assert!(result.is_reachable(2));
    }

    #[test]
    fn test_bfs_layers() {
        let graph = make_linear_graph();
        let explorer = StateExplorer::new(graph);
        let result = explorer.explore_bfs();
        assert_eq!(result.layers.len(), 3);
        assert_eq!(result.layers[0], vec![0]);
        assert_eq!(result.layers[1], vec![1]);
        assert_eq!(result.layers[2], vec![2]);
    }

    #[test]
    fn test_find_path() {
        let graph = make_linear_graph();
        let explorer = StateExplorer::new(graph);
        let path = explorer.find_path_to(2).unwrap();
        assert_eq!(path, vec![0, 1, 2]);
    }

    #[test]
    fn test_find_path_unreachable() {
        let mut g = StateGraph::new();
        let s0 = g.add_state("s0");
        let s1 = g.add_state("s1");
        g.mark_initial(s0);
        // No transition from s0 to s1
        let explorer = StateExplorer::new(g);
        assert!(explorer.find_path_to(s1).is_none());
    }

    #[test]
    fn test_diamond_graph_bfs() {
        let mut g = StateGraph::new();
        let s0 = g.add_state("s0");
        let s1 = g.add_state("s1");
        let s2 = g.add_state("s2");
        let s3 = g.add_state("s3");
        g.mark_initial(s0);
        g.add_transition(s0, s1);
        g.add_transition(s0, s2);
        g.add_transition(s1, s3);
        g.add_transition(s2, s3);

        let explorer = StateExplorer::new(g);
        let result = explorer.explore_bfs();
        assert_eq!(result.num_reachable(), 4);
        assert_eq!(result.layers.len(), 3);
    }

    #[test]
    fn test_cycle_graph() {
        let mut g = StateGraph::new();
        let s0 = g.add_state("s0");
        let s1 = g.add_state("s1");
        g.mark_initial(s0);
        g.add_transition(s0, s1);
        g.add_transition(s1, s0); // cycle

        let explorer = StateExplorer::new(g);
        let result = explorer.explore_bfs();
        assert_eq!(result.num_reachable(), 2);
    }

    #[test]
    fn test_dead_states() {
        let graph = make_linear_graph();
        let explorer = StateExplorer::new(graph);
        let dead = explorer.dead_states();
        assert_eq!(dead, vec![2]); // s2 has no successors
    }

    #[test]
    fn test_find_states_by_predicate() {
        let mut g = StateGraph::new();
        let s0 = g.add_state_with_vars("s0", vec![("ready", true)]);
        let _s1 = g.add_state_with_vars("s1", vec![("ready", false)]);

        let explorer = StateExplorer::new(g);
        let ready_states = explorer.find_states(|vars| vars.get("ready").copied() == Some(true));
        assert_eq!(ready_states, vec![s0]);
    }
}
