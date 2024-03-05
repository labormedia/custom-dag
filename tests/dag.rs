use custom_dag::{Node, Dag};
// randomness is added to the integration tests to assert the validity of nodes that could contain any value within the range of the id type.
// This means that the tests using the random generator are not deterministic. This is why we include them as integration tests.
// It is necessary to ackownledge, then, that a complete formal verification should include all the values of the id type set.
// In favor of performance, probabilistic testing for large number of nodes is prioritized first.
use rand::Rng;

#[test]
fn create_1_000_000_random_nodes_unconnected_DAG() {
    let mut dag = Dag::new();
    for i in 0..1_000_000 {
       dag.insert(Node::new(i, None, None));
    }
    for i in 0..1_000_000 {
        assert!(dag.contains_id(&i));
    }
}

#[test]
fn insert_existing_node_id_does_not_update() {
    let id = 0;
    let nodeA = Node::new(id,Some(3),Some(5));
    let nodeB = Node::new(id,Some(42),Some(42));
    let mut dag = Dag::new();
    assert_eq!(nodeA.id, nodeB.id);
    assert_eq!(dag.insert(nodeA), None);
    let insert_result = dag.insert(nodeB).unwrap();
    assert_eq!(insert_result, &nodeA);
    assert_eq!(insert_result.left, Some(3));
    assert_eq!(insert_result.right, Some(5));
    let node_in_dag = dag.get(&id);
    assert_eq!(
        node_in_dag, 
        Some(&Node {
            id,
            left: nodeA.left,
            right: nodeA.right,
        })
    );
    assert_ne!(
        node_in_dag.expect("Wrong value assumption.").left, 
        nodeB.left,
    );
    assert_ne!(
        node_in_dag.expect("Wrong value assumption.").right, 
        nodeB.right,
    )
}