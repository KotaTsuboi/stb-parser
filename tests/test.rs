#[cfg(test)]
mod tests {
    use stbparser::st_bridge::stb_common::StbCommon;
    use stbparser::st_bridge::stb_common::StbReinforcementStrength;
    use stbparser::*;

    #[test]
    fn it_works() {
        let filename = "data/steel_standard_model_utf8.stb";

        let contents = get_contents(filename);

        let document = roxmltree::Document::parse(&contents).unwrap();

        let root_node = document.root_element();

        for attribute in root_node.attributes() {
            println!("ST-Bridge {}: {}", attribute.name(), attribute.value());
        }

        let stb_common_node = extract_node("StbCommon", root_node);
        let stb_reinforcement_strength_list =
            extract_node("StbReinforcement_Strength_List", stb_common_node);
        let mut stb_common = StbCommon::new();

        for node in stb_reinforcement_strength_list
            .children()
            .filter(|n| n.is_element())
        {
            let d = node.attribute("D").unwrap();
            let sd = node.attribute("SD").unwrap();
            stb_common.push(StbReinforcementStrength { d, sd });
        }

        println!("stb_common: {:?}", stb_common);

        let stb_model_node = extract_node("StbModel", root_node);
        let stb_extensions_node = extract_node("StbExtensions", root_node);
    }
}
