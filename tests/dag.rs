use std::collections::{
    HashSet,
    HashMap,
};
use custom_dag::{
    Node, 
    Dag,
    collitions::CollidingNode,
};
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
    };
    assert!(dag.is_safe())
}

#[test]
fn insert_existing_node_id_does_not_update() {
    type TestType = u32;
    let id: TestType = 0;
    let nodeA = Node::new(id,None,None);
    let nodeB = Node::new(id,Some(42),Some(43));
    let mut dag = Dag::new();
    // Nodes are considered equal by id under the Node<T> type, but they can contain different field values (other than id).
    assert_eq!(nodeA, nodeB);
    assert_ne!(nodeA.left, nodeB.left);
    // The new node was not present in the dag, so it gets inserted in the DAG.
    assert_eq!(dag.insert(nodeA), None);
    assert!(dag.get(&nodeA.id).expect("Wrond type assumption.").has_same_fields_to(&nodeA));
    // When trying to insert a new Node<T> with the same id, it will return the present node and will deflect the insertion to the collition collection.
    // The returned value of this action is the node already prensent, with the same id.
    let insert_result = dag.insert(nodeB).expect("Wrond value assumption.");
    assert_eq!(insert_result, nodeA);
    assert_eq!(insert_result.left, None);
    assert_eq!(insert_result.right, None);
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
    );
    // Examine the DAG's collition collection.
    let collitions: &HashSet<CollidingNode<TestType>> = dag.get_collitions(&id).expect("Wrong value assumption.");
    let colliding_node = collitions.get(&CollidingNode::from(nodeB)).expect("Wrong value assumption.");
    // The colliding node corresponds to nodeB.
    assert_eq!(colliding_node, &CollidingNode::from(nodeB));
    // The collitions set contains a CollidingNode that corresponds to nodeB.
    assert!(collitions.contains(&nodeB.into()));
    // The collitions set does not contains nodeA, even if it has the same id, neither a CollitionNode that corresponds to nodeA.
    assert!(!collitions.contains(&nodeA.into()));
    assert!(!collitions.contains(&CollidingNode::from(nodeA)));
    // The fields in the colliding node corresponds to nodeB, but not to nodeA.
    assert!(colliding_node.has_same_fields_to(&nodeB.into()));
    assert!(!colliding_node.has_same_fields_to(&nodeA.into()))
}

#[test]
fn insert_existing_node_marks_dag_unsafe() {
    type TestType = u32;
    let id: TestType = 0;
    let nodeA = Node::new(id,None,None);
    let nodeB = Node::new(id,Some(42),Some(43));
    let mut dag = Dag::new();
    // Inserts nodeA into the DAG.
    assert_eq!(dag.insert(nodeA), None);
    assert!(dag.get(&nodeA.id).expect("Wrond value assumption.").has_same_fields_to(&nodeA));
    // Dag is still marked as safe (no cycles) after first insertion.
    assert!(dag.is_safe());
    // Trying to insert a node with the id of an already inserted node collects the collition and returns the value of the previously inserted node, which will persist in the DAG.
    assert!(dag.insert(nodeB).expect("Wrong value assumption.").has_same_fields_to(&nodeA));
    // Examine the DAG's collition collection.
    let collitions: &HashSet<CollidingNode<TestType>> = dag.get_collitions(&id).expect("Wrong value assumption.");
    let colliding_node = collitions.get(&CollidingNode::from(nodeB)).expect("Wrong value assumption.");
    assert_eq!(colliding_node, &CollidingNode::from(nodeB));assert_eq!(colliding_node, &CollidingNode::from(nodeB));

    // Node in dag with id 0 is still nodeA, but not nodeB.
    let node_in_dag = dag.get(&nodeA.id).expect("Wrond value assumption.");
    assert!(node_in_dag.has_same_fields_to(&nodeA));
    assert!(!node_in_dag.has_same_fields_to(&nodeB));

    // insert_or_update updates the dag values for the node id 0 and returns the previous values, nodeA. 
    // The final fields values for the node id 0 in the dag are equal to nodeB but not nodeA.
    assert!(dag.insert_or_update(nodeB).expect("Wrong value assumption.").has_same_fields_to(&nodeA));
    assert!(!dag.get(&nodeA.id).expect("Wrong value assumption.").has_same_fields_to(&nodeA));
    assert!(dag.get(&nodeA.id).expect("Wrong value assumption.").has_same_fields_to(&nodeB));

}

#[test]
fn insert_a_node_with_non_existent_left_reference_marks_dag_unsafe() {
    type TestType = u32;
    let id: TestType = 0;
    let nodeA = Node::new(id,Some(3),None);
    let mut dag = Dag::new();
    assert_eq!(dag.insert(nodeA), Some(nodeA));
    assert_eq!(dag.is_safe(), false);
}

#[test]
fn insert_a_node_with_non_existent_right_reference_marks_dag_unsafe() {
    type TestType = u32;
    let id: TestType = 0;
    let nodeA = Node::new(id,None,Some(5));
    let mut dag = Dag::new();
    assert_eq!(dag.insert(nodeA), Some(nodeA));
    assert_eq!(dag.is_safe(), false);
}

#[test]
fn insert_non_existent_nodes_with_existent_references_is_safe() {
    let nodeA = Node::new(0,None,None);
    let nodeB = Node::new(1,Some(0),None);
    let nodeC = Node::new(2,None,Some(0));
    let nodeD = Node::new(3,Some(0), Some(1));
    let nodeE = Node::new(4,Some(2), Some(1));
    let nodeF = Node::new(5,Some(3), Some(4));
    let mut dag = Dag::new();
    assert_eq!(dag.insert(nodeA), None);
    assert_eq!(dag.insert(nodeB), None);
    assert_eq!(dag.insert(nodeC), None);
    assert_eq!(dag.insert(nodeD), None);
    assert_eq!(dag.insert(nodeE), None);
    assert_eq!(dag.insert(nodeF), None);
    assert!(dag.is_safe())
}