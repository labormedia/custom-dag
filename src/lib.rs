use std::collections::HashSet;
use core::hash::Hash;

type BlockId = u32;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node<T: Eq + Hash + PartialEq> {
    id: T,
    left: Option<T>,
    right: Option<T>,
}

impl<T: Eq + Hash + PartialEq> Node<T> {
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
pub struct Dag<T: Eq + Hash + PartialEq> {
    nodes: HashSet<Node<T>>,
}

impl<T: Eq + Hash + PartialEq> Dag<T> {
    pub fn new() -> Self {
        Dag {
            nodes: HashSet::new()
        }
    }
    pub fn insert(&mut self, node: Node<T>) {
        self.nodes.insert(node);
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