use std::path::Path;
use graph::{Graph, ID};

fn main() {
    // let mut id = ID::from("Adam");
    // id += ID::from("McDaniel");
    // println!("{:X}", id);
    // println!("{:X}", id.next_id());


    let mut g = Graph::new()
        .with_property("rankdir", "LR");
        // .with_property("splines", "ortho");
    let record = g.new_record("Bar", 2, 6)
        .insert(0, 0, "Bar")
        .insert(1, 0, "Pin 1")
        .insert(1, 1, "Pin 2")
        .insert(1, 2, "Pin 3")
        .finalize();

    let a = g.new_node("A")
        .with_property("label", "A")
        .finalize();

    for y in 0..3 {
        g.new_edge(record[1][y], a)
            .with_property("label", format!("Pin {} -> A", y + 1))
            .finalize();
    }

    g.build_svg(&Path::new("output.svg"));

    build_family_tree();
    cpu_6502_diagram();
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
    g.build_svg(&Path::new("dfa_diagram.svg"));
}

fn cpu_6502_diagram() {
    let mut g = Graph::new()
        .with_property("rankdir", "TB")
        .with_property("nodesep", "0.5")
        .with_property("layout", "fdp");

    let cpu = g.new_record("CPU_6502", 5, 20)
        .insert(2, 0, "6502\\nCPU")
        .insert(1, 0, "1")
        .insert(1, 1, "2")
        .insert(1, 2, "3")
        .insert(1, 3, "4")
        .insert(1, 4, "5")
        .insert(1, 5, "6")
        .insert(1, 6, "7")
        .insert(1, 7, "8")
        .insert(1, 8, "9")
        .insert(1, 9, "10")
        .insert(1, 10, "11")
        .insert(1, 11, "12")
        .insert(1, 12, "13")
        .insert(1, 13, "14")
        .insert(1, 14, "15")
        .insert(1, 15, "16")
        .insert(1, 16, "17")
        .insert(1, 17, "18")
        .insert(1, 18, "19")
        .insert(1, 19, "20")

        .insert(0, 0, "VSS")
        .insert(0, 1, "RDY")
        .insert(0, 2, "1 (OUT)")
        .insert(0, 3, "IRQ")
        .insert(0, 4, "N.C.")
        .insert(0, 5, "NMI")
        .insert(0, 6, "SYNC")
        .insert(0, 7, "VCC")
        .insert(0, 8, "A0")
        .insert(0, 9, "A1")
        .insert(0, 10, "A2")
        .insert(0, 11, "A3")
        .insert(0, 12, "A4")
        .insert(0, 13, "A5")
        .insert(0, 14, "A6")
        .insert(0, 15, "A7")
        .insert(0, 16, "A8")
        .insert(0, 17, "A9")
        .insert(0, 18, "A10")
        .insert(0, 19, "A11")

        .insert(3, 0, "40")
        .insert(3, 1, "39")
        .insert(3, 2, "38")
        .insert(3, 3, "37")
        .insert(3, 4, "36")
        .insert(3, 5, "35")
        .insert(3, 6, "34")
        .insert(3, 7, "33")
        .insert(3, 8, "32")
        .insert(3, 9, "31")
        .insert(3, 10, "30")
        .insert(3, 11, "29")
        .insert(3, 12, "28")
        .insert(3, 13, "27")
        .insert(3, 14, "26")
        .insert(3, 15, "25")
        .insert(3, 16, "24")
        .insert(3, 17, "23")
        .insert(3, 18, "22")
        .insert(3, 19, "21")

        .insert(4, 0, "RES")
        .insert(4, 1, "2 (OUT)")
        .insert(4, 2, "S.O.")
        .insert(4, 3, "0 (IN)")
        .insert(4, 4, "N.C.")
        .insert(4, 5, "N.C.")
        .insert(4, 6, "R/W")
        .insert(4, 7, "D0")
        .insert(4, 8, "D1")
        .insert(4, 9, "D2")
        .insert(4, 10, "D3")
        .insert(4, 11, "D4")
        .insert(4, 12, "D5")
        .insert(4, 13, "D6")
        .insert(4, 14, "D7")
        .insert(4, 15, "A15")
        .insert(4, 16, "A14")
        .insert(4, 17, "A13")
        .insert(4, 18, "A12")
        .insert(4, 19, "VSS")
        .finalize();
    
    let a = g.new_node("Complex Wiring Stuff")
        .with_property("label", "Complex Wiring Stuff")
        .with_property("pos", "-6,0!")
        .finalize();
    let b = g.new_node("Other Complex Wiring Stuff")
        .with_property("label", "Other Complex Wiring Stuff")
        .with_property("pos", "6,0!")
        .finalize();

    for y in 0..20 {
        g.new_edge(cpu[4][y], b)
            .finalize();

        g.new_edge(a, cpu[0][y])
            .finalize();
    }

    g.build_svg(&Path::new("cpu_6502.svg"));
}


fn build_family_tree() {
    let mut g = Graph::new()
        .with_property("rankdir", "TB")
        .with_property("splines", "ortho")
        .with_property("nodesep", "1.25")
        .with_property("ranksep", "1.25");

    // Read the Tree.json file
    let tree = std::fs::read_to_string("assets/tree.json").unwrap();
    // Parse the JSON into a Value
    let tree: serde_json::Value = serde_json::from_str(&tree).unwrap();

    struct Person {
        id: ID,
        parent_id: Option<ID>,
        name: String,
        gender: String,
        alt_names: Vec<String>,
        category: String,
        type_: String,
    }

    // Read the JSON object into a Vec<Person>
    let mut people = Vec::new();
    // Theyre stored as an array in the "nodes" key
    for node in tree["nodes"].as_array().unwrap() {
        let id = ID::from(node["id"].as_i64().unwrap());
        let parent_id = node["parentId"].as_i64().map(ID::from);
        let name = node["name"].as_str().unwrap().to_string();
        let alt_names = if node["alt_names"].is_null() {
            Vec::new()
        } else {
            node["alt_names"].as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_string()).collect()
        };
        let category = node["category"].as_str().unwrap().to_string();
        let type_ = node["type"].as_str().unwrap_or("").to_string();
        let gender = node["gender"].as_str().unwrap().to_string();
        people.push(Person { id, gender, parent_id, name, alt_names, category, type_ });
    }

    // Create a node for each person
    for person in &people {
        let category = match person.category.as_str() {
            "priest" => "🙏",
            "king" => "👑",
            "nation" => "🌐",
            "patriarch" => "👨‍👧‍👦",
            "prophet" => "📖",
            _ => "👤",
        };
        let alt_names_label = if person.alt_names.len() > 0 {
            format!("({})", person.alt_names.join(", "))
        } else {
            "".to_string()
        };
        let label = format!("{} {} {}", person.name, category, alt_names_label);
    
        let node = g.new_node(person.id)
            .with_property("label", label)
            .with_property("style", "filled")
            .with_property("shape", "box")
            .with_property("fillcolor", if person.gender == "male" { "lightblue" } else { "pink" })
            .finalize();
    }

    // Create edges between parents and children
    for person in &people {
        if let Some(parent_id) = person.parent_id {
            g.new_edge(parent_id, person.id)
                .finalize();
        }
    }

    println!("{}", g);

    g.build_svg(&Path::new("family_tree.svg"));
}