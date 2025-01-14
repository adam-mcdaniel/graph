use std::path::Path;
use graph::{Graph, ID};

fn main() {
    build_family_tree();
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