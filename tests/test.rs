#[cfg(test)]
mod tests {
    use stbparser::stb_common::StbCommon;
    use stbparser::*;

    #[test]
    fn it_works() {
        let filename = "data/steel_standard_model_utf8.stb";

        let mut contents = get_contents(filename);

        let document = roxmltree::Document::parse(&contents).unwrap();

        let root_node = document.root_element();

        for attribute in root_node.attributes() {
            println!("ST-Bridge {}: {}", attribute.name(), attribute.value());
        }

        let stb_common_node = extract_node("StbCommon", root_node);
        let stb_model_node = extract_node("StbModel", root_node);
        let stb_extensions_node = extract_node("StbExtensions", root_node);

        let stb_common = StbCommon::new();

        println!("{:?}", stb_common_node);
        println!("{:?}", stb_model_node);
        println!("{:?}", stb_extensions_node);
    }
}
