use std::fmt::{Display, Formatter, Result as FmtResult};
use super::{Properties, ID};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Node {
    record: Option<ID>,
    id: ID,
}

impl Node {
    pub fn new(id: ID) -> Self {
        Node { record: None, id }
    }

    pub fn with_record(mut self, record: impl Into<ID>) -> Self {
        self.record = Some(record.into());
        self
    }

    pub fn id(&self) -> ID {
        self.id
    }

    pub fn name(&self) -> String {
        if let Some(record) = self.record {
            format!("Record_{}:Node_{}", record, self.id)
        } else {
            format!("Node_{}", self.id)
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(record) = self.record {
            write!(f, "Record_{}:Node_{}", record, self.id)
        } else {
            write!(f, "Node_{}", self.id)
        }
    }
}

impl From<ID> for Node {
    fn from(id: ID) -> Self {
        Node::new(id)
    }
}

impl From<Node> for ID {
    fn from(node: Node) -> Self {
        node.id()
    }
}