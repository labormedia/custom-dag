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
}