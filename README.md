# model-check

A model checking library with explicit-state exploration and property verification.

## Features

- **Explicit-state exploration** with BFS and DFS strategies
- **Reachability analysis**: find paths and reachable state sets
- **Safety properties**: invariant checking with counterexample generation
- **Liveness properties**: eventually property verification with cycle detection
- **Layer computation**: BFS distance layers from initial states
- **Dead state detection**: find states with no outgoing transitions

## Installation

```toml
[dependencies]
model-check = "0.1.0"
```

## Usage

```rust
use model_check::{StateGraph, Property, ModelChecker};

let mut graph = StateGraph::new();
let s0 = graph.add_state_with_vars("idle", vec![("running", false), ("done", false)]);
let s1 = graph.add_state_with_vars("running", vec![("running", true), ("done", false)]);
let s2 = graph.add_state_with_vars("done", vec![("running", false), ("done", true)]);
graph.mark_initial(s0);
graph.add_transition(s0, s1);
graph.add_transition(s1, s2);

let checker = ModelChecker::new(graph);
assert!(checker.check_reachability(s2).is_satisfied());
assert!(checker.check_invariant("done").is_violated()); // fails in idle and running states
```

## Architecture

| Module | Description |
|--------|-------------|
| `state` | State representation and state graph |
| `transition` | Labeled transitions with guards |
| `property` | Property specifications (safety, liveness, reachability) |
| `explorer` | BFS/DFS state space exploration |
| `checker` | Model checker combining exploration with verification |

## License

MIT OR Apache-2.0
