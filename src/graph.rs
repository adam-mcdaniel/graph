use std::{collections::HashMap, fmt::{Display, Formatter, Result as FmtResult}, path::Path};
use super::{Node, Edge, Record, Properties, ID, Value};

pub struct Graph {
    properties: Properties,
    nodes: HashMap<ID, Node>,
    edges: HashMap<ID, Edge>,
    records: HashMap<ID, Record>,
    node_properties: HashMap<ID, Properties>,
    edge_properties: HashMap<ID, Properties>,
    record_properties: HashMap<ID, Properties>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            properties: Properties::new(),
            nodes: HashMap::new(),
            edges: HashMap::new(),
            records: HashMap::new(),
            node_properties: HashMap::new(),
            edge_properties: HashMap::new(),
            record_properties: HashMap::new(),
        }
    }

    pub fn with_property(mut self, key: &str, value: impl Into<Value>) -> Self {
        self.properties.set(key.to_string(), value);
        self
    }

    pub fn new_node(&mut self, id: impl Into<ID>) -> NodeBuilder {
        NodeBuilder::new(id, self)
    }

    pub fn insert_node(&mut self, node: Node, properties: Properties) {
        self.nodes.insert(node.id(), node);
        self.node_properties.insert(node.id(), properties);
    }

    pub fn insert_edge(&mut self, edge: Edge, properties: Properties) {
        self.edges.insert(edge.id(), edge);
        self.edge_properties.insert(edge.id(), properties);
    }

    pub fn new_edge(&mut self, start_node: impl Into<Node>, end_node: impl Into<Node>) -> EdgeBuilder {
        EdgeBuilder::new(start_node, end_node, self)
    }

    pub fn new_record(&mut self, id: impl Into<ID>, width: usize, height: usize) -> RecordBuilder {
        RecordBuilder::new(id, width, height, self)
    }

    pub fn insert_record(&mut self, record: Record, properties: Properties) {
        let id = record.id();
        self.records.insert(id, record);
        self.record_properties.insert(id, properties);
    }

    pub fn to_dot(&self) -> String {
        let mut dot = String::new();
        dot.push_str("digraph {\n");
        // Graph properties
        dot.push_str(&format!("  graph {};\n", self.properties));
        for (id, record) in &self.records {
            let properties = self.record_properties.get(id).unwrap();
            dot.push_str(&format!("  {} {};\n", record, properties));
        }
        for (id, node) in &self.nodes {
            let properties = self.node_properties.get(id).unwrap();
            dot.push_str(&format!("  {} {};\n", node, properties));
        }
        for (id, edge) in &self.edges {
            let properties = self.edge_properties.get(id).unwrap();
            dot.push_str(&format!("  {} {};\n", edge, properties));
        }
        dot.push_str("}\n");
        dot
    }

    pub fn build_png(&self, path: &Path) {
        let dot = self.to_dot();
        let dot_path = path.with_extension("dot");
        std::fs::write(&dot_path, dot).unwrap();
        let output = std::process::Command::new("dot")
            .arg("-Tpng")
            .arg(dot_path)
            .arg("-o")
            .arg(path)
            .output()
            .unwrap();
        assert!(output.status.success());
    }

    pub fn build_svg(&self, path: &Path) {
        let dot = self.to_dot();
        let dot_path = path.with_extension("dot");
        std::fs::write(&dot_path, dot).unwrap();
        let output = std::process::Command::new("dot")
            .arg("-Tsvg")
            .arg(dot_path)
            .arg("-o")
            .arg(path)
            .output()
            .unwrap();
        assert!(output.status.success());
    }

    pub fn build_circuit_diagram(&self, path: &Path) {
        let dot = self.to_dot();
        let dot_path = path.with_extension("dot");
        std::fs::write(&dot_path, dot).unwrap();
        let output = std::process::Command::new("neato")
            .arg("-Tsvg")
            .arg(dot_path)
            .arg("-o")
            .arg(path)
            .output()
            .unwrap();
        assert!(output.status.success());
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // write!(f, "Graph {{\n")?;
        // write!(f, "  nodes: [\n")?;
        // for (id, node) in &self.nodes {
        //     let properties = self.node_properties.get(id).unwrap();
        //     write!(f, "    {} {},\n", node, properties)?;
        // }
        // write!(f, "  ],\n")?;
        // write!(f, "  edges: [\n")?;
        // for (id, edge) in &self.edges {
        //     let properties = self.edge_properties.get(id).unwrap();
        //     write!(f, "    {} {},\n", edge, properties)?;
        // }
        // write!(f, "  ],\n")?;
        // write!(f, "  records: [\n")?;
        // for (id, record) in &self.records {
        //     write!(f, "    {},\n", record)?;
        // }
        // write!(f, "  ]\n")?;
        // write!(f, "}}")

        write!(f, "{} edges, {} nodes, {} records", self.edges.len(), self.nodes.len(), self.records.len())
    }
}

pub struct NodeBuilder<'a> {
    graph: &'a mut Graph,
    id: ID,
    properties: Properties,
}

impl<'a> NodeBuilder<'a> {
    pub fn new(id: impl Into<ID>, graph: &'a mut Graph) -> Self {
        NodeBuilder {
            graph,
            id: id.into(),
            properties: Properties::new(),
        }
    }

    pub fn with_property(mut self, key: &str, value: impl Into<Value>) -> Self {
        self.properties.set(key.to_string(), value);
        self
    }

    pub fn with_properties(mut self, mut properties: Properties) -> Self {
        self.properties.append(&mut properties);
        self
    }

    pub fn finalize(self) -> Node {
        let node = Node::new(self.id);
        self.graph.insert_node(node, self.properties);
        node
    }
}

pub struct EdgeBuilder<'a> {
    graph: &'a mut Graph,
    id: ID,
    start_node: Node,
    end_node: Node,
    properties: Properties,
}

impl<'a> EdgeBuilder<'a> {
    pub fn new(start_node: impl Into<Node>, end_node: impl Into<Node>, graph: &'a mut Graph) -> Self {
        let start_node = start_node.into();
        let end_node = end_node.into();
        EdgeBuilder {
            graph,
            id: start_node.id() + end_node.id(),
            start_node: start_node.into(),
            end_node: end_node.into(),
            properties: Properties::new(),
        }
    }

    pub fn with_property(mut self, key: &str, value: impl Into<Value>) -> Self {
        self.properties.set(key.to_string(), value);
        self
    }

    pub fn with_properties(mut self, mut properties: Properties) -> Self {
        self.properties.append(&mut properties);
        self
    }
    
    pub fn finalize(self) -> Edge {
        let edge = Edge::new(self.id, self.start_node, self.end_node);
        self.graph.insert_edge(edge, self.properties);
        edge
    }
}


pub struct RecordBuilder<'a> {
    graph: &'a mut Graph,
    record: Record,
    properties: Properties,
}

impl<'a> RecordBuilder<'a> {
    pub fn new(id: impl Into<ID>, width: usize, height: usize, graph: &'a mut Graph) -> Self {
        let record = Record::new(id, width, height);
        RecordBuilder {
            graph,
            record,
            properties: Properties::new(),
        }
    }

    pub fn insert(mut self, x: usize, y: usize, label: impl ToString) -> Self {
        self.record.insert_node(x, y, Node::new(ID::from(x) + ID::from(y) + ID::from(label.to_string())), label);
        self
    }

    pub fn with_property(mut self, key: &str, value: impl Into<Value>) -> Self {
        self.properties.set(key.to_string(), value);
        self
    }

    pub fn with_properties(mut self, mut properties: Properties) -> Self {
        self.properties.append(&mut properties);
        self
    }

    pub fn finalize(self) -> Record {
        self.graph.insert_record(self.record.clone(), self.properties);
        self.record
    }
}