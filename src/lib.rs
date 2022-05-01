use std::fs::File;
use std::io::prelude::*;

pub mod st_bridge;

pub fn get_contents(filename: &str) -> String {
    let mut f = File::open(filename).expect("File not found.");

    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    return contents;
}

pub fn extract_node<'a>(name: &str, parent: roxmltree::Node<'a, '_>) -> roxmltree::Node<'a, 'a> {
    let child_elements = parent.children().filter(|n| n.is_element());

    for node in child_elements {
        let tag_name = node.tag_name().name();

        if tag_name == name {
            return node;
        }
    }

    panic!("Tag {} not found", name);
}
