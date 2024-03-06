use core::hash::Hash;
use crate::Node;

/// `CollidingNode<T>` is a struct similar and exchangeable to `Node<T>`. Whereas normal `Node<T>`
/// are defined to be equal if the id is equal, `CollingNode<T>` will be different if *any* field is different.
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

impl<T: Eq + Hash + PartialEq + Copy> CollidingNode<T> {
    pub fn has_same_fields_to(&self, node: &Node<T>) -> bool {
        self.id == node.id
        &&
        self.left == node.left
        &&
        self.right == node.right
    }
}