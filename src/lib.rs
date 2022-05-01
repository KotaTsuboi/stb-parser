use std::fs::File;
use std::io::prelude::*;

fn get_contents(filename: &str) -> String {
    let mut f = File::open(filename).expect("File not found.");

    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    return contents;
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let filename = "data/steel_standard_model_utf8.stb";

        let mut contents = crate::get_contents(filename);

        let document = roxmltree::Document::parse(&contents).unwrap();

        let root_node = document.root_element();

        match root_node.node_type() {
            roxmltree::NodeType::Element => {}
            _ => {
                panic!("This node is not an element.");
            }
        }

        for attribute in root_node.attributes() {
            println!("ST-Bridge {}: {}", attribute.name(), attribute.value());
        }

        let child_elements = root_node
            .children()
            .filter(|n| n.node_type() == roxmltree::NodeType::Element);

        for node in child_elements {
            let tag_name = node.tag_name().name();
            if tag_name == "StbCommon" {
                println!("StbCommon found.");
            } else if tag_name == "StbModel" {
                println!("StbModel found.");
            } else if tag_name == "StbExtensions" {
                println!("StbExtensions found.");
            }
        }
    }
}
