use statemachine::graph::KrustletGraph;

use petgraph::dot::Dot;

fn main() {
    let graph = KrustletGraph::generate();

    println!("{:?}", Dot::new(&graph));
}
