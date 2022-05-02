#[cfg(test)]
mod tests {
    use stbparser::st_bridge::stb_common::StbCommon;
    use stbparser::st_bridge::stb_common::StbReinforcementStrength;
    use stbparser::*;

    static FILE_NAME: &str = "data/steel_standard_model_utf8.stb";

    #[test]
    fn it_works() {
        let contents = get_contents(FILE_NAME);

        let document = roxmltree::Document::parse(&contents).unwrap();

        let root_node = document.root_element();

        for attribute in root_node.attributes() {
            println!("ST-Bridge {}: {}", attribute.name(), attribute.value());
        }

        let stb_common = extract_stb_common(root_node);

        println!("stb_common: {:?}", stb_common);

        extract_stb_model(root_node);

        let stb_model_node = extract_node("StbModel", root_node);
        let stb_extensions_node = extract_node("StbExtensions", root_node);
    }

    #[test]
    fn extract_stb_common_test() {
        let contents = get_contents(FILE_NAME);

        let document = roxmltree::Document::parse(&contents).unwrap();

        let root_node = document.root_element();

        let stb_common = extract_stb_common(root_node);

        assert_eq!(
            stb_common
                .stb_reinforcement_strength_list
                .get("D10".to_string())
                .unwrap(),
            "SD295"
        );
        assert_eq!(
            stb_common
                .stb_reinforcement_strength_list
                .get("D19".to_string())
                .unwrap(),
            "SD345"
        );
    }
}
