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
                self.insert_edges(node); // collect the node's edges
            } else { // else if the node was already inserted in the collection of unique nodes (compared by id)
                if self.collitions.insert(node.into()) { // when inserting the node to the collection of collitions, if the node was not listed there
                    // done
                } else {
                    self.insert_repeated_node(node); // also collect the node as a repeated_node
                };
            };
        } else {
        };
        Ok(node)
    }
    fn insert_repeated_node(&mut self, node: Node<T>) {
        if let Some(repeated_nodes_set) = self.repeated_nodes.get(&node.id) {

        } else {
            let mut new_set: HashSet<CollidingNode<T>> = HashSet::new();
            new_set.insert(node.into());
            assert_eq!(self.repeated_nodes.insert(node.id, new_set), None);
        };
    }
    fn insert_edges(&mut self, node: Node<T>) {
        match node.left {
            None => {},
            Some(ancestor) => {
                if let Some(edges_for_ancestor) = self.edges.get_mut(&ancestor) { // pushes the node.id to the list of its ancestor's directed edges (directed to it)
                    edges_for_ancestor.push(node.id); 
                } else {
                    assert_eq!(self.edges.insert(ancestor, vec!(node.id)), None);
                }
            },
        };
        match node.right {
            None => {},
            Some(ancestor) => {
                if let Some(edges_for_ancestor) = self.edges.get_mut(&ancestor) { // pushes the node.id to the list of its ancestor's directed edges (directed to it
                    edges_for_ancestor.push(node.id); 
                } else {
                    assert_eq!(self.edges.insert(ancestor, vec!(node.id)), None);
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