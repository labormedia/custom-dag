use std::collections::{
    HashSet,
    // HashMap,
};
use custom_dag::{
    Node, 
    Dag,
    collitions::CollidingNode,
};

#[test]
fn create_1_000_000_random_nodes_unconnected_dag() {
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
    let node_a = Node::new(id,None,None);
    let node_b = Node::new(id,Some(42),Some(43));
    let mut dag = Dag::new();
    // Nodes are considered equal by id under the Node<T> type, but they can contain different field values (other than id).
    assert_eq!(node_a, node_b);
    assert_ne!(node_a.left, node_b.left);
    // The new node was not present in the dag, so it gets inserted in the DAG.
    assert_eq!(dag.insert(node_a), None);
    assert!(dag.get(&node_a.id).expect("Invalid type assumption.").has_same_fields_to(&node_a));
    // When trying to insert a new Node<T> with the same id, it will return the present node and will deflect the insertion to the collition collection.
    // The returned value of this action is the node already prensent, with the same id.
    let insert_result = dag.insert(node_b).expect("Invalid value assumption.");
    assert_eq!(insert_result, node_a);
    assert_eq!(insert_result.left, None);
    assert_eq!(insert_result.right, None);
    let node_in_dag = dag.get(&id);
    assert_eq!(
        node_in_dag, 
        Some(&Node {
            id,
            left: node_a.left,
            right: node_a.right,
        })
    );
    assert_ne!(
        node_in_dag.expect("Invalid value assumption.").left, 
        node_b.left,
    );
    assert_ne!(
        node_in_dag.expect("Invalid value assumption.").right, 
        node_b.right,
    );
    // Examine the DAG's collition collection.
    let collitions: &HashSet<CollidingNode<TestType>> = dag.get_collitions(&id).expect("Invalid value assumption.");
    let colliding_node = collitions.get(&CollidingNode::from(node_b)).expect("Invalid value assumption.");
    // The colliding node corresponds to node_b.
    assert_eq!(colliding_node, &CollidingNode::from(node_b));
    // The collitions set contains a CollidingNode that corresponds to node_b.
    assert!(collitions.contains(&node_b.into()));
    // The collitions set does not contains a CollitionNode<Node<T>> that corresponds to node_a.
    assert!(!collitions.contains(&node_a.into()));
    assert!(!collitions.contains(&CollidingNode::from(node_a)));
    // The fields in the colliding node corresponds to node_b, but not to node_a.
    assert!(colliding_node.has_same_fields_to(&node_b.into()));
    assert!(!colliding_node.has_same_fields_to(&node_a.into()))
}

#[test]
fn insert_existing_node_marks_dag_unsafe() {
    type TestType = u32;
    let id: TestType = 0;
    let node_a = Node::new(id,None,None);
    let node_b = Node::new(id,Some(42),Some(43));
    let mut dag = Dag::new();
    // Inserts node_a into the DAG.
    assert_eq!(dag.insert(node_a), None);
    assert!(dag.get(&node_a.id).expect("Invalid value assumption.").has_same_fields_to(&node_a));
    // Dag is still marked as safe (no cycles) after first insertion.
    assert!(dag.is_safe());
    // Trying to insert a node with the id of an already inserted node collects the collition and returns the value of the previously inserted node, which will persist in the DAG.
    assert!(dag.insert(node_b).expect("Invalid value assumption.").has_same_fields_to(&node_a));
    // Examine the DAG's collition collection.
    let collitions: &HashSet<CollidingNode<TestType>> = dag.get_collitions(&id).expect("Invalid value assumption.");
    let colliding_node = collitions.get(&CollidingNode::from(node_b)).expect("Invalid value assumption.");
    assert_eq!(colliding_node, &CollidingNode::from(node_b));
    assert_eq!(colliding_node, &CollidingNode::from(node_b));

    // Node in dag with id 0 is still node_a, but not node_b.
    let node_in_dag = dag.get(&node_a.id).expect("Invalid value assumption.");
    assert!(node_in_dag.has_same_fields_to(&node_a));

    // insert_or_update updates the dag values for the node id 0 and returns the previous values, node_a. 
    // The final fields values for the node id 0 in the dag are equal to node_b but not node_a.
    assert!(dag.insert_or_update(node_b).expect("Invalid value assumption.").has_same_fields_to(&node_a));
    assert!(!dag.get(&node_a.id).expect("Invalid value assumption.").has_same_fields_to(&node_a));
    assert!(dag.get(&node_a.id).expect("Invalid value assumption.").has_same_fields_to(&node_b));

    assert!(!dag.is_safe());
}

#[test]
fn insert_a_node_with_non_existent_left_reference_marks_dag_unsafe() {
    type TestType = u32;
    let id: TestType = 0;
    let node_a = Node::new(id,Some(3),None);
    let mut dag = Dag::new();
    assert_eq!(dag.insert(node_a), Some(node_a));
    assert_eq!(dag.is_safe(), false);
}

#[test]
fn insert_a_node_with_non_existent_right_reference_marks_dag_unsafe() {
    type TestType = u32;
    let id: TestType = 0;
    let node_a = Node::new(id,None,Some(5));
    let mut dag = Dag::new();
    assert_eq!(dag.insert(node_a), Some(node_a));
    assert_eq!(dag.is_safe(), false);
}

#[test]
fn insert_non_existent_nodes_with_existent_references_is_safe() {
    let node_a = Node::new(0,None,None);
    let node_b = Node::new(1,Some(0),None);
    let node_c = Node::new(2,None,Some(0));
    let node_d = Node::new(3,Some(0), Some(1));
    let node_e = Node::new(4,Some(2), Some(1));
    let node_f = Node::new(5,Some(3), Some(4));
    let mut dag = Dag::new();
    assert_eq!(dag.insert(node_a), None);
    assert_eq!(dag.insert(node_b), None);
    assert_eq!(dag.insert(node_c), None);
    assert_eq!(dag.insert(node_d), None);
    assert_eq!(dag.insert(node_e), None);
    assert_eq!(dag.insert(node_f), None);
    assert!(dag.is_safe())
}

#[test]
fn insert_dag_from_list() {
    let node_a = Node::new(0,None,None);
    let node_b = Node::new(1,Some(0),None);
    let node_c = Node::new(2,None,Some(0));
    let node_d = Node::new(3,Some(0), Some(1));
    let node_e = Node::new(4,Some(2), Some(1));
    let node_f = Node::new(5,Some(3), Some(4));
    let mut dag = Dag::new();
    let insert_result = dag.insert_from(&[node_a, node_b, node_c, node_d, node_e, node_f]);
    for result in insert_result {
        assert_eq!(result, None)
    };
    assert!(dag.is_safe())
}