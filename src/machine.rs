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

pub struct Node<T> {
    pub name: String,
    pub handler: Box<dyn StateHandler<T>>,
}

impl<T> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node").field("name", &self.name).finish()
    }
}

pub struct PodMachine<T> {
    graph: Graph<Node<T>, Edge>,
}

// TODO: Construct PodMachine and add start method (with listening loop)
