use std::sync::Arc;

use petgraph::Graph;

use crate::handlers::*;
use crate::machine::*;

const ERROR_FILLER: &str = "doesn'tmatter";

pub struct KrustletGraph;

impl KrustletGraph {
    pub fn generate() -> Graph<Node<Pod>, Edge> {
        let state = Arc::new(dashmap::DashMap::new());
        let mut graph = Graph::new();
        let node = graph.add_node(Node {
            name: "ImagePull".to_string(),
            handler: Box::new(ImagePullHandler {
                state: state.clone(),
            }),
        });
        let success = graph.add_node(Node {
            name: "Volume".to_string(),
            handler: Box::new(VolumeHandler {
                state: state.clone(),
            }),
        });
        let err = graph.add_node(Node {
            name: "ImagePullError".to_string(),
            handler: Box::new(ImagePullHandler {
                state: state.clone(),
            }),
        });
        graph.add_edge(node, success, Edge::Success);
        graph.add_edge(node, err, Edge::Failure(ERROR_FILLER.to_owned()));
        graph.add_edge(err, err, Edge::Wait(std::time::Duration::from_secs(10)));
        graph.add_edge(err, node, Edge::Success);

        let node = success;
        let success = graph.add_node(Node {
            name: "ContainerStart".to_string(),
            handler: Box::new(ContainerStartHandler {
                state: state.clone(),
            }),
        });
        let err = graph.add_node(Node {
            name: "VolumeError".to_string(),
            handler: Box::new(VolumeErrorHandler {
                state: state.clone(),
            }),
        });
        graph.add_edge(node, success, Edge::Success);
        graph.add_edge(node, err, Edge::Failure(ERROR_FILLER.to_owned()));
        graph.add_edge(err, err, Edge::Wait(std::time::Duration::from_secs(10)));
        graph.add_edge(err, node, Edge::Success);

        let node = success;
        let success = graph.add_node(Node {
            name: "PodRunning".to_string(),
            handler: Box::new(PodRunningHandler {
                state: state.clone(),
            }),
        });
        let err = graph.add_node(Node {
            name: "ContainerError".to_string(),
            handler: Box::new(ContainerErrorHandler { state }),
        });
        graph.add_edge(node, success, Edge::Success);
        graph.add_edge(node, err, Edge::Failure(ERROR_FILLER.to_owned()));
        graph.add_edge(err, err, Edge::Wait(std::time::Duration::from_secs(10)));
        graph.add_edge(err, node, Edge::Success);

        graph
    }
}
