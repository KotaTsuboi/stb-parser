struct StBridge {
    version: String,
    stb_common: StbCommon,
    stb_model: StbModel,
    stb_extensions: StbExtensions,
}

struct StbCommon {
    stb_reinforcement_strength_list: StbReinforcementStrengthList,
}

struct StbReinforcementStrengthList {
    list: Vec<StbReinforcementStrength>,
}

struct StbReinforcementStrength {
    d: String,
    sd: String,
}

struct StbModel {
    stb_nodes: StbNodes,
    stb_axes: StbAxes,
    stb_stories: StbStories,
    stb_members: StbMembers,
    stb_sections: StbSections,
}

struct StbNodes {
    stb_node_list: Vec<StbNode>,
}

struct StbNode {
    id: i32,
    x: f64,
    y: f64,
    z: f64,
    kind: StbNodeKind,
}

enum StbNodeKind {
    OnGirder,
    OnBeam,
    OnColumn,
    OnPost,
    OnGrid,
    OnCanti,
    OnSlab,
    Other,
}

struct StbAxes {
    stb_x_axis_list: Vec<StbXAxis>,
    stb_y_axis_list: Vec<StbYAxis>,
}

struct StbXAxis {
    id: i32,
    name: String,
    distance: f64,
    stb_node_id_list: StbNodeIdList,
}

struct StbNodeIdList {
    stb_node_id_list: Vec<StbNodeId>,
}

struct StbYAxis {
    id: i32,
    name: String,
    distance: f64,
    stb_node_id_list: Vec<StbNodeId>,
}

struct StbNodeId {
    id: i32,
}

struct StbStories {
    stb_story_list: Vec<StbStory>,
}

struct StbStory {
    id: i32,
    name: String,
    height: f64,
    kind: StbStoryKind,
    concrete_strength: String,
    stb_node_id_list: StbNodeIdList,
}

enum StbStoryKind {
    General,
    Basement,
    Roof,
    Penthouse,
    Isolation,
    Dependence,
}

struct StbMembers {
    stb_footings: StbFootings,
    stb_piles: StbPiles,
    stb_columns: StbColumns,
    stb_girders: StbGirders,
    stb_walls: StbWalls,
    stb_posts: StbPosts,
    stb_beams: StbBeams,
    stb_slabs: StbSlabs,
    stb_braces: StbBraces,
}

struct StbFootings {}

struct StbPiles {}

struct StbColumns {
    stb_column_list: Vec<StbColumn>,
}

struct StbColumn {
    id: i32,
    name: String,
    id_node_bottom: i32,
    id_node_top: i32,
    rotate: f64,
    id_section: i32,
    kind_structure: ColumnStructureKind,
    offset_z: f64,
    offset_y: f64,
    condition_bottom: JointCondition,
    condition_top: JointCondition,
}

enum ColumnStructureKind {
    RC,
    S,
    SRC,
    CFT,
    Undefined,
}

enum JointCondition {
    Fix,
    Pin,
}

struct StbGirders {
    stb_girder_list: Vec<StbGirder>,
}

struct StbGirder {
    id: i32,
    name: String,
    id_node_start: i32,
    id_node_end: i32,
    rotate: f64,
    id_section: i32,
    kind_structure: GirderStructureKind,
    is_foundation: bool,
    offset: f64,
    level: f64,
    type_haunch_H: HanchType,
}

enum GirderStructureKind {
    RC,
    S,
    SRC,
    Undefined,
}

enum HanchType {
    Both,
    Right,
    Left,
}

struct StbWalls {}

struct StbPosts {
    stb_post_list: Vec<StbPost>,
}

struct StbPost {
    id: i32,
    name: String,
    id_node_bottom: i32,
    id_node_top: i32,
    rotate: f64,
    id_section: i32,
    kind_structure: ColumnStructureKind,
    offset_x: f64,
    offset_y: f64,
    offset_bottom_x: f64,
    offset_bottom_y: f64,
    offset_bottom_z: f64,
    offset_top_x: f64,
    offset_top_y: f64,
    offset_top_z: f64,
    condition_bottom: JointCondition,
    condition_top: JointCondition,
}

struct StbBeams {
    stb_beam_list: Vec<StbBeam>,
}

struct StbBeam {
    id: i32,
    name: String,
    idNode_start: i32,
    idNode_end: i32,
    rotate: f64,
    id_section: i32,
    kind_structure: GirderStructureKind,
    isFoundation: bool,
    offset: f64,
    level: f64,
}

struct StbSlabs {
    stb_slab_list: Vec<StbSlab>,
}

struct StbSlab {
    id: i32,
    name: String,
    id_section: i32,
    kind_structure: SlabStructureKind,
    kind_slab: SlabKind,
    level: f64,
    is_foundation: bool,
}

enum SlabStructureKind {
    Rc,
    Deck,
    Precast,
}

enum SlabKind {
    Normal,
    Canti,
}

struct StbBraces {
    stb_brace_list: Vec<StbBrace>,
}

struct StbBrace {
    id: i32,
    name: String,
    id_node_start: i32,
    id_node_end: i32,
    rotate: f64,
    id_section: i32,
    kind_structure: BraceStructureKind,
    offset_start_x: f64,
    offset_start_y: f64,
    offset_start_z: f64,
    offset_end_x: f64,
    offset_end_y: f64,
    offset_end_z: f64,
    condition_start: JointCondition,
    condition_end: JointCondition,
}

enum BraceStructureKind {
    RC,
    S,
    SRC,
}

struct StbSections {
    stb_sec_column_s_list: Vec<StbSecColumnS>,
    stb_sec_beam_rc_list: Vec<StbSecBeamRC>,
    stb_sec_beam_s_list: Vec<StbSecBeamS>,
    stb_sec_slab_rc_list: Vec<StbSecSlabRC>,
    stb_sec_brace_s_list: Vec<StbSecBraceS>,
    stb_sec_steel: StbSecSteel,
}

struct StbSecColumnS {
    id: i32,
    name: String,
    floor: String,
    kind_column: ColumnKind,
    direction: bool,
    base_type: SteelBaseType,
    stb_sec_steel_column: StbSecSteelColumn,
}

enum ColumnKind {
    Column,
    Post,
}

enum SteelBaseType {
    Null,
    Expose,
    Embedded,
    Wrap,
}

struct StbSecSteelColumn {
    pos: String,
    shape: String,
    strength_main: String,
    strength_web: String,
}

struct StbSecBeamRC {
    id: i32,
    name: String,
    floor: String,
    kind_beam: BeamKind,
    is_foundation: bool,
    is_canti: bool,
    d_reinforcement_main: String,
    d_stirrup: String,
    d_reinforcement_web: String,
    d_bar_spacing: String,
    strength_concrete: String,
    strength_reinforcement_main: String,
    strength_reinforcement_2nd_main: String,
    strength_stirrup: String,
    strength_reinforcement_web: String,
    strength_bar_spacing: String,
    depth_cover_left: f64,
    depth_cover_right: f64,
    depth_cover_top: f64,
    depth_cover_bottom: f64,
    stb_sec_figure: StbSecFigure,
    stb_sec_bar_arrangement: StbSecBarArrangement,
}

enum BeamKind {
    Girder,
    Beam,
}

struct StbSecFigure {
    stb_sec_haunch: StbSecHaunch,
    stb_sec_straight: StbSecStraight,
}

struct StbSecHaunch {
    width_start: f64,
    depth_start: f64,
    width_center: f64,
    depth_center: f64,
    width_end: f64,
    depth_end: f64,
}

struct StbSecStraight {
    depth: f64,
}

struct StbSecBarArrangement {
    stb_sec_beam_start_center_end_section_list: StbSecBeamStartCenterEndSection,
}

struct StbSecBeamStartCenterEndSection {
    pos: Position,
    count_main_top_1st: u32,
    count_main_bottom_1st: u32,
    count_stirrup: u32,
    pitch_stirrup: f64,
    count_web: u32,
    count_bar_spacing: u32,
    pitch_bar_spacing: f64,
}

enum Position {
    Start,
    Center,
    End,
}

struct StbSecBeamS {
    id: i32,
    name: String,
    floor: String,
    kind_beam: BeamKind,
    is_canti: bool,
    stb_sec_steel_beam: StbSecSteelBeam,
}

struct StbSecSteelBeam {
    pos: String,
    shape: String,
    strength_main: String,
    strength_web: String,
}

struct StbSecSlabRC {
    id: i32,
    name: String,
    is_foundation: bool,
    is_canti: bool,
    strength_concrete: String,
    stb_sec_figure: StbSecFigure,
    stb_sec_bar_arrangement: StbSecBarArrangement,
}

struct StbExtensions {}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn it_works() {
        let filename = "data/steel_standard_model_utf8.stb";

        let mut f = File::open(filename).expect("File not found.");

        let mut contents = String::new();

        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

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
            println!("name: {}, text{:?}", node.tag_name().name(), node.text());
        }
    }
}
