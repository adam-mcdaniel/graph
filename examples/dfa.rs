use std::path::Path;
use graph::{Graph, ID};

fn main() {
    dfa_diagram();
}


fn dfa_diagram() {
    let mut g = Graph::new()
        .with_property("rankdir", "LR");

    let q0 = g.new_node("q0")
        .with_property("label", "q0 (start)")
        .with_property("style", "filled")
        .with_property("fillcolor", "lightgreen")
        .finalize();

    let q1 = g.new_node("q1")
        .with_property("label", "q1")
        .finalize();

    let q2 = g.new_node("q2")
        .with_property("label", "q2")
        .with_property("shape", "doublecircle")
        .with_property("style", "filled")
        .with_property("fillcolor", "lightblue")
        .finalize();
    
    g.new_edge(q0, q1)
        .with_property("label", "1")
        .finalize();

    g.new_edge(q1, q1)
        .with_property("label", "1")
        .finalize();

    g.new_edge(q1, q2)
        .with_property("label", "0")
        .finalize();

    g.new_edge(q2, q2)
        .with_property("label", "0")
        .finalize();

    // Build the SVG
    g.save_svg(&Path::new("dfa_diagram.svg"));
}
