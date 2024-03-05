use core::hash::Hash;
use crate::Node;

// For comparison purposes, colliding nodes are marked with a different type than normal nodes because normal nodes
// are defined to be equal if the id is equal, whereas colliding nodes will be different if *any* field is different.
// This feature will make the comparison logic more suitable for colliding analysis.
#[derive(Debug, Clone, Hash, Copy, PartialEq, Eq)]
pub struct CollidingNode<T: Eq + Hash + PartialEq + Copy> {
    pub id: T,
    pub left: Option<T>,
    pub right: Option<T>,
}

impl<T: Eq + Hash + PartialEq + Copy> From<Node<T>> for CollidingNode<T> {
    fn from(node: Node<T>) -> Self  {
        CollidingNode {
            id: node.id,
            left: node.left,
            right: node.right,
        }
    }
}