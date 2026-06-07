//! # model-check
//!
//! A model checking library with explicit-state exploration and property verification.
//!
//! ## Features
//!
//! - Explicit-state exploration with BFS and DFS
//! - Reachability analysis
//! - Safety property verification (invariants)
//! - Liveness property verification (eventually properties)
//! - Counterexample generation for violated properties
//!
//! ## Example
//!
//! ```
//! use model_check::{StateGraph, ModelChecker};
//!
//! let mut graph = StateGraph::new();
//! let s0 = graph.add_state("idle");
//! let s1 = graph.add_state("running");
//! graph.mark_initial(s0);
//! graph.add_transition(s0, s1);
//!
//! let checker = ModelChecker::new(graph);
//! let result = checker.check_reachability(s1);
//! assert!(result.is_satisfied());
//! ```

mod state;
mod transition;
mod property;
mod explorer;
mod checker;

pub use state::{State, StateId, StateGraph};
pub use transition::Transition;
pub use property::{Property, PropertyResult, SafetyProp, LivenessProp};
pub use explorer::StateExplorer;
pub use checker::ModelChecker;
