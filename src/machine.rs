use std::fmt;
use std::time::Duration;

use petgraph::Graph;

use crate::handlers::StateHandler;

#[derive(Clone, Debug)]
pub enum Edge {
    Success,
    Failure(String), //TODO: anyhow::Error is not clonable
    Wait(Duration),
}

pub struct Node {
    pub name: String,
    pub handler: Box<dyn StateHandler>,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node").field("name", &self.name).finish()
    }
}

pub struct State {
    // TODO: define needed state
}

pub struct PodMachine {
    state: State, // Trying to make this non-mutable/not need a lock
    graph: Graph<Node, Edge>,
}
