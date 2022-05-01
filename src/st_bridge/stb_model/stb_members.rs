pub struct StbMembers {
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
    type_haunch_h: HanchType,
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
    id_node_start: i32,
    id_node_end: i32,
    rotate: f64,
    id_section: i32,
    kind_structure: GirderStructureKind,
    is_foundation: bool,
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
