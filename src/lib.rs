use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

use crate::st_bridge::stb_common::StbCommon;
use crate::st_bridge::stb_model::stb_axes_and_stories::StbAxes;
use crate::st_bridge::stb_model::stb_axes_and_stories::StbNodeId;
use crate::st_bridge::stb_model::stb_axes_and_stories::StbNodeIdList;
use crate::st_bridge::stb_model::stb_axes_and_stories::StbStories;
use crate::st_bridge::stb_model::stb_axes_and_stories::StbStory;
use crate::st_bridge::stb_model::stb_axes_and_stories::StbStoryKind;
use crate::st_bridge::stb_model::stb_axes_and_stories::StbXAxis;
use crate::st_bridge::stb_model::stb_axes_and_stories::StbYAxis;
use crate::st_bridge::stb_model::stb_members::StbBeam;
use crate::st_bridge::stb_model::stb_members::StbBeams;
use crate::st_bridge::stb_model::stb_members::StbBrace;
use crate::st_bridge::stb_model::stb_members::StbBraces;
use crate::st_bridge::stb_model::stb_members::StbColumn;
use crate::st_bridge::stb_model::stb_members::StbColumns;
use crate::st_bridge::stb_model::stb_members::StbGirder;
use crate::st_bridge::stb_model::stb_members::StbGirders;
use crate::st_bridge::stb_model::stb_members::StbMembers;
use crate::st_bridge::stb_model::stb_members::StbPost;
use crate::st_bridge::stb_model::stb_members::StbPosts;
use crate::st_bridge::stb_model::stb_members::StbSlab;
use crate::st_bridge::stb_model::stb_members::StbSlabs;
use crate::st_bridge::stb_model::stb_nodes::StbNode;
use crate::st_bridge::stb_model::stb_nodes::StbNodeKind;
use crate::st_bridge::stb_model::stb_nodes::StbNodes;
use crate::st_bridge::stb_model::stb_sections::StbSections;
use crate::st_bridge::stb_model::stb_sections::StbSectionsChildren;
use crate::st_bridge::stb_model::StbModel;

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

pub fn extract_stb_common<'a>(root_node: roxmltree::Node<'a, 'a>) -> StbCommon {
    let stb_common_node = extract_node("StbCommon", root_node);
    let stb_reinforcement_strength_list =
        extract_node("StbReinforcement_Strength_List", stb_common_node);
    let mut stb_common = StbCommon::new();

    for node in stb_reinforcement_strength_list
        .children()
        .filter(|n| n.is_element())
    {
        let d = node.attribute("D").unwrap().to_string();
        let sd = node.attribute("SD").unwrap().to_string();
        stb_common.stb_reinforcement_strength_list.insert(d, sd);
    }

    return stb_common;
}

pub fn extract_stb_model(root_node: roxmltree::Node) {
    let stb_model_node = extract_node("StbModel", root_node);

    let stb_nodes = extract_stb_nodes(stb_model_node);
    println!("stb_nodes: {:?}", stb_nodes);

    let stb_axes = extract_stb_axes(stb_model_node);
    println!("stb_axes: {:?}", stb_axes);

    let stb_stories = extract_stb_stories(stb_model_node);
    println!("stb_stories: {:?}", stb_stories);

    let stb_members = extract_stb_members(stb_model_node);
    println!("stb_members: {:?}", stb_members);
    //let stb_sections = extract_stb_sections(stb_model_node);

    /*
    StbModel {
        stb_nodes,
        stb_axes,
        stb_stories,
        stb_members,
        stb_sections,
    }
    */
}

fn extract_stb_nodes(stb_model_node: roxmltree::Node) -> StbNodes {
    let stb_nodes_node = extract_node("StbNodes", stb_model_node);

    let mut stb_nodes = StbNodes::new();

    for node in stb_nodes_node.children().filter(|n| n.is_element()) {
        let id = node.attribute("id").unwrap().parse::<i32>().unwrap();
        let x = node.attribute("x").unwrap().parse::<f64>().unwrap();
        let y = node.attribute("y").unwrap().parse::<f64>().unwrap();
        let z = node.attribute("z").unwrap().parse::<f64>().unwrap();
        let kind = StbNodeKind::from_str(node.attribute("kind").unwrap()).unwrap();
        let id_member = match node.attribute("id_member") {
            Some(s) => Some(s.parse::<i32>().unwrap()),
            None => None,
        };

        stb_nodes.stb_node_list.push(StbNode {
            id,
            x,
            y,
            z,
            kind,
            id_member,
        });
    }

    stb_nodes
}

fn extract_stb_axes(stb_model_node: roxmltree::Node) -> StbAxes {
    let stb_axes_node = extract_node("StbAxes", stb_model_node);

    let mut stb_axes = StbAxes::new();

    for node in stb_axes_node.children().filter(|n| n.is_element()) {
        let stb_node_id_list_node = extract_node("StbNodeid_List", node);

        let mut stb_node_id_list = Vec::new();

        for children in stb_node_id_list_node.children().filter(|n| n.is_element()) {
            let id = children.attribute("id").unwrap().parse::<i32>().unwrap();
            stb_node_id_list.push(StbNodeId { id });
        }

        match node.tag_name().name() {
            "StbX_Axis" => {
                stb_axes.stb_x_axis_list.push(StbXAxis {
                    id: node.attribute("id").unwrap().parse::<i32>().unwrap(),
                    name: node.attribute("name").unwrap().to_string(),
                    distance: node.attribute("distance").unwrap().parse::<f64>().unwrap(),
                    stb_node_id_list: StbNodeIdList {
                        children: stb_node_id_list,
                    },
                });
            }
            "StbY_Axis" => {
                stb_axes.stb_y_axis_list.push(StbYAxis {
                    id: node.attribute("id").unwrap().parse::<i32>().unwrap(),
                    name: node.attribute("name").unwrap().to_string(),
                    distance: node.attribute("distance").unwrap().parse::<f64>().unwrap(),
                    stb_node_id_list: StbNodeIdList {
                        children: stb_node_id_list,
                    },
                });
            }
            _ => {
                panic!("Tag name {} is invalid.", node.tag_name().name());
            }
        }
    }

    stb_axes
}

fn extract_stb_stories(stb_model_node: roxmltree::Node) -> StbStories {
    let stb_stories_node = extract_node("StbStories", stb_model_node);

    let mut stb_stories = StbStories::new();

    for node in stb_stories_node.children().filter(|n| n.is_element()) {
        let stb_node_id_list_node = extract_node("StbNodeid_List", node);

        let mut stb_node_id_list = Vec::new();

        for children in stb_node_id_list_node.children().filter(|n| n.is_element()) {
            let id = children.attribute("id").unwrap().parse::<i32>().unwrap();
            stb_node_id_list.push(StbNodeId { id });
        }

        stb_stories.stb_story_list.push(StbStory {
            id: node.attribute("id").unwrap().parse::<i32>().unwrap(),
            name: node.attribute("name").unwrap().to_string(),
            height: node.attribute("height").unwrap().parse::<f64>().unwrap(),
            kind: StbStoryKind::from_str(node.attribute("kind").unwrap()).unwrap(),
            concrete_strength: node.attribute("concrete_strength").unwrap().to_string(),
            stb_node_id_list: StbNodeIdList {
                children: stb_node_id_list,
            },
        });
    }

    stb_stories
}

fn extract_stb_members(stb_model_node: roxmltree::Node) -> StbMembers {
    let stb_members_node = extract_node("StbMembers", stb_model_node);

    let stb_columns = extract_stb_columns(stb_members_node);
    let stb_posts = extract_stb_posts(stb_members_node);
    let stb_girders = extract_stb_girders(stb_members_node);
    let stb_beams = extract_stb_beams(stb_members_node);
    let stb_braces = extract_stb_braces(stb_members_node);
    let stb_slabs = extract_stb_slabs(stb_members_node);

    StbMembers {
        stb_columns,
        stb_posts,
        stb_girders,
        stb_beams,
        stb_braces,
        stb_slabs,
    }
}

fn parse_attribute<T: FromStr>(
    name: &str,
    node: roxmltree::Node,
) -> Result<T, <T as FromStr>::Err> {
    node.attribute(name).unwrap().to_lowercase().parse::<T>()
}

fn parse_enum_attribute<T: FromStr>(
    name: &str,
    node: roxmltree::Node,
) -> Result<T, <T as FromStr>::Err> {
    T::from_str(node.attribute(name).unwrap())
}

fn extract_stb_columns(stb_members_node: roxmltree::Node) -> StbColumns {
    let stb_columns_node = extract_node("StbColumns", stb_members_node);

    let mut stb_column_list: Vec<StbColumn> = Vec::new();

    for node in stb_columns_node.children().filter(|n| n.is_element()) {
        stb_column_list.push(StbColumn {
            id: parse_attribute("id", node).unwrap(),
            name: node.attribute("name").unwrap().to_string(),
            id_node_bottom: parse_attribute("idNode_bottom", node).unwrap(),
            id_node_top: parse_attribute("idNode_top", node).unwrap(),
            rotate: parse_attribute("rotate", node).unwrap(),
            id_section: parse_attribute("id_section", node).unwrap(),
            kind_structure: parse_enum_attribute::<ColumnStructureKind>("kind_structure", node)
                .unwrap(),
            offset_x: parse_attribute("offset_X", node).unwrap(),
            offset_y: parse_attribute("offset_Y", node).unwrap(),
            condition_bottom: parse_enum_attribute("condition_bottom", node).unwrap(),
            condition_top: parse_enum_attribute("condition_top", node).unwrap(),
        });
    }

    StbColumns { stb_column_list }
}

fn extract_stb_posts(stb_members_node: roxmltree::Node) -> StbPosts {
    let stb_posts_node = extract_node("StbPosts", stb_members_node);

    let mut stb_post_list: Vec<StbPost> = Vec::new();

    for node in stb_posts_node.children().filter(|n| n.is_element()) {
        stb_post_list.push(StbPost {
            id: parse_attribute("id", node).unwrap(),
            name: node.attribute("name").unwrap().to_string(),
            id_node_bottom: parse_attribute("idNode_bottom", node).unwrap(),
            id_node_top: parse_attribute("idNode_top", node).unwrap(),
            rotate: parse_attribute("rotate", node).unwrap(),
            id_section: parse_attribute("id_section", node).unwrap(),
            kind_structure: parse_enum_attribute("kind_structure", node).unwrap(),
            offset_x: parse_attribute("offset_X", node).unwrap(),
            offset_y: parse_attribute("offset_Y", node).unwrap(),
            offset_bottom_x: parse_attribute("offset_bottom_X", node).unwrap(),
            offset_bottom_y: parse_attribute("offset_bottom_Y", node).unwrap(),
            offset_bottom_z: parse_attribute("offset_bottom_Z", node).unwrap(),
            offset_top_x: parse_attribute("offset_top_X", node).unwrap(),
            offset_top_y: parse_attribute("offset_top_Y", node).unwrap(),
            offset_top_z: parse_attribute("offset_top_Z", node).unwrap(),
            condition_bottom: parse_enum_attribute("condition_bottom", node).unwrap(),
            condition_top: parse_enum_attribute("condition_top", node).unwrap(),
        });
    }

    StbPosts { stb_post_list }
}

fn extract_stb_girders(stb_members_node: roxmltree::Node) -> StbGirders {
    let stb_girders_node = extract_node("StbGirders", stb_members_node);

    let mut stb_girder_list: Vec<StbGirder> = Vec::new();

    for node in stb_girders_node.children().filter(|n| n.is_element()) {
        stb_girder_list.push(StbGirder {
            id: parse_attribute("id", node).unwrap(),
            name: node.attribute("name").unwrap().to_string(),
            id_node_start: parse_attribute("idNode_start", node).unwrap(),
            id_node_end: parse_attribute("idNode_end", node).unwrap(),
            rotate: parse_attribute("rotate", node).unwrap(),
            id_section: parse_attribute("id_section", node).unwrap(),
            kind_structure: parse_enum_attribute("kind_structure", node).unwrap(),
            is_foundation: parse_attribute("isFoundation", node).unwrap(),
            offset: parse_attribute("offset", node).unwrap(),
            level: parse_attribute("level", node).unwrap(),
            type_haunch_h: match node.attribute("type_haunch_H") {
                Some(s) => Some(HaunchType::from_str(s).unwrap()),
                None => None,
            },
        });
    }

    StbGirders { stb_girder_list }
}

fn extract_stb_beams(stb_members_node: roxmltree::Node) -> StbBeams {
    let stb_beams_node = extract_node("StbBeams", stb_members_node);

    let mut stb_beam_list: Vec<StbBeam> = Vec::new();

    for node in stb_beams_node.children().filter(|n| n.is_element()) {
        stb_beam_list.push(StbBeam {
            id: parse_attribute("id", node).unwrap(),
            name: node.attribute("name").unwrap().to_string(),
            id_node_start: parse_attribute("idNode_start", node).unwrap(),
            id_node_end: parse_attribute("idNode_end", node).unwrap(),
            rotate: parse_attribute("rotate", node).unwrap(),
            id_section: parse_attribute("id_section", node).unwrap(),
            kind_structure: parse_enum_attribute("kind_structure", node).unwrap(),
            is_foundation: parse_attribute("isFoundation", node).unwrap(),
            offset: parse_attribute("offset", node).unwrap(),
            level: parse_attribute("level", node).unwrap(),
        });
    }

    StbBeams { stb_beam_list }
}

fn extract_stb_braces(stb_members_node: roxmltree::Node) -> StbBraces {
    let stb_braces_node = extract_node("StbBraces", stb_members_node);

    let mut stb_brace_list: Vec<StbBrace> = Vec::new();

    for node in stb_braces_node.children().filter(|n| n.is_element()) {
        stb_brace_list.push(StbBrace {
            id: parse_attribute("id", node).unwrap(),
            name: node.attribute("name").unwrap().to_string(),
            id_node_start: parse_attribute("idNode_start", node).unwrap(),
            id_node_end: parse_attribute("idNode_end", node).unwrap(),
            rotate: parse_attribute("rotate", node).unwrap(),
            id_section: parse_attribute("id_section", node).unwrap(),
            kind_structure: parse_enum_attribute("kind_structure", node).unwrap(),
            offset_start_x: parse_attribute("offset_start_X", node).unwrap(),
            offset_start_y: parse_attribute("offset_start_Y", node).unwrap(),
            offset_start_z: parse_attribute("offset_start_Z", node).unwrap(),
            offset_end_x: parse_attribute("offset_end_X", node).unwrap(),
            offset_end_y: parse_attribute("offset_end_Y", node).unwrap(),
            offset_end_z: parse_attribute("offset_end_Z", node).unwrap(),
            condition_start: parse_enum_attribute("condition_start", node).unwrap(),
            condition_end: parse_enum_attribute("condition_end", node).unwrap(),
        });
    }

    StbBraces { stb_brace_list }
}

fn extract_stb_slabs(stb_members_node: roxmltree::Node) -> StbSlabs {
    let stb_slabs_node = extract_node("StbSlabs", stb_members_node);

    let mut stb_slab_list: Vec<StbSlab> = Vec::new();

    for node in stb_slabs_node.children().filter(|n| n.is_element()) {
        stb_slab_list.push(StbSlab {
            id: parse_attribute("id", node).unwrap(),
            name: node.attribute("name").unwrap().to_string(),
            id_section: parse_attribute("id_section", node).unwrap(),
            kind_structure: parse_enum_attribute("kind_structure", node).unwrap(),
            kind_slab: parse_enum_attribute("kind_slab", node).unwrap(),
            level: parse_attribute("level", node).unwrap(),
            is_foundation: parse_attribute("isFoundation", node).unwrap(),
        });
    }

    StbSlabs { stb_slab_list }
}

fn extract_stb_sections(stb_model_node: roxmltree::Node) -> StbSections {
    let stb_sections_node = extract_node("StbSections", stb_model_node);

    let stb_sections = StbSections::new();

    for node in stb_sections_node.children().filter(|n| n.is_element()) {
        let children: Box<dyn StbSectionsChildren>;

        match node.tag_name().name() {
            "StbSecColumn_RC" => panic_unimplemented(node.tag_name().name()),
            "StbSecColumn_S" => children = Box::new(extract_stb_sec_column_s), //
            "StbSecColumn_SRC" => panic_unimplemented(node.tag_name().name()),
            "StbSecColumn_CFT" => panic_unimplemented(node.tag_name().name()),
            "StbSecBeam_RC" => {} //
            "StbSecBeam_S" => {}  //
            "StbSecBeam_SRC" => panic_unimplemented(node.tag_name().name()),
            "StbSecBrace_S" => {} //
            "StbSecSlab_RC" => {} //
            "StbSecSlabDeck" => panic_unimplemented(node.tag_name().name()),
            "StbSecSlabPrecast" => panic_unimplemented(node.tag_name().name()),
            "StbSecWall_RC" => panic_unimplemented(node.tag_name().name()),
            "StbSecFoundation_RC" => panic_unimplemented(node.tag_name().name()),
            "StbSecPile_RC" => panic_unimplemented(node.tag_name().name()),
            "StbSecPile_S" => panic_unimplemented(node.tag_name().name()),
            "StbSecPileProduct" => panic_unimplemented(node.tag_name().name()),
            "StbSecOpen_RC" => panic_unimplemented(node.tag_name().name()),
            "StbSecParapet_RC" => panic_unimplemented(node.tag_name().name()),
            "StbSecSteel" => {} //
            "StbSecUndefined" => panic_unimplemented(node.tag_name().name()),
        };

        stb_sections.children.push(children);
    }

    stb_sections
}

fn extract_stb_sec_column_s(node: roxmltree::Node) -> StbSecColumnS {
    let stb_sec_steel_column_node = extract_node("StbSecSteelColumn", node);
    StbSecColumnS {
    id: parse_attribute("id").unwrap(),
    name: parse_attribute("name").unwrap(),
    floor: parse_attribute("floor").unwrap(),
    kind_column: parse_enum_attribute("kind_column").unwrap(),
    direction: parse_attribute("direction").unwrap(),
    base_type: parse_enum_attribute("base_type").unwrap(),
    stb_sec_steel_column: StbSecSteelColumn{
        stb_sec_steel_column_node
    pos: ,
    shape: String,
    strength_main: String,
    strength_web: String,
    },
    }
}

fn panic_unimplemented(what: &str) {
    panic!("{} is unimplemented!", what);
}
