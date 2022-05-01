mod stb_axes_and_stories;
mod stb_members;
mod stb_nodes;

use stb_axes_and_stories::StbAxes;
use stb_axes_and_stories::StbStories;
use stb_members::StbMembers;
use stb_nodes::StbNodes;

pub struct StbModel {
    stb_nodes: StbNodes,
    stb_axes: StbAxes,
    stb_stories: StbStories,
    stb_members: StbMembers,
    stb_sections: StbSections,
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
    d: String,
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
