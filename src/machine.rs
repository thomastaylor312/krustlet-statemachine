use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use petgraph::{graph::NodeIndex, visit::EdgeRef, Direction, Graph};
use tokio::sync::mpsc::Receiver;

use crate::handlers::StateHandler;

#[derive(Clone, Debug)]
pub enum Edge {
    Success,
    Failure(String), //TODO: anyhow::Error is not clonable
    Wait(Duration),
}

pub struct Node<T> {
    pub name: String,
    pub handler: Box<dyn StateHandler<T> + Send + Sync>,
}

impl<T> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node").field("name", &self.name).finish()
    }
}

pub struct PodMachine<T> {
    graph: Arc<Graph<Node<T>, Edge>>,
    pod_receiver: Receiver<T>,
}

impl<T: Send + Sync + 'static> PodMachine<T> {
    pub fn new(graph: Graph<Node<T>, Edge>, pod_receiver: Receiver<T>) -> Self {
        // TODO: Validate that the graph only has one start node (and grab the start index?)
        PodMachine {
            graph: Arc::new(graph),
            pod_receiver,
        }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        while let Some(item) = self.pod_receiver.recv().await {
            tokio::task::spawn(walk_graph(self.graph.clone(), item));
        }
        Ok(())
    }
}

async fn walk_graph<T: Send + Sync + 'static>(graph: Arc<Graph<Node<T>, Edge>>, item: T) {
    let mut current_index = NodeIndex::new(0);
    let mut current_node = &graph[current_index];
    // Get the sinks for checking if we are at the end of the graph
    let sinks: Vec<NodeIndex> = graph.externals(Direction::Outgoing).collect();

    // TODO: Maybe design a custom iterator that checks for the edge type and then points at the next one
    loop {
        // TODO: Add handling for Wait edge
        let res = current_node.handler.handle(&item);
        // Circuit breaker, checks if we've reached a sink
        if sinks.iter().any(|idx| idx == &current_index) {
            break;
        }
        // TODO: Handle case where returned edge doesn't exist
        current_index = graph
            .edges(current_index)
            .find(|er| std::mem::discriminant(er.weight()) == std::mem::discriminant(&res))
            .unwrap()
            .target();
        current_node = &graph[current_index]
    }
    println!("completed graph walk")
}
