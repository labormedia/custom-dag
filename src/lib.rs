#![allow(unused_imports)]
#![allow(dead_code)]
//! Custom-dag is a general purpose directed acyclic graph analyzer written entirely on Rust.
//! # How to use
//! ```
//! use custom_dag::{
//!    Node,
//!    collitions::CollidingNode,
//!    Dag,
//!    topological::Topology,
//! };
//! let node_a = Node::new(0,None,None,());
//! let node_b = Node::new(1,Some(0),None,());
//! let node_c = Node::new(2,None,Some(0),());
//! let node_d = Node::new(3,Some(0), Some(1), ());
//! let node_e = Node::new(4,Some(2), Some(1), ());
//! let node_f = Node::new(5,Some(3), Some(4), ());
//! let node_list = [node_a, node_b, node_c, node_d, node_e, node_f];
//! let ordering = Topology::sort(&node_list).unwrap().unwrap();
//! assert!(ordering.len() > 0);
//! let mut dag = Dag::new();
//! for node in ordering {
//!     dag.insert(node);
//! };
//! assert!(dag.is_safe());
//! 
//!```
//! # Examples
//! Examples are provided in the examples directory:
//! - hello-dag
//! - dag-stats
//! - wasm-binding

use std::collections::{
    HashMap,
    HashSet,
};
use core::{
    hash::Hash,
    fmt::Debug
};
/// This module includes the code necessary for node collition analysis when manipulating the DAG structure.
pub mod collitions;
/// This modules includes the helpers necessary for topological analysis of dag structure.
#[allow(unused_imports)]
pub mod topological;
use collitions::CollidingNode;
pub mod error;
use error::TopologicalError;

use serde::{Serialize, Deserialize};  // Serde is called for wasm-bindgen implementation.

/// Custom Node struct.
/// Nodes makes no assumption about the structure of the graph they are inserted in.
#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, Hash, Copy)]
pub struct Node<T: Eq + Hash + PartialEq + Copy, U: Eq + Hash + PartialEq + Copy> {
    pub id: T,
    pub left: Option<T>,
    pub right: Option<T>,
    pub payload: U,
}

impl<T: Eq + Hash + PartialEq + Copy, U: Eq + Hash + PartialEq + Copy> PartialEq<Node<T, U>> for Node<T, U> {
    fn eq(&self, other: &Node<T, U>) -> bool {
        self.id == other.id
    }
}

impl<T: Eq + Hash + PartialEq + Copy, U: Eq + Hash + PartialEq + Copy> Eq for Node<T, U> {}

impl<T: Eq + Hash + PartialEq + Copy, U: Eq + Hash + PartialEq + Copy> Node<T, U> {
    /// The Node itself can be self-referential and makes no assumptions about the structure of the graph.
    /// Nodes define two ancestors by their id defined in left and right fields of same type as an option containing the id, i.e. `Some(id)`, of the node being referenced.
    /// If that branch does not references any node, the value can take the value `Option::<T>::None`.
    pub fn new(id: T, left: Option<T>, right: Option<T>, payload: U) -> Self {
        Node {
            id,
            left,
            right,
            payload
        }
    }
    /// Compares the equality of all fields from the base node to the reference of other node presented as the argument.
    pub fn has_same_fields_to(&self, node: &Node<T, U>) -> bool {
        self.id == node.id
        &&
        self.left == node.left
        &&
        self.right == node.right
    }
    /// Counts non `None` references of the node.
    pub fn in_degree(&self) -> usize {
        let mut in_degree: usize = 0;
        if self.left.is_some() { in_degree += 1 };
        if self.right.is_some() { in_degree += 1 };
        in_degree
    }
}

/// Dag struct.
#[derive(Debug, Clone)]
pub struct Dag<T: Eq + Hash + PartialEq + Copy, U: Eq + Hash + PartialEq + Copy> {
    nodes: HashMap<T, Node<T, U>>,
    possible_collitions: HashMap<T, HashSet<CollidingNode<T, U>>>,
    is_safe: bool,
}

impl<T: Eq + Hash + PartialEq + Copy + Debug, U: Eq + Hash + PartialEq + Copy + Debug> Dag<T, U> {
    /// Creates a new empty Dag marked as safe.
    pub fn new() -> Self {
        Dag {
            nodes: HashMap::new(),
            possible_collitions: HashMap::new(),
            is_safe: true,
        }
    }
    /// Check the safety of a given topological order creating a new Dag from a list of nodes given as a slice references of Nodes, i.e. `&[Node<T>]`.
    /// If the generation is succesful (i.e. conforms to a topological order for the nodes list) it returns `true`, otherwise `false`.
    pub fn check_topological_order(node_list: &[Node<T, U>]) -> bool {
        let mut topology = Self::new();
        topology.insert_from(node_list);
        topology.is_safe()
    }
    /// Inserts nodes to the dag from a list.
    pub fn insert_from(&mut self, node_list: &[Node<T, U>]) -> Vec<Option<Node<T, U>>> {
        let node_iterator = node_list.iter();
        node_iterator.map(|node| { 
            self.insert(*node)
        }).collect()
    }
    /// Inserts a value only if the value doesn't exists, otherwise it collects it on a collition map.
    /// Note that as nodes only specifies ancestors and its id (but not descendants), if the node's id are not already included in the dag before the insertion
    /// but their references are or None, then it will always preserve the structure of a DAG.
    /// If the intented behaviour is updating an existing value, insert_or_update method should be used instead,
    /// though this will create an unsafe condition for the acyclic structure of the DAG, and this will be marked in the is_safe (bool) field.
    /// If the id is not present in the dag but their references are, the node is inserted and None is returned.
    /// If the id is not present in the dag and at least one of their reference is neither, it inserts the node in the dag but marks the is_safe flag as false and returns an option with the new value added to the dag.
    /// If the id is present it does not update the dag, returns an option with the value that was present previously and accumulates the collition.
    pub fn insert(&mut self, node: Node<T, U>) -> Option<Node<T, U>> {
        if self.nodes.contains_key(&node.id) {
            match self.possible_collitions.get_mut(&node.id) {
                Some(collition_set) => { 
                    collition_set.insert(node.into());
                },
                None => {
                    let mut collition_set = HashSet::new();
                    assert!(collition_set.insert(CollidingNode::from(node)));
                    assert_eq!(self.possible_collitions.insert(node.id, collition_set), None);
                },
            };
            self.nodes.get(&node.id).copied()
        } else if node.left.is_some()
            && !self.nodes.contains_key(&node.left.expect("Invalid type definition assumption for Node<T>."))
        {
            self.is_safe = false;
            assert_eq!(self.nodes.insert(node.id, node), None);
            self.nodes.get(&node.id).copied()
        } else if node.right.is_some() 
            && !self.nodes.contains_key(&node.right.expect("Invalid type definition assumption for Node<T>."))
        {
            self.is_safe = false;
            assert_eq!(self.nodes.insert(node.id, node), None);
            self.nodes.get(&node.id).copied()
        }
        else {
            assert_eq!(self.nodes.insert(node.id, node), None);
            None
        }
    }
    /// This method updates a node if it already exists.
    /// If this method is used effectively, the DAG will be marked as unsafe in its `is_safe` field as a `false` value.
    pub fn insert_or_update(&mut self, node: Node<T, U>) -> Option<Node<T, U>> {
        self.is_safe = false;
        self.nodes.insert(node.id, node)
    }
    /// Searches for nodes by id and returns `true` if present in the nodes list.
    pub fn contains_id(&self, id: &T) -> bool {
        self.nodes.contains_key(id)
    }
    /// Gets a node by id. Returns `Some(id)` if present, or `None` if not.
    pub fn get(&self, id: &T) -> Option<&Node<T, U>> {
        self.nodes.get(id)
    }
    /// Returns an Option with a reference of the a HashSet of possible collitions for the list of nodes inserted.
    pub fn get_collitions(&self, id: &T) -> Option<&HashSet<CollidingNode<T, U>>> {
        self.possible_collitions.get(id)
    }
    /// Gets the value of the dag safety marker.
    pub fn is_safe(&mut self) -> bool {
        self.is_safe
    }
}

#[cfg(test)]
mod tests {
    use crate::Node;

    #[test]
    fn create_node_without_references() {
        let node = Node::new(0,None,None,());
        assert_eq!(
            node, 
            Node {
                id: 0,
                left: None,
                right: None,
                payload: (),
            }
        )
    }

    #[test]
    fn node_is_equivalent_by_id() {
        let node = Node::new(0,None,None,());
        assert_eq!(
            node, 
            Node {
                id: 0,
                left: Some(1),
                right: Some(2),
                payload: (),
            }
        );
        assert_ne!(
            node, 
            Node {
                id: 1,
                left: None,
                right: None,
                payload: (),
            }
        )
    }

    #[test]
    fn create_node_zero_u32() {
        let node = Node::new(0,Some(0),Some(0),());
        assert_eq!(
            node, 
            Node {
                id: 0,
                left:Some(0),
                right: Some(0),
                payload: (),
            }
        )
    }

    #[test]
    fn create_node_zero_u64() {
        let value = 0_u64;
        let node = Node::new(value,Some(0),Some(0),());
        assert_eq!(
            node, 
            Node {
                id: 0,
                left:Some(0),
                right: Some(0),
                payload: (),
            }
        )
    }

    #[test]
    fn create_node_zero_usize() {
        let value = 0_usize;
        let node = Node::new(value,Some(0),Some(0),());
        assert_eq!(
            node, 
            Node {
                id: 0,
                left:Some(0),
                right: Some(0),
                payload: (),
            }
        )
    }
    #[test]
    fn has_same_fields_to() {
        type TestType = u32;
        let id: TestType = 0;
        let node_a = Node::new(id,Some(3),Some(5),());
        let node_b = Node::new(id,Some(42),Some(43),());

        let node_c = Node::new(id, Some(3), Some(5), ());

        assert!(node_a.has_same_fields_to(&node_a));
        assert!(node_a.has_same_fields_to(&node_c));
        assert!(!node_a.has_same_fields_to(&node_b));
    }
}