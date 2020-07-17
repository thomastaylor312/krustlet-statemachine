use statemachine::graph::KrustletGraph;
use statemachine::handlers::Pod;
use statemachine::machine::PodMachine;

use petgraph::dot::Dot;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let graph = KrustletGraph::generate();

    println!("{:?}", Dot::new(&graph));

    let (mut tx, rx) = mpsc::channel(100);
    let mut pod_machine = PodMachine::new(graph, rx);

    tx.send(Pod {
        name: "foobar".to_owned(),
        namespace: "foo".to_owned(),
        image: "mycoolimage:1.0.0".to_owned(),
        volumes: vec!["vol1".to_owned(), "vol2".to_owned()],
    })
    .await
    .expect("first pod send");
    tx.send(Pod {
        name: "blah".to_owned(),
        namespace: "foo".to_owned(),
        image: "myothercoolimage:1.0.0".to_owned(),
        volumes: vec!["vol1".to_owned()],
    })
    .await
    .expect("second pod send");
    // Do an update with only one change
    tx.send(Pod {
        name: "foobar".to_owned(),
        namespace: "foo".to_owned(),
        image: "mycoolimage:1.1.0".to_owned(),
        volumes: vec!["vol1".to_owned(), "vol2".to_owned()],
    })
    .await
    .expect("3rd pod send");

    pod_machine.run().await.expect("oh noes")
}
