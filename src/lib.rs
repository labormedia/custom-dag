use std::collections::HashMap;
use core::{
    hash::Hash,
    fmt::Debug
};

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
    // The Node itself can be self-referential and makes no assumptions about the structure of the graph.
    pub fn new(id: T, left: Option<T>, right: Option<T>) -> Self {
        Node {
            id,
            left,
            right,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dag<T: Eq + Hash + PartialEq + Copy> {
    nodes: HashMap<T, Node<T>>,
    collitions: HashMap<T, Vec<Node<T>>>
}

impl<T: Eq + Hash + PartialEq + Copy + Debug> Dag<T> {
    pub fn new() -> Self {
        Dag {
            nodes: HashMap::new(),
            collitions: HashMap::new(),
        }
    }
    // Inserts a value only if the value doesn't exists, otherwise it accumulates it collects it on a collition map.
    // If the intented behavious is updating an existing value, insert_or_update method should be used instead.
    // If the id is not present in the dag, the node is inserted and None is returned.
    // If the id is present, it does not update the dag. Instead it returns the value that was present before and accumulates the collition.
    pub fn insert(&mut self, node: Node<T>) -> Option<&Node<T>> {
        let id = &node.id;
        if self.nodes.contains_key(id) {
            self.nodes.get(id)
        } else {
            assert_eq!(self.nodes.insert(node.id, node), None);
            None
        }
    }
    pub fn insert_or_update(&mut self, node: Node<T>) -> Option<Node<T>> {
        self.nodes.insert(node.id, node)
    }
    pub fn contains_id(&self, id: &T) -> bool {
        self.nodes.contains_key(id)
    }
    pub fn get(&self, id: &T) -> Option<&Node<T>> {
        self.nodes.get(id)
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
    
}