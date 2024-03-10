use core::{
    hash::Hash,
};
use std::{
    collections::{
        HashSet,
        HashMap,
    },
    error::Error,
    fmt,
};
use crate::{
    Node,
    collitions::CollidingNode,
};

#[derive(Debug)]
enum TopologicalError {
    Custom,
    RepeatedNodes,
    WrongTopologicalAssumptions
}

impl Error for TopologicalError {}

impl fmt::Display for TopologicalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom => write!(f, "Custom error"),
            Self::RepeatedNodes => write!(f, "The list has repeated nodes."),
            Self::WrongTopologicalAssumptions => write!(f, "Wrong topological assumptions."),
        }
    }
}

/// Topology struct layout for analysis.
#[derive(Debug, Clone)]
pub struct Topology<T: Eq + Hash + PartialEq + Copy + std::fmt::Debug> {
    all_nodes: HashSet<CollidingNode<T>>, // collection of all nodes compared by all fields
    unique_nodes: HashMap<T, Node<T>>,
    collitions: HashSet<CollidingNode<T>>,
    repeated_nodes: HashMap<T, HashSet<CollidingNode<T>>>,
    outgoing_edges: HashMap<T, Vec<T>>,
}

impl<T: Eq + Hash + PartialEq + Copy + std::fmt::Debug> Topology<T> {
    fn new() -> Topology<T> {
        Topology {
            all_nodes: HashSet::new(),
            unique_nodes: HashMap::new(),
            collitions: HashSet::new(),
            repeated_nodes: HashMap::new(),
            outgoing_edges: HashMap::new(),
        }
    }
    /// Inserts the node into the topology analysis.
    /// If it finds a collition, returns an Option<CollidingNode<T>> with the value of the node.
    /// otherwise it returns None if there is no collision.
    fn insert(&mut self, node: Node<T>) -> Option<CollidingNode<T>>{
        if self.all_nodes.insert(node.into()) { // if the node didn't existed in the collection of all nodes (compared by all fields)
            if self.get_unique_node_by_id(node.id) == None { // if the node didn't existed in the collection of unique nodes (compared by id)
                assert_eq!(self.unique_nodes.insert(node.id, node), None); // Inserts node to the unique nodes collection.
                self.collect_edges(node); // collect the node's edges
                None
            } else { // else if the node was already inserted in the collection of unique nodes (compared by id)
                if self.collitions.insert(node.into()) { // insert the node to the collection of collitions; if the node was not listed there
                    // done
                } else {
                    self.collect_repeated_node(node); // also collect the node as a repeated_node
                };
                Some(node.into())  // return the colliding node.
            }
        } else { // else if the node existed in the collection of all nodes (compared by all fields)
            assert_eq!(self.get_unique_node_by_id(node.id), Some(node));  // It should have been already added to the collection of unique nodes (compared by id).
            self.collect_repeated_node(node); // and collects it to the collection of repeated nodes.
            Some(node.into()) // return the colliding node
        }
    }
    fn collect_repeated_node(&mut self, node: Node<T>) {
        if let Some(repeated_nodes_set) = self.repeated_nodes.get_mut(&node.id) { // if the set of repeated nodes is already created for this node.id
            repeated_nodes_set.insert(node.into());  // insert the node into the collection of repeated nodes
        } else {  // else, if the set of repeated nodes have not been created yet for this node.id
            let mut new_set: HashSet<CollidingNode<T>> = HashSet::new(); // creates a new set
            new_set.insert(node.into());  // inserts the node into the set
            assert_eq!(self.repeated_nodes.insert(node.id, new_set), None); // and inserts the set into the collection of repeated nodes for this node.id
        };
    }
    fn collect_edges(&mut self, node: Node<T>) {
        match node.left {
            None => {},
            Some(ancestor) => {
                if let Some(edges_for_ancestor) = self.outgoing_edges.get_mut(&ancestor) { // pushes the node.id to the list of its ancestor's directed edges (directed to it)
                    edges_for_ancestor.push(node.id);  // Inserts the edge 
                } else {
                    assert_eq!(self.outgoing_edges.insert(ancestor, vec!(node.id)), None);  // Inserts the edge and asserts that the list didn't existed before.
                }
            },
        };
        match node.right {
            None => {},
            Some(ancestor) => {
                if let Some(edges_for_ancestor) = self.outgoing_edges.get_mut(&ancestor) { // pushes the node.id to the list of its ancestor's directed edges (directed to it
                    edges_for_ancestor.push(node.id); // Inserts the edge
                } else {
                    assert_eq!(self.outgoing_edges.insert(ancestor, vec!(node.id)), None); // Inserts the edge and asserts that the list didn't existed before.
                }
            },
        };
    }
    fn edge_sum(&self) -> usize {
        self
            .outgoing_edges
            .iter()
            .map(|(from, list)| { list.len() })
            .sum()
    }
    fn get_unique_node_by_id(&self, id:T) -> Option<Node<T>> {
        self.unique_nodes.get(&id).copied()
    }
    /// Checks the consistency of nodes with its references, 
    /// i.e. checks: 
    /// * if all nodes references have been defined -> true
    /// * if there are no repeated nodes -> true
    /// * if there are no collitions -> true
    /// otherwise returns false
    /// References to itself are not considered.
    fn is_consistent(&self) -> bool {
        self.repeated_nodes.len() == 0
        && self.collitions.len() == 0
        && self.unique_nodes
            .iter()
            .fold(true, |acc, node| {
                acc &&
                ( node.1.left == None || self.get_unique_node_by_id(node.1.left.expect("Wrong value assumption")) != None ) &&
                ( node.1.right == None || self.get_unique_node_by_id(node.1.right.expect("Wrong value assumption")) != None )
            })
    }
    /// Tries to build a topological sort from a list of nodes.
    /// Returns a sequence of nodes that follows a topological order if it exists, otherwise it returns None.
    fn sort(nodes:&[Node<T>]) -> Result<Option<Vec<T>>, Box<dyn Error> > {
        let mut topology: Topology<T> = Topology::new();
        for node in nodes.iter() {
            topology.insert(*node);
        };
        let checked_topology = if topology.collitions.len() == 0
            && topology.repeated_nodes.len() == 0
            && topology.is_consistent()
            { Some(&topology) } else { None };
        match checked_topology {
            Some(topology_for_sorting) => { // sorting algorithm
                let mut in_degree_map: HashMap<&T,usize> =
                    topology_for_sorting
                        .unique_nodes
                        .iter()
                        .map( |node| {
                            (node.0, node.1.in_degree())
                        })
                        .collect();
                let mut ordering: Vec<T> = Vec::with_capacity(in_degree_map.len());
                let ordering = loop {
                    let next_ordering: Vec<T> = in_degree_map
                        .clone()
                        .into_iter()
                        .filter( |(id,in_degree)| { 
                            in_degree == &0_usize 
                        })
                        .map( |(id, degree)| {
                            *id
                        })
                        .collect();
                    if next_ordering.len() > 0 {
                        for id in next_ordering.iter() {
                            ordering.push(*id);
                            match topology_for_sorting.outgoing_edges.get(id) {
                                Some(edges) => {
                                    assert!(edges.len() > 0); // if it was inserted, should have values.
                                    for outgoing_node_id in edges {
                                        let in_degree = in_degree_map.get_mut(outgoing_node_id).ok_or(TopologicalError::WrongTopologicalAssumptions)?;
                                        assert!( in_degree > &mut 0 );
                                        *in_degree -= 1;
                                    };
                                    Some(edges)
                                },
                                None => None
                            };
                            assert_eq!(in_degree_map.remove(id), Some(0_usize));
                        }
                    } else { break ordering; };
                };
                Ok(Some(ordering))
            },
            None => Ok(None)
        }
    }
}

#[test]
fn insert_nodes_in_topology_analysis() {
    let node_a = Node::new(0,None,None);
    let node_b = Node::new(1,Some(0),None);
    let node_c = Node::new(2,None,Some(0));
    let node_d = Node::new(3,Some(0), Some(1));
    let node_e = Node::new(4,Some(2), Some(1));
    let node_f = Node::new(5,Some(3), Some(4));
    let mut topology = Topology::new();
    let node_list = [node_a, node_b, node_c, node_d, node_e, node_f];
    for node in node_list.into_iter() {
        assert_eq!(topology.insert(node), None); // All nodes inserted are new nodes to the topology.
    };
    assert_eq!(topology.all_nodes.get(&Node::new(23, Some(42), Some(50)).into()), None);  // tries to take an inexistent node
    for node in node_list.into_iter() {
        assert_eq!(topology.all_nodes.get(&node.into()).expect("Wrong value assumption."), &CollidingNode(node));
    };
    for node in node_list.into_iter() {
        assert_eq!(topology.get_unique_node_by_id(node.id).expect("Wrong value assumption."), node);
    };
    assert_eq!(topology.collitions.len(), 0);
    assert_eq!(topology.repeated_nodes.len(), 0);
    assert_eq!(topology.edge_sum(), 8);
    assert_eq!(topology.unique_nodes.len(), 6);
    assert_eq!(topology.get_unique_node_by_id(4), Some(node_e)); // Check existence for node_e
    for (from, to_list) in topology.outgoing_edges.iter() {
        for to in to_list.iter() {
            let compare_node = topology.get_unique_node_by_id(*to).expect("Wrong value assumptions");
            assert!(compare_node.left == Some(*from) || compare_node.right == Some(*from)); // Checks that all edges corresponds to a node in the original node's list.
        }
    };
    assert!(topology.is_consistent()); // Upto this stage, the topology is consistent.
    let colliding_node = Node::new(4, None, None);
    assert_eq!(topology.insert(colliding_node), Some(node_e.into())); // Tries to insert a node with the same id of an already indexed node in the topology analysis.
    assert_eq!(topology.collitions.len(), 1); // There's one collition.
    assert_eq!(topology.get_unique_node_by_id(4), Some(node_e)); // Checks that the Node in the collection of unique nodes is still the first one added by the same id.
    assert_eq!(topology.collitions.get(&node_e.into()), None); // The first added node_e will not be present in the collitions set.
    assert_eq!(topology.collitions.get(&colliding_node.into()), Some(&colliding_node.into()));  // The colliding node will be present in the collitions set.
    assert_eq!(topology.repeated_nodes.len(), 0);  // There is no repeated nodes yet.
    assert_eq!(topology.insert(colliding_node), Some(node_e.into()));
    assert_eq!(topology.repeated_nodes.len(), 1); // There is one repeated node i.e. the colliding node.
    assert_eq!(topology.insert(node_b), Some(node_b.into())); // Tries to insert node_b again.
    assert_eq!(topology.collitions.len(), 1); // The number of collitions is still 1.
    assert_eq!(topology.repeated_nodes.len(), 2); // The number of repeated nodes is now 2.
    assert_eq!(topology.edge_sum(), 8); // The number of edges has not changed.
    assert_eq!(topology.unique_nodes.len(), 6);  // The number of unique nodes has not changed.
    for node in node_list.into_iter() {
        assert_eq!(topology.get_unique_node_by_id(node.id).expect("Wrong value assumption."), node); // The map of unique_nodes is still the original list.
    };
    for (from, to_list) in topology.outgoing_edges.iter() {
        for to in to_list.iter() {
            let compare_node = topology.get_unique_node_by_id(*to).expect("Wrong value assumptions");
            assert!(compare_node.left == Some(*from) || compare_node.right == Some(*from)); // All edges still corresponds to a node in the original node's list.
        }
    };
    assert!(!topology.is_consistent()); // At this stage, the topology is *NOT* consistent.
}

fn insert_node_with_inexistent_references_is_inconsistent() {
    let mut topology: Topology<u32> = Topology::new();
    let node = Node::new(1, None, Some(2));
    topology.insert(node);
    assert_eq!(topology.collitions.len(), 0);
    assert_eq!(topology.repeated_nodes.len(), 0);
    assert!(!topology.is_consistent());
}

#[test]
fn topological_order() {
    let node_a = Node::new(0,None,None);
    let node_b = Node::new(1,Some(0),None);
    let node_c = Node::new(2,None,Some(0));
    let node_d = Node::new(3,Some(0), Some(1));
    let node_e = Node::new(4,Some(2), Some(1));
    let node_f = Node::new(5,Some(3), Some(4));
    let node_list = [node_a, node_b, node_c, node_d, node_e, node_f];
    let ordering = Topology::sort(&node_list);
    println!("Original list: {:?}", node_list);
    println!("Ordered list: {:?}", ordering);
}