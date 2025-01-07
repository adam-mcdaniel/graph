use super::{Properties, ID, Node};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Edge {
    id: ID,
    start_node: Node,
    end_node: Node,
}

impl Edge {
    pub fn new(id: impl Into<ID>, start_node: impl Into<Node>, end_node: impl Into<Node>) -> Self {
        Edge {
            id: id.into(),
            start_node: start_node.into(),
            end_node: end_node.into(),
        }
    }

    pub fn id(&self) -> ID {
        self.id
    }

    pub fn start_node(&self) -> Node {
        self.start_node
    }

    pub fn end_node(&self) -> Node {
        self.end_node
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} -> {}", self.start_node, self.end_node)
    }
}

impl From<Edge> for ID {
    fn from(edge: Edge) -> Self {
        edge.id
    }
}