use crate::handlers::*;
use crate::machine::*;

use petgraph::Graph;

const ERROR_FILLER: &str = "doesn'tmatter";

pub struct KrustletGraph;

impl KrustletGraph {
    pub fn generate() -> Graph<Node, Edge> {
        let mut graph = Graph::new();
        let node = graph.add_node(Node {
            name: "ImagePull".to_string(),
            handler: Box::new(ImagePullHandler {}),
        });
        let success = graph.add_node(Node {
            name: "Volume".to_string(),
            handler: Box::new(VolumeHandler {}),
        });
        let err = graph.add_node(Node {
            name: "ImagePullError".to_string(),
            handler: Box::new(ImagePullHandler {}),
        });
        graph.add_edge(node, success, Edge::Success);
        graph.add_edge(node, err, Edge::Failure(ERROR_FILLER.to_owned()));
        graph.add_edge(err, err, Edge::Wait(std::time::Duration::from_secs(10)));
        graph.add_edge(err, node, Edge::Success);

        let node = success;
        let success = graph.add_node(Node {
            name: "ContainerStart".to_string(),
            handler: Box::new(ContainerStartHandler {}),
        });
        let err = graph.add_node(Node {
            name: "VolumeError".to_string(),
            handler: Box::new(VolumeErrorHandler {}),
        });
        graph.add_edge(node, success, Edge::Success);
        graph.add_edge(node, err, Edge::Failure(ERROR_FILLER.to_owned()));
        graph.add_edge(err, err, Edge::Wait(std::time::Duration::from_secs(10)));
        graph.add_edge(err, node, Edge::Success);

        let node = success;
        let success = graph.add_node(Node {
            name: "PodRunning".to_string(),
            handler: Box::new(PodRunningHandler {}),
        });
        let err = graph.add_node(Node {
            name: "ContainerError".to_string(),
            handler: Box::new(ContainerErrorHandler {}),
        });
        graph.add_edge(node, success, Edge::Success);
        graph.add_edge(node, err, Edge::Failure(ERROR_FILLER.to_owned()));
        graph.add_edge(err, err, Edge::Wait(std::time::Duration::from_secs(10)));
        graph.add_edge(err, node, Edge::Success);

        graph
    }
}
