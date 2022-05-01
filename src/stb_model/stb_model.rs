pub struct StbModel {
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
    pos: StbSecSteelColumnPosition,
    shape: String,
    strength_main: String,
    strength_web: String,
}

enum StbSecSteelColumnPosition {
    All,
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
    stb_sec_figure: StbSecFigureBeam,
    stb_sec_bar_arrangement: StbSecBarArrangementBeam,
}

enum BeamKind {
    Girder,
    Beam,
}

struct StbSecFigureBeam {
    stb_sec_haunch: StbSecHaunch,
}

struct StbSecHaunch {
    width_start: f64,
    depth_start: f64,
    width_center: f64,
    depth_center: f64,
    width_end: f64,
    depth_end: f64,
}

struct StbSecBarArrangementBeam {
    stb_sec_beam_start_center_end_section_list: StbSecBeamStartCenterEndSection,
}

struct StbSecBeamStartCenterEndSection {
    pos: StbSecBeamSectionPosition,
    count_main_top_1st: u32,
    count_main_bottom_1st: u32,
    count_stirrup: u32,
    pitch_stirrup: f64,
    count_web: u32,
    count_bar_spacing: u32,
    pitch_bar_spacing: f64,
}

enum StbSecBeamSectionPosition {
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
    pos: StbSecSteelBeamPosition,
    shape: String,
    strength_main: String,
    strength_web: String,
}

enum StbSecSteelBeamPosition {
    All,
}

struct StbSecSlabRC {
    id: i32,
    name: String,
    is_foundation: bool,
    is_canti: bool,
    strength_concrete: String,
    stb_sec_figure: StbSecFigureSlab,
    stb_sec_bar_arrangement: StbSecBarArrangementSlab,
}

struct StbSecFigureSlab {
    stb_sec_straight: StbSecStraight,
}

struct StbSecStraight {
    depth: f64,
}

struct StbSecBarArrangementSlab {
    stb_sec_1way_slab_1_list: Vec<StbSec1WaySlab1>,
}

struct StbSec1WaySlab1 {
    pos: StbSec1WaySlab1Position,
    strength: String,
    D: String,
    pitch: f64,
}

enum StbSec1WaySlab1Position {
    MainTop,
    MainBottom,
    TransverseTop,
    TransverseBottom,
}

struct StbSecBraceS {
    id: i32,
    name: String,
    floor: String,
    kind_brace: BraceKind,
    stb_sec_steel_figure_brace_s: StbSecSteelBraceS,
}

enum BraceKind {
    Vertical,
    Horizontal,
}

struct StbSecSteelBraceS {
    pos: StbSecSteelBraceSPosition,
    shape: String,
    strength_main: String,
    strength_web: String,
}

enum StbSecSteelBraceSPosition {
    All,
}

struct StbSecSteel {
    children_list: Vec<Box<dyn StbSecSteelChildren>>,
}

trait StbSecSteelChildren {}

struct StbSecRollH {
    name: String,
    sec_type: StbSecRollHType,
    a: f64,
    b: f64,
    t1: f64,
    t2: f64,
    r: f64,
}

enum StbSecRollHType {
    H,
    SH,
}

impl StbSecSteelChildren for StbSecRollH {}

struct StbSecBuildH {
    name: String,
    a: f64,
    b: f64,
    t1: f64,
    t2: f64,
}

impl StbSecSteelChildren for StbSecBuildH {}

struct StbSecRollBox {
    name: String,
    sec_type: StbSecRollBoxType,
    a: f64,
    b: f64,
    t: f64,
    r: f64,
}

enum StbSecRollBoxType {
    BCP,
    BCR,
    STKR,
    Else,
}

impl StbSecSteelChildren for StbSecRollBox {}

struct StbSecBuildBOX {
    name: String,
    a: f64,
    b: f64,
    t1: f64,
    t2: f64,
}

impl StbSecSteelChildren for StbSecBuildBOX {}

struct StbSecPipe {
    name: String,
    d: f64,
    t: f64,
}

impl StbSecSteelChildren for StbSecPipe {}

struct StbSecRollL {
    name: String,
    sec_type: StbSecRollLType,
    a: f64,
    b: f64,
    t1: f64,
    t2: f64,
    r1: f64,
    r2: f64,
    side: bool,
}

enum StbSecRollLType {
    L,
}
