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
    WrongTopologicalAssumtpions
}

impl fmt::Display for TopologicalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom => write!(f, "Custom error"),
            Self::RepeatedNodes => write!(f, "The list has repeated nodes."),
            Self::WrongTopologicalAssumtpions => write!(f, "Wrong topological assumptions."),
        }
    }
}

/// Topology struct layout for analysis.
#[derive(Debug, Clone)]
struct Topology<T: Eq + Hash + PartialEq + Copy + std::fmt::Debug> {
    all_nodes: HashSet<CollidingNode<T>>, // collection of all nodes compared by all fields
    unique_nodes: HashSet<Node<T>>,
    collitions: HashSet<CollidingNode<T>>,
    repeated_nodes: HashMap<T, HashSet<CollidingNode<T>>>,
    edges: HashMap<T, Vec<T>>,
}

impl<T: Eq + Hash + PartialEq + Copy + std::fmt::Debug> Topology<T> {
    fn new() -> Topology<T> {
        Topology {
            all_nodes: HashSet::new(),
            unique_nodes: HashSet::new(),
            collitions: HashSet::new(),
            repeated_nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }
    fn insert(&mut self, node: Node<T>) -> Result<Node<T>, TopologicalError>{
        if self.all_nodes.insert(node.into()) { // if the node didn't existed in the collection of all nodes (compared by all fields)
            if self.unique_nodes.insert(node) { // if the node didn't existed in the collection of unique nodes (compared by id)
                self.collect_edges(node); // collect the node's edges
            } else { // else if the node was already inserted in the collection of unique nodes (compared by id)
                if self.collitions.insert(node.into()) { // when inserting the node to the collection of collitions, if the node was not listed there
                    // done
                } else {
                    self.collect_repeated_node(node); // also collect the node as a repeated_node
                };
            };
        } else { // else if the node existed in the collection of all nodes (compared by all fields)
            assert!(self.unique_nodes.insert(node));  // It should have been already added to the collection of unique nodes (compared by id).
            self.collect_repeated_node(node); // and collects it to the collection of repeated nodes.
        };
        Ok(node)
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
                if let Some(edges_for_ancestor) = self.edges.get_mut(&ancestor) { // pushes the node.id to the list of its ancestor's directed edges (directed to it)
                    edges_for_ancestor.push(node.id);  // Inserts the edge 
                } else {
                    assert_eq!(self.edges.insert(ancestor, vec!(node.id)), None);  // Inserts the edge and asserts that the list didn't existed before.
                }
            },
        };
        match node.right {
            None => {},
            Some(ancestor) => {
                if let Some(edges_for_ancestor) = self.edges.get_mut(&ancestor) { // pushes the node.id to the list of its ancestor's directed edges (directed to it
                    edges_for_ancestor.push(node.id); // Inserts the edge
                } else {
                    assert_eq!(self.edges.insert(ancestor, vec!(node.id)), None); // Inserts the edge and asserts that the list didn't existed before.
                }
            },
        };
    }
    fn sort(nodes:&[CollidingNode<T>]) -> Result<Topology<T>, TopologicalError> {
        let topology: Topology<T> = Topology::new();
        for nodes in nodes.iter() {
            // topology.all_nodes
        };
        Ok::<Topology<T>, TopologicalError>(topology);
        Err(TopologicalError::Custom)
    }
}