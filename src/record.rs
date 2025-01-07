use std::{ops::{Index, IndexMut}, fmt::{Display, Formatter, Result as FmtResult}};

use super::{Node, Edge, Properties, ID, Value};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Record {
    id: ID,
    nodes: Grid,
}

impl Into<ID> for Record {
    fn into(self) -> ID {
        self.id
    }
}

impl Into<ID> for &Record {
    fn into(self) -> ID {
        self.id
    }
}

/// A grid which can have nodes placed on it
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid {
    width: usize,
    height: usize,
    nodes: Vec<Column>
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            width,
            height,
            nodes: vec![Column::new(height); width]
        }
    }

    pub fn get_column(&self, x: usize) -> Option<&Column> {
        self.nodes.get(x)
    }

    pub fn insert_node(&mut self, x: usize, y: usize, node: impl Into<Node>, label: impl ToString) {
        if let Some(column) = self.nodes.get_mut(x) {
            column.insert_node(y, node, label);
        }
    }
}

/// A column in a grid
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Column {
    nodes: Vec<Option<(Node, String)>>
}

impl Column {
    pub fn new(length: usize) -> Self {
        Column {
            nodes: vec![None; length]
        }
    }

    pub fn insert_node(&mut self, y: usize, node: impl Into<Node>, label: impl ToString) {
        self.nodes.insert(y, Some((node.into(), label.to_string())));
    }
}

impl Display for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.nodes.len() > 1 {
            write!(f, "{{")?;
        }
        for (y, maybe_node_label) in self.nodes.iter().enumerate() {
            if let Some((node, label)) = maybe_node_label {
                if y > 0 {
                    write!(f, "|")?;
                }
                write!(f, "<{}>{}", node, label)?;
            }
        }
        if self.nodes.len() > 1 {
            write!(f, "}}")?;
        }
        Ok(())
    }
}

impl Record {
    pub fn new(id: impl Into<ID>, width: usize, height: usize) -> Self {
        Record {
            id: id.into(),
            nodes: Grid::new(width, height)
        }
    }

    pub fn id(&self) -> ID {
        self.id
    }

    pub fn nodes(&self) -> &Grid {
        &self.nodes
    }

    pub fn nodes_mut(&mut self) -> &mut Grid {
        &mut self.nodes
    }

    pub fn insert_node(&mut self, x: usize, y: usize, node: impl Into<Node>, label: impl ToString) {
        self.nodes.insert_node(x, y, node.into().with_record(self.id()), label);
    }

    pub fn name(&self) -> String {
        format!("Record_{}", self.id)
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Record_{} [shape=record, label=\" ", self.id)?;
        for (x, column) in self.nodes.nodes.iter().enumerate() {
            if x > 0 {
                write!(f, "|")?;
            }
            if column.nodes.is_empty() {
                write!(f, " ")?;
            } else {
                write!(f, "{}", column)?;
            }
        }
        write!(f, " \"]")
    }
}


impl Index<usize> for Grid {
    type Output = Column;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}

impl Index<usize> for Column {
    type Output = Node;

    fn index(&self, index: usize) -> &Self::Output {
        self.nodes[index].as_ref().map(|(node, _)| node).unwrap()
    }
}

impl Index<usize> for Record {
    type Output = Column;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

impl IndexMut<usize> for Record {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}