use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

use crate::st_bridge::stb_common::StbCommon;
use crate::st_bridge::stb_extensions::StbExtension;
use crate::st_bridge::stb_extensions::StbExtensions;
use crate::st_bridge::stb_model::stb_axes_and_stories::StbAxes;
use crate::st_bridge::stb_model::stb_axes_and_stories::StbNodeId;
use crate::st_bridge::stb_model::stb_axes_and_stories::StbNodeIdList;
use crate::st_bridge::stb_model::stb_axes_and_stories::StbStories;
use crate::st_bridge::stb_model::stb_axes_and_stories::StbStory;
use crate::st_bridge::stb_model::stb_axes_and_stories::StbStoryKind;
use crate::st_bridge::stb_model::stb_axes_and_stories::StbXAxis;
use crate::st_bridge::stb_model::stb_axes_and_stories::StbYAxis;
use crate::st_bridge::stb_model::stb_members::HaunchType;
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
use crate::st_bridge::stb_model::stb_nodes::StbNodes;
use crate::st_bridge::stb_model::stb_sections::StbSec1WaySlab1;
use crate::st_bridge::stb_model::stb_sections::StbSecBarArrangementBeam;
use crate::st_bridge::stb_model::stb_sections::StbSecBarArrangementSlab;
use crate::st_bridge::stb_model::stb_sections::StbSecBeamRC;
use crate::st_bridge::stb_model::stb_sections::StbSecBeamS;
use crate::st_bridge::stb_model::stb_sections::StbSecBeamSameSection;
use crate::st_bridge::stb_model::stb_sections::StbSecBeamStartCenterEndSection;
use crate::st_bridge::stb_model::stb_sections::StbSecBraceS;
use crate::st_bridge::stb_model::stb_sections::StbSecBuildBox;
use crate::st_bridge::stb_model::stb_sections::StbSecBuildH;
use crate::st_bridge::stb_model::stb_sections::StbSecColumnS;
use crate::st_bridge::stb_model::stb_sections::StbSecFigureBeam;
use crate::st_bridge::stb_model::stb_sections::StbSecFigureSlab;
use crate::st_bridge::stb_model::stb_sections::StbSecHaunch;
use crate::st_bridge::stb_model::stb_sections::StbSecPipe;
use crate::st_bridge::stb_model::stb_sections::StbSecRollBox;
use crate::st_bridge::stb_model::stb_sections::StbSecRollH;
use crate::st_bridge::stb_model::stb_sections::StbSecRollL;
use crate::st_bridge::stb_model::stb_sections::StbSecSlabRC;
use crate::st_bridge::stb_model::stb_sections::StbSecSteel;
use crate::st_bridge::stb_model::stb_sections::StbSecSteelBeam;
use crate::st_bridge::stb_model::stb_sections::StbSecSteelBrace;
use crate::st_bridge::stb_model::stb_sections::StbSecSteelColumn;
use crate::st_bridge::stb_model::stb_sections::StbSecStraightBeam;
use crate::st_bridge::stb_model::stb_sections::StbSecStraightSlab;
use crate::st_bridge::stb_model::stb_sections::StbSections;
use crate::st_bridge::stb_model::StbModel;
use crate::st_bridge::StBridge;

pub mod st_bridge;

pub fn read_st_bridge(file_name: &str) -> StBridge {
    let contents = get_contents(file_name);

    let document = roxmltree::Document::parse(&contents).unwrap();

    let root_node = document.root_element();

    let version = root_node.attribute("version").unwrap().to_string();

    let stb_common = extract_stb_common(root_node);

    let stb_model = extract_stb_model(root_node);

    let stb_extensions = extract_stb_extensions(root_node);

    StBridge {
        version,
        stb_common,
        stb_model,
        stb_extensions,
    }
}

pub fn get_contents(filename: &str) -> String {
    let mut f = File::open(filename).expect("File not found.");

    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    return contents;
}

pub fn extract_node<'a>(
    name: &str,
    parent: roxmltree::Node<'a, '_>,
) -> Option<roxmltree::Node<'a, 'a>> {
    let child_elements = parent.children().filter(|n| n.is_element());

    for node in child_elements {
        let tag_name = node.tag_name().name();

        if tag_name == name {
            return Some(node);
        }
    }

    None
}

pub fn extract_stb_common<'a>(root_node: roxmltree::Node<'a, 'a>) -> StbCommon {
    let stb_common_node = extract_node("StbCommon", root_node).unwrap();
    let stb_reinforcement_strength_list =
        extract_node("StbReinforcement_Strength_List", stb_common_node).unwrap();
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

pub fn extract_stb_model(root_node: roxmltree::Node) -> StbModel {
    let stb_model_node = extract_node("StbModel", root_node).unwrap();

    let stb_nodes = extract_stb_nodes(stb_model_node);

    let stb_axes = extract_stb_axes(stb_model_node);

    let stb_stories = extract_stb_stories(stb_model_node);

    let stb_members = extract_stb_members(stb_model_node);

    let stb_sections = extract_stb_sections(stb_model_node);

    StbModel {
        stb_nodes,
        stb_axes,
        stb_stories,
        stb_members,
        stb_sections,
    }
}

fn extract_stb_nodes(stb_model_node: roxmltree::Node) -> StbNodes {
    let stb_nodes_node = extract_node("StbNodes", stb_model_node).unwrap();

    let mut stb_nodes = StbNodes::new();

    for node in stb_nodes_node.children().filter(|n| n.is_element()) {
        let id_member = match node.attribute("id_member") {
            Some(s) => Some(s.parse::<i32>().unwrap()),
            None => None,
        };

        stb_nodes.insert(
            parse_attribute("id", node).unwrap(),
            StbNode {
                x: parse_attribute("x", node).unwrap(),
                y: parse_attribute("y", node).unwrap(),
                z: parse_attribute("z", node).unwrap(),
                kind: parse_enum_attribute("kind", node).unwrap(),
                id_member,
            },
        );
    }

    stb_nodes
}

fn extract_stb_axes(stb_model_node: roxmltree::Node) -> StbAxes {
    let stb_axes_node = extract_node("StbAxes", stb_model_node).unwrap();

    let mut stb_axes = StbAxes::new();

    for node in stb_axes_node.children().filter(|n| n.is_element()) {
        let stb_node_id_list_node = extract_node("StbNodeid_List", node).unwrap();

        let mut stb_node_id_list = Vec::new();

        for children in stb_node_id_list_node.children().filter(|n| n.is_element()) {
            let id = parse_attribute("id", children).unwrap();
            stb_node_id_list.push(StbNodeId { id });
        }

        match node.tag_name().name() {
            "StbX_Axis" => {
                stb_axes.stb_x_axis_list.push(StbXAxis {
                    id: parse_attribute("id", node).unwrap(),
                    name: parse_attribute("name", node).unwrap(),
                    distance: parse_attribute("distance", node).unwrap(),
                    stb_node_id_list: StbNodeIdList {
                        children: stb_node_id_list,
                    },
                });
            }
            "StbY_Axis" => {
                stb_axes.stb_y_axis_list.push(StbYAxis {
                    id: parse_attribute("id", node).unwrap(),
                    name: parse_attribute("name", node).unwrap(),
                    distance: parse_attribute("distance", node).unwrap(),
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
    let stb_stories_node = extract_node("StbStories", stb_model_node).unwrap();

    let mut stb_stories = StbStories::new();

    for node in stb_stories_node.children().filter(|n| n.is_element()) {
        let stb_node_id_list_node = extract_node("StbNodeid_List", node).unwrap();

        let mut stb_node_id_list = Vec::new();

        for children in stb_node_id_list_node.children().filter(|n| n.is_element()) {
            let id = parse_attribute("id", children).unwrap();
            stb_node_id_list.push(StbNodeId { id });
        }

        stb_stories.stb_story_list.push(StbStory {
            id: parse_attribute("id", node).unwrap(),
            name: parse_attribute("name", node).unwrap(),
            height: parse_attribute("height", node).unwrap(),
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
    let stb_members_node = extract_node("StbMembers", stb_model_node).unwrap();

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

fn extract_stb_columns(stb_members_node: roxmltree::Node) -> StbColumns {
    let stb_columns_node = extract_node("StbColumns", stb_members_node).unwrap();

    let mut stb_column_list: Vec<StbColumn> = Vec::new();

    for node in stb_columns_node.children().filter(|n| n.is_element()) {
        stb_column_list.push(StbColumn {
            id: parse_attribute("id", node).unwrap(),
            name: node.attribute("name").unwrap().to_string(),
            id_node_bottom: parse_attribute("idNode_bottom", node).unwrap(),
            id_node_top: parse_attribute("idNode_top", node).unwrap(),
            rotate: parse_attribute("rotate", node).unwrap(),
            id_section: parse_attribute("id_section", node).unwrap(),
            kind_structure: parse_enum_attribute("kind_structure", node).unwrap(),
            offset_x: parse_attribute("offset_X", node).unwrap(),
            offset_y: parse_attribute("offset_Y", node).unwrap(),
            condition_bottom: parse_enum_attribute("condition_bottom", node).unwrap(),
            condition_top: parse_enum_attribute("condition_top", node).unwrap(),
        });
    }

    StbColumns { stb_column_list }
}

fn extract_stb_posts(stb_members_node: roxmltree::Node) -> StbPosts {
    let stb_posts_node = extract_node("StbPosts", stb_members_node).unwrap();

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
    let stb_girders_node = extract_node("StbGirders", stb_members_node).unwrap();

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
    let stb_beams_node = extract_node("StbBeams", stb_members_node).unwrap();

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
    let stb_braces_node = extract_node("StbBraces", stb_members_node).unwrap();

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
    let stb_slabs_node = extract_node("StbSlabs", stb_members_node).unwrap();

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
    let stb_sections_node = extract_node("StbSections", stb_model_node).unwrap();

    let mut stb_sections = StbSections::new();

    for node in stb_sections_node.children().filter(|n| n.is_element()) {
        let tag_name = node.tag_name().name();

        match tag_name {
            "StbSecColumn_RC" => unimplemented_panic(tag_name),
            "StbSecColumn_S" => stb_sections
                .children
                .push(Box::new(extract_stb_sec_column_s(node))),
            "StbSecColumn_SRC" => unimplemented_panic(tag_name),
            "StbSecColumn_CFT" => unimplemented_panic(tag_name),
            "StbSecBeam_RC" => stb_sections
                .children
                .push(Box::new(extract_stb_sec_beam_rc(node))),
            "StbSecBeam_S" => stb_sections
                .children
                .push(Box::new(extract_stb_sec_beam_s(node))),
            "StbSecBeam_SRC" => unimplemented_panic(tag_name),
            "StbSecBrace_S" => stb_sections
                .children
                .push(Box::new(extract_stb_sec_brace_s(node))),
            "StbSecSlab_RC" => stb_sections
                .children
                .push(Box::new(extract_stb_sec_slab_rc(node))),
            "StbSecSlabDeck" => unimplemented_panic(tag_name),
            "StbSecSlabPrecast" => unimplemented_panic(tag_name),
            "StbSecWall_RC" => unimplemented_panic(tag_name),
            "StbSecFoundation_RC" => unimplemented_panic(tag_name),
            "StbSecPile_RC" => unimplemented_panic(tag_name),
            "StbSecPile_S" => unimplemented_panic(tag_name),
            "StbSecPileProduct" => unimplemented_panic(tag_name),
            "StbSecOpen_RC" => unimplemented_panic(tag_name),
            "StbSecParapet_RC" => unimplemented_panic(tag_name),
            "StbSecSteel" => stb_sections.stb_sec_steel = extract_stb_sec_steel(node),
            "StbSecUndefined" => unimplemented_panic(tag_name),
            _ => {
                panic!("Tag name {} is invalid.", tag_name)
            }
        };
    }

    stb_sections
}

fn extract_stb_sec_column_s(node: roxmltree::Node) -> StbSecColumnS {
    let stb_sec_steel_column_node = extract_node("StbSecSteelColumn", node).unwrap();
    StbSecColumnS {
        id: parse_attribute("id", node).unwrap(),
        name: parse_attribute("name", node).unwrap(),
        floor: parse_attribute("floor", node).unwrap(),
        kind_column: parse_enum_attribute("kind_column", node).unwrap(),
        direction: parse_attribute("direction", node).unwrap(),
        base_type: parse_enum_attribute("base_type", node).unwrap(),
        stb_sec_steel_column: StbSecSteelColumn {
            pos: parse_enum_attribute("pos", stb_sec_steel_column_node).unwrap(),
            shape: parse_enum_attribute("shape", stb_sec_steel_column_node).unwrap(),
            strength_main: parse_attribute("strength_web", stb_sec_steel_column_node).unwrap(),
            strength_web: parse_attribute("strength_web", stb_sec_steel_column_node).unwrap(),
        },
    }
}

fn extract_stb_sec_beam_rc(node: roxmltree::Node) -> StbSecBeamRC {
    let stb_sec_figure_node = extract_node("StbSecFigure", node).unwrap();
    let stb_sec_figure = extract_stb_sec_figure_beam(stb_sec_figure_node);

    let stb_sec_bar_arrangement_node = extract_node("StbSecBar_Arrangement", node).unwrap();
    let stb_sec_bar_arrangement = extract_stb_sec_bar_arrangement(stb_sec_bar_arrangement_node);

    StbSecBeamRC {
        id: parse_attribute("id", node).unwrap(),
        name: parse_attribute("name", node).unwrap(),
        floor: parse_attribute("name", node).unwrap(),
        kind_beam: parse_enum_attribute("kind_beam", node).unwrap(),
        is_foundation: parse_attribute("isFoundation", node).unwrap(),
        is_canti: parse_attribute("isCanti", node).unwrap(),
        d_reinforcement_main: parse_attribute("D_reinforcement_main", node).unwrap(),
        d_stirrup: parse_attribute("D_stirrup", node).unwrap(),
        d_reinforcement_web: parse_attribute("D_reinforcement_web", node).unwrap(),
        d_bar_spacing: parse_attribute("D_bar_spacing", node).unwrap(),
        strength_concrete: match node.attribute("strength_concrete") {
            Some(s) => Some(s.to_string()),
            None => None,
        },
        strength_reinforcement_main: parse_attribute("strength_reinforcement_main", node).unwrap(),
        strength_reinforcement_2nd_main: match node.attribute("strength_reinforcement_2nd_main") {
            Some(s) => Some(s.to_string()),
            None => None,
        },
        strength_stirrup: parse_attribute("strength_stirrup", node).unwrap(),
        strength_reinforcement_web: parse_attribute("strength_reinforcement_web", node).unwrap(),
        strength_bar_spacing: parse_attribute("strength_bar_spacing", node).unwrap(),
        depth_cover_left: match node.attribute("depth_cover_left") {
            Some(s) => Some(s.parse().unwrap()),
            None => None,
        },
        depth_cover_right: match node.attribute("depth_cover_right") {
            Some(s) => Some(s.parse().unwrap()),
            None => None,
        },
        depth_cover_top: match node.attribute("depth_cover_top") {
            Some(s) => Some(s.parse().unwrap()),
            None => None,
        },
        depth_cover_bottom: match node.attribute("depth_cover_bottom") {
            Some(s) => Some(s.parse().unwrap()),
            None => None,
        },
        stb_sec_figure,
        stb_sec_bar_arrangement,
    }
}

fn extract_stb_sec_bar_arrangement(node: roxmltree::Node) -> StbSecBarArrangementBeam {
    let stb_sec_beam_start_center_end_section_list = match extract_node("", node) {
        Some(_) => {
            let mut list = Vec::new();
            for children in node.children().filter(|n| n.is_element()) {
                list.push(extract_stb_sec_beam_start_center_end_section(children));
            }
            Some(list)
        }
        None => None,
    };

    let stb_sec_beam_same_section = match extract_node("", node) {
        Some(n) => Some(extract_stb_sec_beam_same_section(n)),
        None => None,
    };

    StbSecBarArrangementBeam {
        stb_sec_beam_start_center_end_section_list,
        stb_sec_beam_same_section,
    }
}

fn extract_stb_sec_beam_start_center_end_section(
    node: roxmltree::Node,
) -> StbSecBeamStartCenterEndSection {
    StbSecBeamStartCenterEndSection {
        pos: parse_enum_attribute("pos", node).unwrap(),
        count_main_top_1st: parse_attribute("count_main_top_1st", node).unwrap(),
        count_main_bottom_1st: parse_attribute("count_main_bottom_1st", node).unwrap(),
        count_stirrup: parse_attribute("count_stirrup", node).unwrap(),
        pitch_stirrup: parse_attribute("pitch_stirrup", node).unwrap(),
        count_web: parse_attribute("count_web", node).unwrap(),
        count_bar_spacing: parse_attribute("count_bar_spacing", node).unwrap(),
        pitch_bar_spacing: parse_attribute("pitch_bar_spacing", node).unwrap(),
    }
}

fn extract_stb_sec_beam_same_section(node: roxmltree::Node) -> StbSecBeamSameSection {
    StbSecBeamSameSection {
        count_main_top_1st: parse_attribute("count_main_top_1st", node).unwrap(),
        count_main_bottom_1st: parse_attribute("count_main_bottom_1st", node).unwrap(),
        count_stirrup: parse_attribute("count_stirrup", node).unwrap(),
        pitch_stirrup: parse_attribute("pitch_stirrup", node).unwrap(),
        count_web: parse_attribute("count_web", node).unwrap(),
        count_bar_spacing: parse_attribute("count_bar_spacing", node).unwrap(),
        pitch_bar_spacing: parse_attribute("pitch_bar_spacing", node).unwrap(),
    }
}

fn extract_stb_sec_figure_beam(node: roxmltree::Node) -> StbSecFigureBeam {
    let stb_sec_haunch = match extract_node("StbSecHaunch", node) {
        Some(n) => Some(extract_stb_sec_haunch(n)),
        None => None,
    };

    let stb_sec_straight = match extract_node("StbSecStraight", node) {
        Some(n) => Some(extract_stb_sec_straight(n)),
        None => None,
    };

    StbSecFigureBeam {
        stb_sec_haunch,
        stb_sec_straight,
    }
}

fn extract_stb_sec_haunch(node: roxmltree::Node) -> StbSecHaunch {
    StbSecHaunch {
        width_start: parse_attribute("width_start", node).unwrap(),
        depth_start: parse_attribute("depth_start", node).unwrap(),
        width_center: parse_attribute("width_center", node).unwrap(),
        depth_center: parse_attribute("depth_center", node).unwrap(),
        width_end: parse_attribute("width_end", node).unwrap(),
        depth_end: parse_attribute("depth_end", node).unwrap(),
    }
}

fn extract_stb_sec_straight(node: roxmltree::Node) -> StbSecStraightBeam {
    StbSecStraightBeam {
        depth: parse_attribute("depth", node).unwrap(),
    }
}

fn extract_stb_sec_beam_s(node: roxmltree::Node) -> StbSecBeamS {
    let stb_sec_steel_beam_node = extract_node("StbSecSteelBeam", node).unwrap();
    let stb_sec_steel_beam = extract_stb_sec_steel_beam(stb_sec_steel_beam_node);

    StbSecBeamS {
        id: parse_attribute("id", node).unwrap(),
        name: parse_attribute("name", node).unwrap(),
        floor: parse_attribute("floor", node).unwrap(),
        kind_beam: parse_enum_attribute("kind_beam", node).unwrap(),
        is_canti: parse_attribute("isCanti", node).unwrap(),
        stb_sec_steel_beam,
    }
}

fn extract_stb_sec_steel_beam(node: roxmltree::Node) -> StbSecSteelBeam {
    StbSecSteelBeam {
        pos: parse_enum_attribute("pos", node).unwrap(),
        shape: parse_attribute("shape", node).unwrap(),
        strength_main: parse_attribute("strength_main", node).unwrap(),
        strength_web: parse_attribute("strength_web", node).unwrap(),
    }
}

fn extract_stb_sec_brace_s(node: roxmltree::Node) -> StbSecBraceS {
    let stb_sec_steel_brace_node = extract_node("StbSecSteelBrace", node).unwrap();
    let stb_sec_steel_brace = StbSecSteelBrace {
        pos: parse_enum_attribute("pos", stb_sec_steel_brace_node).unwrap(),
        shape: parse_attribute("shape", stb_sec_steel_brace_node).unwrap(),
        strength_main: parse_attribute("strength_main", stb_sec_steel_brace_node).unwrap(),
        strength_web: parse_attribute("strength_web", stb_sec_steel_brace_node).unwrap(),
    };

    StbSecBraceS {
        id: parse_attribute("id", node).unwrap(),
        name: parse_attribute("name", node).unwrap(),
        floor: parse_attribute("floor", node).unwrap(),
        kind_brace: parse_enum_attribute("kind_brace", node).unwrap(),
        stb_sec_steel_brace,
    }
}

fn extract_stb_sec_slab_rc(node: roxmltree::Node) -> StbSecSlabRC {
    let stb_sec_figure_node = extract_node("StbSecFigure", node).unwrap();
    let stb_sec_straight_node = extract_node("StbSecStraight", stb_sec_figure_node).unwrap();
    let stb_sec_straight = StbSecStraightSlab {
        depth: parse_attribute("depth", stb_sec_straight_node).unwrap(),
    };
    let stb_sec_figure = StbSecFigureSlab { stb_sec_straight };

    let stb_sec_bar_arrangement_node = extract_node("StbSecBar_Arrangement", node).unwrap();
    let mut stb_sec_1way_slab_1_list = Vec::new();

    for children in stb_sec_bar_arrangement_node
        .children()
        .filter(|n| n.is_element())
    {
        stb_sec_1way_slab_1_list.push(StbSec1WaySlab1 {
            pos: parse_enum_attribute("pos", children).unwrap(),
            strength: parse_attribute("strength", children).unwrap(),
            d: parse_attribute("D", children).unwrap(),
            pitch: parse_attribute("pitch", children).unwrap(),
        });
    }

    let stb_sec_bar_arrangement = StbSecBarArrangementSlab {
        stb_sec_1way_slab_1_list,
    };

    StbSecSlabRC {
        id: parse_attribute("id", node).unwrap(),
        name: parse_attribute("name", node).unwrap(),
        is_foundation: parse_attribute("isFoundation", node).unwrap(),
        is_canti: parse_attribute("isCanti", node).unwrap(),
        strength_concrete: parse_attribute("strength_concrete", node).unwrap(),
        stb_sec_figure,
        stb_sec_bar_arrangement,
    }
}

fn extract_stb_sec_steel(stb_sec_steel_node: roxmltree::Node) -> StbSecSteel {
    let mut stb_sec_steel = StbSecSteel::new();

    for node in stb_sec_steel_node.children().filter(|n| n.is_element()) {
        let tag_name = node.tag_name().name();
        match tag_name {
            "StbSecRoll-H" => stb_sec_steel
                .children
                .push(Box::new(extract_stb_sec_roll_h(node))),
            "StbSecBuild-H" => stb_sec_steel
                .children
                .push(Box::new(extract_stb_sec_build_h(node))),
            "StbSecRoll-BOX" => stb_sec_steel
                .children
                .push(Box::new(extract_stb_sec_roll_box(node))),
            "StbSecBuild-BOX" => stb_sec_steel
                .children
                .push(Box::new(extract_stb_sec_build_box(node))),
            "StbSecPipe" => stb_sec_steel
                .children
                .push(Box::new(extract_stb_sec_pipe(node))),
            "StbSecRoll-T" => unimplemented_panic(tag_name),
            "StbSecRoll-C" => unimplemented_panic(tag_name),
            "StbSecRoll-L" => stb_sec_steel
                .children
                .push(Box::new(extract_stb_sec_roll_l(node))),
            "StbSecLipC" => unimplemented_panic(tag_name),
            "StbSecFlatBar" => unimplemented_panic(tag_name),
            "StbSecRoundBar" => unimplemented_panic(tag_name),
            "StbSecSteelProduct" => unimplemented_panic(tag_name),
            "StbSecSteelUndefined" => unimplemented_panic(tag_name),
            _ => panic!("Tag name {} is invalid.", tag_name),
        }
    }

    stb_sec_steel
}

fn extract_stb_sec_roll_h(node: roxmltree::Node) -> StbSecRollH {
    StbSecRollH {
        name: parse_attribute("name", node).unwrap(),
        sec_type: parse_enum_attribute("type", node).unwrap(),
        a: parse_attribute("A", node).unwrap(),
        b: parse_attribute("B", node).unwrap(),
        t1: parse_attribute("t1", node).unwrap(),
        t2: parse_attribute("t2", node).unwrap(),
        r: parse_attribute("r", node).unwrap(),
    }
}

fn extract_stb_sec_build_h(node: roxmltree::Node) -> StbSecBuildH {
    StbSecBuildH {
        name: parse_attribute("name", node).unwrap(),
        a: parse_attribute("A", node).unwrap(),
        b: parse_attribute("B", node).unwrap(),
        t1: parse_attribute("t1", node).unwrap(),
        t2: parse_attribute("t2", node).unwrap(),
    }
}

fn extract_stb_sec_roll_box(node: roxmltree::Node) -> StbSecRollBox {
    StbSecRollBox {
        name: parse_attribute("name", node).unwrap(),
        sec_type: parse_enum_attribute("type", node).unwrap(),
        a: parse_attribute("A", node).unwrap(),
        b: parse_attribute("B", node).unwrap(),
        t: parse_attribute("t", node).unwrap(),
        r: parse_attribute("R", node).unwrap(),
    }
}

fn extract_stb_sec_build_box(node: roxmltree::Node) -> StbSecBuildBox {
    StbSecBuildBox {
        name: parse_attribute("name", node).unwrap(),
        a: parse_attribute("A", node).unwrap(),
        b: parse_attribute("B", node).unwrap(),
        t1: parse_attribute("t1", node).unwrap(),
        t2: parse_attribute("t2", node).unwrap(),
    }
}

fn extract_stb_sec_pipe(node: roxmltree::Node) -> StbSecPipe {
    StbSecPipe {
        name: parse_attribute("name", node).unwrap(),
        d: parse_attribute("D", node).unwrap(),
        t: parse_attribute("t", node).unwrap(),
    }
}

fn extract_stb_sec_roll_l(node: roxmltree::Node) -> StbSecRollL {
    StbSecRollL {
        name: parse_attribute("name", node).unwrap(),
        sec_type: parse_enum_attribute("type", node).unwrap(),
        a: parse_attribute("A", node).unwrap(),
        b: parse_attribute("B", node).unwrap(),
        t1: parse_attribute("t1", node).unwrap(),
        t2: parse_attribute("t2", node).unwrap(),
        r1: parse_attribute("r1", node).unwrap(),
        r2: parse_attribute("r2", node).unwrap(),
        side: parse_attribute("side", node).unwrap(),
    }
}

pub fn extract_stb_extensions(root_node: roxmltree::Node) -> StbExtensions {
    let stb_extensions_node = extract_node("StbExtensions", root_node).unwrap();

    let mut stb_extension_list = Vec::new();

    for node in stb_extensions_node.children().filter(|n| n.is_element()) {
        stb_extension_list.push(StbExtension {
            identifier: parse_attribute("identifier", node).unwrap(),
            description: parse_attribute("description", node).unwrap(),
        });
    }

    StbExtensions { stb_extension_list }
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

fn unimplemented_panic(what: &str) {
    panic!("{} is unimplemented!", what);
}
