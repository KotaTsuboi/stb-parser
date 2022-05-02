use strum_macros::EnumString;

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
    pub id: i32,
    pub name: String,
    pub floor: String,
    pub kind_column: ColumnKind,
    pub direction: bool,
    pub base_type: SteelBaseType,
    pub stb_sec_steel_column: StbSecSteelColumn,
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
    #[strum(serialize = "")]
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
    pub pos: StbSecSteelColumnPosition,
    pub shape: String,
    pub strength_main: String,
    pub strength_web: String,
}

#[derive(Debug, EnumString)]
pub enum StbSecSteelColumnPosition {
    #[strum(serialize = "ALL")]
    All,
}

#[derive(Debug)]
pub struct StbSecBeamRC {
    pub id: i32,
    pub name: String,
    pub floor: String,
    pub kind_beam: BeamKind,
    pub is_foundation: bool,
    pub is_canti: bool,
    pub d_reinforcement_main: String,
    pub d_stirrup: String,
    pub d_reinforcement_web: String,
    pub d_bar_spacing: String,
    pub strength_concrete: Option<String>,
    pub strength_reinforcement_main: String,
    pub strength_reinforcement_2nd_main: Option<String>,
    pub strength_stirrup: String,
    pub strength_reinforcement_web: String,
    pub strength_bar_spacing: String,
    pub depth_cover_left: Option<f64>,
    pub depth_cover_right: Option<f64>,
    pub depth_cover_top: Option<f64>,
    pub depth_cover_bottom: Option<f64>,
    pub stb_sec_figure: StbSecFigureBeam,
    pub stb_sec_bar_arrangement: StbSecBarArrangementBeam,
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
    pub stb_sec_haunch: Option<StbSecHaunch>,
    pub stb_sec_straight: Option<StbSecStraightBeam>,
}

#[derive(Debug)]
pub struct StbSecHaunch {
    pub width_start: f64,
    pub depth_start: f64,
    pub width_center: f64,
    pub depth_center: f64,
    pub width_end: f64,
    pub depth_end: f64,
}

#[derive(Debug)]
pub struct StbSecStraightBeam {
    pub depth: f64,
}

#[derive(Debug)]
pub struct StbSecBarArrangementBeam {
    pub stb_sec_beam_start_center_end_section_list: Option<Vec<StbSecBeamStartCenterEndSection>>,
    pub stb_sec_beam_same_section: Option<StbSecBeamSameSection>,
}

#[derive(Debug)]
pub struct StbSecBeamStartCenterEndSection {
    pub pos: StbSecBeamSectionPosition,
    pub count_main_top_1st: u32,
    pub count_main_bottom_1st: u32,
    pub count_stirrup: u32,
    pub pitch_stirrup: f64,
    pub count_web: u32,
    pub count_bar_spacing: u32,
    pub pitch_bar_spacing: f64,
}

#[derive(Debug)]
pub struct StbSecBeamSameSection {
    pub count_main_top_1st: u32,
    pub count_main_bottom_1st: u32,
    pub count_stirrup: u32,
    pub pitch_stirrup: f64,
    pub count_web: u32,
    pub count_bar_spacing: u32,
    pub pitch_bar_spacing: f64,
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
    pub id: i32,
    pub name: String,
    pub floor: String,
    pub kind_beam: BeamKind,
    pub is_canti: bool,
    pub stb_sec_steel_beam: StbSecSteelBeam,
}

impl StbSectionsChildren for StbSecBeamS {}

#[derive(Debug)]
pub struct StbSecSteelBeam {
    pub pos: StbSecSteelBeamPosition,
    pub shape: String,
    pub strength_main: String,
    pub strength_web: String,
}

#[derive(Debug, EnumString)]
pub enum StbSecSteelBeamPosition {
    #[strum(serialize = "ALL")]
    All,
}

#[derive(Debug)]
pub struct StbSecSlabRC {
    pub id: i32,
    pub name: String,
    pub is_foundation: bool,
    pub is_canti: bool,
    pub strength_concrete: String,
    pub stb_sec_figure: StbSecFigureSlab,
    pub stb_sec_bar_arrangement: StbSecBarArrangementSlab,
}

impl StbSectionsChildren for StbSecSlabRC {}

#[derive(Debug)]
pub struct StbSecFigureSlab {
    pub stb_sec_straight: StbSecStraightSlab,
}

#[derive(Debug)]
pub struct StbSecStraightSlab {
    pub depth: f64,
}

#[derive(Debug)]
pub struct StbSecBarArrangementSlab {
    pub stb_sec_1way_slab_1_list: Vec<StbSec1WaySlab1>,
}

#[derive(Debug)]
pub struct StbSec1WaySlab1 {
    pub pos: StbSec1WaySlab1Position,
    pub strength: String,
    pub d: String,
    pub pitch: f64,
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
    pub id: i32,
    pub name: String,
    pub floor: String,
    pub kind_brace: BraceKind,
    pub stb_sec_steel_brace: StbSecSteelBrace,
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
pub struct StbSecSteelBrace {
    pub pos: StbSecSteelBraceSPosition,
    pub shape: String,
    pub strength_main: String,
    pub strength_web: String,
}

#[derive(Debug, EnumString)]
pub enum StbSecSteelBraceSPosition {
    #[strum(serialize = "ALL")]
    All,
}

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

pub trait StbSecSteelChildren {}

#[derive(Debug)]
pub struct StbSecRollH {
    pub name: String,
    pub sec_type: StbSecRollHType,
    pub a: f64,
    pub b: f64,
    pub t1: f64,
    pub t2: f64,
    pub r: f64,
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
    pub name: String,
    pub a: f64,
    pub b: f64,
    pub t1: f64,
    pub t2: f64,
}

impl StbSecSteelChildren for StbSecBuildH {}

#[derive(Debug)]
pub struct StbSecRollBox {
    pub name: String,
    pub sec_type: StbSecRollBoxType,
    pub a: f64,
    pub b: f64,
    pub t: f64,
    pub r: f64,
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
pub struct StbSecBuildBox {
    pub name: String,
    pub a: f64,
    pub b: f64,
    pub t1: f64,
    pub t2: f64,
}

impl StbSecSteelChildren for StbSecBuildBox {}

#[derive(Debug)]
pub struct StbSecPipe {
    pub name: String,
    pub d: f64,
    pub t: f64,
}

impl StbSecSteelChildren for StbSecPipe {}

#[derive(Debug)]
pub struct StbSecRollL {
    pub name: String,
    pub sec_type: StbSecRollLType,
    pub a: f64,
    pub b: f64,
    pub t1: f64,
    pub t2: f64,
    pub r1: f64,
    pub r2: f64,
    pub side: bool,
}

impl StbSecSteelChildren for StbSecRollL {}

#[derive(Debug, EnumString)]
pub enum StbSecRollLType {
    #[strum(serialize = "L")]
    L,
}
