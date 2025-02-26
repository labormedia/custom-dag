use core::hash::Hash;
use crate::Node;

/// `CollidingNode<T>` is a struct wrapper similar and exchangeable to `Node<T>`. Whereas normal `Node<T>`
/// are defined to be equal if the id is equal, `CollingNode<T>` will be different if *any* field of the node is different.
#[derive(Debug, Clone, Hash, Copy, PartialEq, Eq)]
pub struct CollidingNode<T: Eq + Hash + PartialEq + Copy, U: Eq + Hash + PartialEq + Copy>(pub Node<T, U>);

impl<T: Eq + Hash + PartialEq + Copy, U: Eq + Hash + PartialEq + Copy> From<Node<T, U>> for CollidingNode<T, U> {
    fn from(node: Node<T, U>) -> Self  {
        CollidingNode(node)
    }
}

impl<T: Eq + Hash + PartialEq + Copy, U: Eq + Hash + PartialEq + Copy> CollidingNode<T, U> {
    pub fn has_same_fields_to(&self, node: &Node<T, U>) -> bool {
        self.0.id == node.id
        &&
        self.0.left == node.left
        &&
        self.0.right == node.right
        &&
        self.0.payload == node.payload
    }
}