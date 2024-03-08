use custom_dag::{
    Node,
    Dag,
};
// randomness is added to the integration tests to assert the validity of nodes that could contain any value within the range of the id type.
// This means that the tests using the random generator are not deterministic.
// A complete formal verification should include all the values of the id type set.
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut dag = Dag::new();
    for _ in 1..100 {
        let node_id: u32 = rng.gen();
        let new_node = Node::new(node_id, None, None);
        println!("Created new node : {:?}", new_node);
        dag.insert(new_node);
        println!("Inserted new node {:?} to the DAG.", dag.get(&node_id).expect("Wrong value assumption."));
    };
}