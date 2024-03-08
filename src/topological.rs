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
enum TopologicalErrors {
    Custom,
    RepeatedNodes,
}

impl fmt::Display for TopologicalErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom => write!(f, "Custom error"),
            Self::RepeatedNodes => write!(f, "The list has repeated nodes."),
        }
    }
}

/// Topology struct layout for analysis.
#[derive(Debug, Clone)]
struct Topology<T: Eq + Hash + PartialEq + Copy> {
    all_nodes: HashSet<CollidingNode<T>>,
    unique_nodes: HashSet<Node<T>>,
    collitions: HashSet<CollidingNode<T>>,
    repeated_nodes: HashMap<T, HashSet<CollidingNode<T>>>,
    edges: HashMap<T, Vec<T>>,
}

impl<T: Eq + Hash + PartialEq + Copy> Topology<T> {
    fn new() -> Topology<T> {
        Topology {
            all_nodes: HashSet::new(),
            unique_nodes: HashSet::new(),
            collitions: HashSet::new(),
            repeated_nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }
    fn insert(&mut self, node: Node<T>) {
        if !self.all_nodes.insert(node.into()) {
            // self.repeated_nodes.get(node.id, node.into());
        };
    }
    fn sort(nodes:&[CollidingNode<T>]) -> Result<Topology<T>, TopologicalErrors> {
        let topology: Topology<T> = Topology::new();
        for nodes in nodes.iter() {
            // topology.all_nodes
        };
        Ok::<Topology<T>, TopologicalErrors>(topology);
        Err(TopologicalErrors::Custom)
    }
}