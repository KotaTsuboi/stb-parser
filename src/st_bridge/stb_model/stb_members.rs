use strum_macros::EnumString;

#[derive(Debug)]
pub struct StbMembers {
    pub stb_columns: StbColumns,
    pub stb_posts: StbPosts,
    pub stb_girders: StbGirders,
    pub stb_beams: StbBeams,
    pub stb_braces: StbBraces,
    pub stb_slabs: StbSlabs,
    // TODO: implement StbWalls
    // TODO: implement StbFootings
    // TODO: implement StbStripFootings
    // TODO: implement StbPiles
    // TODO: implement StbFoundationColumns
    // TODO: implement StbParapets
    // TODO: implement StbOpens
}

#[derive(Debug)]
pub struct StbColumns {
    pub stb_column_list: Vec<StbColumn>,
}

/*
impl StbColumns {
    pub fn new() -> StbColumns {
        StbColumns {
            stb_column_list: Vec::new(),
        }
    }
}
*/

#[derive(Debug)]
pub struct StbColumn {
    pub id: i32,
    pub name: String,
    pub id_node_bottom: i32,
    pub id_node_top: i32,
    pub rotate: f64,
    pub id_section: i32,
    pub kind_structure: ColumnStructureKind,
    pub offset_x: f64,
    pub offset_y: f64,
    pub condition_bottom: JointCondition,
    pub condition_top: JointCondition,
}

#[derive(Debug, EnumString)]
pub enum ColumnStructureKind {
    #[strum(serialize = "RC")]
    RC,
    #[strum(serialize = "S")]
    S,
    #[strum(serialize = "SRC")]
    SRC,
    #[strum(serialize = "CFT")]
    CFT,
    #[strum(serialize = "UNDEFINED")]
    Undefined,
}

#[derive(Debug, EnumString)]
pub enum JointCondition {
    #[strum(serialize = "FIX")]
    Fix,
    #[strum(serialize = "PIN")]
    Pin,
}

#[derive(Debug)]
pub struct StbGirders {
    pub stb_girder_list: Vec<StbGirder>,
}

/*
impl StbGirders {
    pub fn new() -> StbGirders {
        StbGirders {
            stb_girder_list: Vec::new(),
        }
    }
}
*/

#[derive(Debug)]
pub struct StbGirder {
    pub id: i32,
    pub name: String,
    pub id_node_start: i32,
    pub id_node_end: i32,
    pub rotate: f64,
    pub id_section: i32,
    pub kind_structure: GirderStructureKind,
    pub is_foundation: bool,
    pub offset: f64,
    pub level: f64,
    pub type_haunch_h: Option<HaunchType>,
}

#[derive(Debug, EnumString)]
pub enum GirderStructureKind {
    #[strum(serialize = "RC")]
    RC,
    #[strum(serialize = "S")]
    S,
    #[strum(serialize = "SRC")]
    SRC,
    #[strum(serialize = "UNDEFINED")]
    Undefined,
}

#[derive(Debug, EnumString)]
pub enum HaunchType {
    #[strum(serialize = "BOTH")]
    Both,
    #[strum(serialize = "RIGHT")]
    Right,
    #[strum(serialize = "LEFT")]
    Left,
}

#[derive(Debug)]
pub struct StbPosts {
    pub stb_post_list: Vec<StbPost>,
}

/*
impl StbPosts {
    pub fn new() -> StbPosts {
        StbPosts {
            stb_post_list: Vec::new(),
        }
    }
}
*/

#[derive(Debug)]
pub struct StbPost {
    pub id: i32,
    pub name: String,
    pub id_node_bottom: i32,
    pub id_node_top: i32,
    pub rotate: f64,
    pub id_section: i32,
    pub kind_structure: ColumnStructureKind,
    pub offset_x: f64,
    pub offset_y: f64,
    pub offset_bottom_x: f64,
    pub offset_bottom_y: f64,
    pub offset_bottom_z: f64,
    pub offset_top_x: f64,
    pub offset_top_y: f64,
    pub offset_top_z: f64,
    pub condition_bottom: JointCondition,
    pub condition_top: JointCondition,
}

#[derive(Debug)]
pub struct StbBeams {
    pub stb_beam_list: Vec<StbBeam>,
}

/*
impl StbBeams {
    pub fn new() -> StbBeams {
        StbBeams {
            stb_beam_list: Vec::new(),
        }
    }
}
*/

#[derive(Debug)]
pub struct StbBeam {
    pub id: i32,
    pub name: String,
    pub id_node_start: i32,
    pub id_node_end: i32,
    pub rotate: f64,
    pub id_section: i32,
    pub kind_structure: GirderStructureKind,
    pub is_foundation: bool,
    pub offset: f64,
    pub level: f64,
}

#[derive(Debug)]
pub struct StbSlabs {
    pub stb_slab_list: Vec<StbSlab>,
}

/*
impl StbSlabs {
    pub fn new() -> StbSlabs {
        StbSlabs {
            stb_slab_list: Vec::new(),
        }
    }
}
*/

#[derive(Debug)]
pub struct StbSlab {
    pub id: i32,
    pub name: String,
    pub id_section: i32,
    pub kind_structure: SlabStructureKind,
    pub kind_slab: SlabKind,
    pub level: f64,
    pub is_foundation: bool,
}

#[derive(Debug, EnumString)]
pub enum SlabStructureKind {
    #[strum(serialize = "RC")]
    RC,
    #[strum(serialize = "DECK")]
    Deck,
    #[strum(serialize = "PRECAST")]
    Precast,
}

#[derive(Debug, EnumString)]
pub enum SlabKind {
    #[strum(serialize = "NORMAL")]
    Normal,
    #[strum(serialize = "CANTI")]
    Canti,
}

#[derive(Debug)]
pub struct StbBraces {
    pub stb_brace_list: Vec<StbBrace>,
}

/*
impl StbBraces {
    pub fn new() -> StbBraces {
        StbBraces {
            stb_brace_list: Vec::new(),
        }
    }
}
*/

#[derive(Debug)]
pub struct StbBrace {
    pub id: i32,
    pub name: String,
    pub id_node_start: i32,
    pub id_node_end: i32,
    pub rotate: f64,
    pub id_section: i32,
    pub kind_structure: BraceStructureKind,
    pub offset_start_x: f64,
    pub offset_start_y: f64,
    pub offset_start_z: f64,
    pub offset_end_x: f64,
    pub offset_end_y: f64,
    pub offset_end_z: f64,
    pub condition_start: JointCondition,
    pub condition_end: JointCondition,
}

#[derive(Debug, EnumString)]
pub enum BraceStructureKind {
    #[strum(serialize = "RC")]
    RC,
    #[strum(serialize = "S")]
    S,
    #[strum(serialize = "SRC")]
    SRC,
}
