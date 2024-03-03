use std::collections::HashSet;

type BlockId = u32;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node {
    id: BlockId,
    left: Option<BlockId>,
    right: Option<BlockId>,
}

impl Node {
    // The Node itself can be self-referential and makes no assumptions about the structure of the graph.
    pub fn new(id: BlockId, left: Option<BlockId>, right: Option<BlockId>) -> Self {
        Node {
            id,
            left,
            right,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dag {
    nodes: HashSet<Node>,
}

impl Dag {
    pub fn new() -> Self {
        Dag {
            nodes: HashSet::new()
        }
    }
    pub fn insert(&mut self, node: Node) {
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
    fn create_node_zero() {
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
}