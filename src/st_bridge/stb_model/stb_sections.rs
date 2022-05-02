#[derive(Debug)]
pub struct StbSections {
    //stb_sec_column_s_list: Vec<StbSecColumnS>,
    //stb_sec_beam_rc_list: Vec<StbSecBeamRC>,
    //stb_sec_beam_s_list: Vec<StbSecBeamS>,
    //stb_sec_slab_rc_list: Vec<StbSecSlabRC>,
    //stb_sec_brace_s_list: Vec<StbSecBraceS>,
    pub children: Vec<Box<dyn StbSectionsChildren>>,
    pub stb_sec_steel: StbSecSteel,
}

impl StbSections {
    pub fn new() -> StbSections {
        StbSections {
            children: Vec::new(),
            stb_sec_steel: StbSecSteel::new(),
        }
    }
}

pub trait StbSectionsChildren {}

#[derive(Debug)]
pub struct StbSecColumnS {
    id: i32,
    name: String,
    floor: String,
    kind_column: ColumnKind,
    direction: bool,
    base_type: SteelBaseType,
    stb_sec_steel_column: StbSecSteelColumn,
}

impl StbSectionsChildren for StbSecColumnS {}

#[derive(Debug, EnumString)]
pub enum ColumnKind {
    #[strum(serialize = "COLUMN")]
    Column,
    #[strum(serialize = "POST")]
    Post,
}

#[derive(Debug, EnumString)]
pub enum SteelBaseType {
    #[strum(serialize = "NONE")]
    Null,
    #[strum(serialize = "EXPOSE")]
    Expose,
    #[strum(serialize = "EMBEDDED")]
    Embedded,
    #[strum(serialize = "WRAP")]
    Wrap,
}

#[derive(Debug)]
pub struct StbSecSteelColumn {
    pos: StbSecSteelColumnPosition,
    shape: String,
    strength_main: String,
    strength_web: String,
}

#[derive(Debug, EnumString)]
pub enum StbSecSteelColumnPosition {
    #[strum(serialize = "ALL")]
    All,
}

#[derive(Debug)]
pub struct StbSecBeamRC {
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

impl StbSectionsChildren for StbSecBeamRC {}

#[derive(Debug, EnumString)]
pub enum BeamKind {
    #[strum(serialize = "GIRDER")]
    Girder,
    #[strum(serialize = "BEAM")]
    Beam,
}

#[derive(Debug)]
pub struct StbSecFigureBeam {
    stb_sec_haunch: StbSecHaunch,
}

#[derive(Debug)]
pub struct StbSecHaunch {
    width_start: f64,
    depth_start: f64,
    width_center: f64,
    depth_center: f64,
    width_end: f64,
    depth_end: f64,
}

#[derive(Debug)]
pub struct StbSecBarArrangementBeam {
    stb_sec_beam_start_center_end_section_list: StbSecBeamStartCenterEndSection,
}

#[derive(Debug)]
pub struct StbSecBeamStartCenterEndSection {
    pos: StbSecBeamSectionPosition,
    count_main_top_1st: u32,
    count_main_bottom_1st: u32,
    count_stirrup: u32,
    pitch_stirrup: f64,
    count_web: u32,
    count_bar_spacing: u32,
    pitch_bar_spacing: f64,
}

#[derive(Debug, EnumString)]
pub enum StbSecBeamSectionPosition {
    #[strum(serialize = "START")]
    Start,
    #[strum(serialize = "CENTER")]
    Center,
    #[strum(serialize = "END")]
    End,
}

#[derive(Debug)]
pub struct StbSecBeamS {
    id: i32,
    name: String,
    floor: String,
    kind_beam: BeamKind,
    is_canti: bool,
    stb_sec_steel_beam: StbSecSteelBeam,
}

impl StbSectionsChildren for StbSecBeamS {}

#[derive(Debug)]
pub struct StbSecSteelBeam {
    pos: StbSecSteelBeamPosition,
    shape: String,
    strength_main: String,
    strength_web: String,
}

#[derive(Debug, EnumString)]
pub enum StbSecSteelBeamPosition {
    #[strum(serialize = "ALL")]
    All,
}

#[derive(Debug)]
pub struct StbSecSlabRC {
    id: i32,
    name: String,
    is_foundation: bool,
    is_canti: bool,
    strength_concrete: String,
    stb_sec_figure: StbSecFigureSlab,
    stb_sec_bar_arrangement: StbSecBarArrangementSlab,
}

impl StbSectionsChildren for StbSecSlabRC {}

#[derive(Debug)]
pub struct StbSecFigureSlab {
    stb_sec_straight: StbSecStraight,
}

#[derive(Debug)]
pub struct StbSecStraight {
    depth: f64,
}

#[derive(Debug)]
pub struct StbSecBarArrangementSlab {
    stb_sec_1way_slab_1_list: Vec<StbSec1WaySlab1>,
}

#[derive(Debug)]
pub struct StbSec1WaySlab1 {
    pos: StbSec1WaySlab1Position,
    strength: String,
    d: String,
    pitch: f64,
}

#[derive(Debug, EnumString)]
pub enum StbSec1WaySlab1Position {
    #[strum(serialize = "MAIN_TOP")]
    MainTop,
    #[strum(serialize = "MAIN_BOTTOM")]
    MainBottom,
    #[strum(serialize = "TRANSVERS_TOP")]
    TransverseTop,
    #[strum(serialize = "TRANSVERS_BOTTOM")]
    TransverseBottom,
}

#[derive(Debug)]
pub struct StbSecBraceS {
    id: i32,
    name: String,
    floor: String,
    kind_brace: BraceKind,
    stb_sec_steel_figure_brace_s: StbSecSteelBraceS,
}

impl StbSectionsChildren for StbSecBraceS {}

#[derive(Debug, EnumString)]
pub enum BraceKind {
    #[strum(serialize = "VERTICAL")]
    Vertical,
    #[strum(serialize = "HORIZONTAL")]
    Horizontal,
}

#[derive(Debug)]
pub struct StbSecSteelBraceS {
    pos: StbSecSteelBraceSPosition,
    shape: String,
    strength_main: String,
    strength_web: String,
}

#[derive(Debug, EnumString)]
pub enum StbSecSteelBraceSPosition {
    #[strum(serialize = "ALL")]
    All,
}

#[derive(Debug)]
pub struct StbSecSteel {
    pub children: Vec<Box<dyn StbSecSteelChildren>>,
}

impl StbSecSteel {
    pub fn new() -> StbSecSteel {
        StbSecSteel {
            children: Vec::new(),
        }
    }
}

trait StbSecSteelChildren {}

#[derive(Debug)]
pub struct StbSecRollH {
    name: String,
    sec_type: StbSecRollHType,
    a: f64,
    b: f64,
    t1: f64,
    t2: f64,
    r: f64,
}

impl StbSecSteelChildren for StbSecRollH {}

#[derive(Debug, EnumString)]
pub enum StbSecRollHType {
    #[strum(serialize = "H")]
    H,
    #[strum(serialize = "SH")]
    SH,
}

#[derive(Debug)]
pub struct StbSecBuildH {
    name: String,
    a: f64,
    b: f64,
    t1: f64,
    t2: f64,
}

impl StbSecSteelChildren for StbSecBuildH {}

#[derive(Debug)]
pub struct StbSecRollBox {
    name: String,
    sec_type: StbSecRollBoxType,
    a: f64,
    b: f64,
    t: f64,
    r: f64,
}

impl StbSecSteelChildren for StbSecRollBox {}

#[derive(Debug, EnumString)]
pub enum StbSecRollBoxType {
    #[strum(serialize = "BCP")]
    BCP,
    #[strum(serialize = "BCR")]
    BCR,
    #[strum(serialize = "STKR")]
    STKR,
    #[strum(serialize = "ELSE")]
    Else,
}

#[derive(Debug)]
pub struct StbSecBuildBOX {
    name: String,
    a: f64,
    b: f64,
    t1: f64,
    t2: f64,
}

impl StbSecSteelChildren for StbSecBuildBOX {}

#[derive(Debug)]
pub struct StbSecPipe {
    name: String,
    d: f64,
    t: f64,
}

impl StbSecSteelChildren for StbSecPipe {}

#[derive(Debug)]
pub struct StbSecRollL {
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

impl StbSecSteelChildren for StbSecRollL {}

#[derive(Debug, EnumString)]
pub enum StbSecRollLType {
    #[strum(serialize = "L")]
    L,
}
