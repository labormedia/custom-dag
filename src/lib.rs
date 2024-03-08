//! Custom-dag is a general purpose directed acyclic graph analyzer written entirely on Rust.
//! # How to use
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
use collitions::CollidingNode;

/// Custom Node struct.
#[derive(Debug, Clone, Hash, Copy)]
pub struct Node<T: Eq + Hash + PartialEq + Copy> {
    pub id: T,
    pub left: Option<T>,
    pub right: Option<T>,
}

impl<T: Eq + Hash + PartialEq + Copy> PartialEq<Node<T>> for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        self.id == other.id
    }
}

impl<T: Eq + Hash + PartialEq + Copy> Eq for Node<T> {}

impl<T: Eq + Hash + PartialEq + Copy> Node<T> {
    /// The Node itself can be self-referential and makes no assumptions about the structure of the graph.
    /// Nodes define two ancestors by their id defined in left and right fields of same type as the id of the node itself.
    pub fn new(id: T, left: Option<T>, right: Option<T>) -> Self {
        Node {
            id,
            left,
            right,
        }
    }
    pub fn has_same_fields_to(&self, node: &Node<T>) -> bool {
        self.id == node.id
        &&
        self.left == node.left
        &&
        self.right == node.right
    }
}

/// Dag struct.
#[derive(Debug, Clone)]
pub struct Dag<T: Eq + Hash + PartialEq + Copy> {
    nodes: HashMap<T, Node<T>>,
    possible_collitions: HashMap<T, HashSet<CollidingNode<T>>>,
    is_safe: bool,
}

impl<T: Eq + Hash + PartialEq + Copy + Debug> Dag<T> {
    /// Creates a new empty Dag marked as safe (No possible cycles).
    pub fn new() -> Self {
        Dag {
            nodes: HashMap::new(),
            possible_collitions: HashMap::new(),
            is_safe: true,
        }
    }
    /// Inserts nodes to the dag from a list.
    pub fn insert_from(&mut self, node_list: &[Node<T>]) -> Vec<Option<Node<T>>> {
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
    pub fn insert(&mut self, node: Node<T>) -> Option<Node<T>> {
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
        } else if node.left != None
            && !self.nodes.contains_key(&node.left.expect("Wrong type definition assumption for Node<T>."))
        {
            self.is_safe = false;
            assert_eq!(self.nodes.insert(node.id, node), None);
            self.nodes.get(&node.id).copied()
        } else if node.right != None 
            && !self.nodes.contains_key(&node.right.expect("Wrong type definition assumption for Node<T>."))
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
    /// If this method is used effectively, the DAG will be marked as unsafe in its is_safe field as a false value.
    pub fn insert_or_update(&mut self, node: Node<T>) -> Option<Node<T>> {
        self.is_safe = false;
        self.nodes.insert(node.id, node)
    }
    pub fn contains_id(&self, id: &T) -> bool {
        self.nodes.contains_key(id)
    }
    pub fn get(&self, id: &T) -> Option<&Node<T>> {
        self.nodes.get(id)
    }
    pub fn get_collitions(&self, id: &T) -> Option<&HashSet<CollidingNode<T>>> {
        self.possible_collitions.get(id)
    }
    pub fn is_safe(&mut self) -> bool {
        if self.is_safe { true } else { false }
    }
}

#[cfg(test)]
mod tests {
    use crate::Node;

    #[test]
    fn create_node_without_references() {
        let node = Node::new(0,None,None);
        assert_eq!(
            node, 
            Node {
                id: 0,
                left: None,
                right: None,
            }
        )
    }

    #[test]
    fn node_is_equivalent_by_id() {
        let node = Node::new(0,None,None);
        assert_eq!(
            node, 
            Node {
                id: 0,
                left: Some(1),
                right: Some(2),
            }
        );
        assert_ne!(
            node, 
            Node {
                id: 1,
                left: None,
                right: None,
            }
        )
    }

    #[test]
    fn create_node_zero_u32() {
        let node = Node::new(0,Some(0),Some(0));
        assert_eq!(
            node, 
            Node {
                id: 0,
                left:Some(0),
                right: Some(0),
            }
        )
    }

    #[test]
    fn create_node_zero_u64() {
        let value = 0_u64;
        let node = Node::new(value,Some(0),Some(0));
        assert_eq!(
            node, 
            Node {
                id: 0,
                left:Some(0),
                right: Some(0),
            }
        )
    }

    #[test]
    fn create_node_zero_usize() {
        let value = 0_usize;
        let node = Node::new(value,Some(0),Some(0));
        assert_eq!(
            node, 
            Node {
                id: 0,
                left:Some(0),
                right: Some(0),
            }
        )
    }
    #[test]
    fn has_same_fields_to() {
        type TestType = u32;
        let id: TestType = 0;
        let node_a = Node::new(id,Some(3),Some(5));
        let node_b = Node::new(id,Some(42),Some(43));

        let node_c = Node::new(id, Some(3), Some(5));

        assert!(node_a.has_same_fields_to(&node_a));
        assert!(node_a.has_same_fields_to(&node_c));
        assert!(!node_a.has_same_fields_to(&node_b));
    }
}