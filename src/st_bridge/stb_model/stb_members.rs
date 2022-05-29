use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::EnumString;

#[derive(Debug, Serialize, Deserialize)]
pub struct StbMembers {
    pub stb_member_map: HashMap<u32, StbMember>,
    //pub stb_columns: StbColumns,
    //pub stb_posts: StbPosts,
    //pub stb_girders: StbGirders,
    //pub stb_beams: StbBeams,
    //pub stb_braces: StbBraces,
    //pub stb_slabs: StbSlabs,
}

// TODO: implement StbWalls
// TODO: implement StbFootings
// TODO: implement StbStripFootings
// TODO: implement StbPiles
// TODO: implement StbFoundationColumns
// TODO: implement StbParapets
// TODO: implement StbOpens
#[derive(Debug, Serialize, Deserialize)]
pub enum StbMember {
    StbColumn {
        id: u32,
        name: String,
        id_node_bottom: i32,
        id_node_top: i32,
        rotate: f64,
        id_section: u32,
        kind_structure: ColumnStructureKind,
        offset_x: f64,
        offset_y: f64,
        condition_bottom: JointCondition,
        condition_top: JointCondition,
    },
    StbPost {
        id: u32,
        name: String,
        id_node_bottom: i32,
        id_node_top: i32,
        rotate: f64,
        id_section: u32,
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
    },
    StbGirder {
        id: u32,
        name: String,
        id_node_start: i32,
        id_node_end: i32,
        rotate: f64,
        id_section: u32,
        kind_structure: GirderStructureKind,
        is_foundation: bool,
        offset: f64,
        level: f64,
        type_haunch_h: Option<HaunchType>,
    },
    StbBeam {
        id: u32,
        name: String,
        id_node_start: i32,
        id_node_end: u32,
        rotate: f64,
        id_section: u32,
        kind_structure: GirderStructureKind,
        is_foundation: bool,
        offset: f64,
        level: f64,
    },
    StbBrace {
        id: u32,
        name: String,
        id_node_start: i32,
        id_node_end: i32,
        rotate: f64,
        id_section: u32,
        kind_structure: BraceStructureKind,
        offset_start_x: f64,
        offset_start_y: f64,
        offset_start_z: f64,
        offset_end_x: f64,
        offset_end_y: f64,
        offset_end_z: f64,
        condition_start: JointCondition,
        condition_end: JointCondition,
    },
    StbSlab {
        id: u32,
        name: String,
        id_section: u32,
        kind_structure: SlabStructureKind,
        kind_slab: SlabKind,
        level: f64,
        is_foundation: bool,
    },
}

impl StbMember {
    pub fn id(&self) -> u32 {
        match *self {
            StbMember::StbColumn { id, .. } => id,
            StbMember::StbSlab { id, .. } => id,
            StbMember::StbBrace { id, .. } => id,
            StbMember::StbBeam { id, .. } => id,
            StbMember::StbGirder { id, .. } => id,
            StbMember::StbPost { id, .. } => id,
        }
    }
}

/*
#[derive(Debug, Serialize, Deserialize)]
pub struct StbColumns {
    pub stb_column_list: Vec<StbColumn>,
}

impl StbColumns {
    pub fn new() -> StbColumns {
        StbColumns {
            stb_column_list: Vec::new(),
        }
    }
}
*/

#[derive(Debug, EnumString, Serialize, Deserialize)]
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

#[derive(Debug, EnumString, Serialize, Deserialize)]
pub enum JointCondition {
    #[strum(serialize = "FIX")]
    Fix,
    #[strum(serialize = "PIN")]
    Pin,
}

/*
#[derive(Debug, Serialize, Deserialize)]
pub struct StbGirders {
    pub stb_girder_list: Vec<StbGirder>,
}

impl StbGirders {
    pub fn new() -> StbGirders {
        StbGirders {
            stb_girder_list: Vec::new(),
        }
    }
}
*/

#[derive(Debug, EnumString, Serialize, Deserialize)]
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

#[derive(Debug, EnumString, Serialize, Deserialize)]
pub enum HaunchType {
    #[strum(serialize = "BOTH")]
    Both,
    #[strum(serialize = "RIGHT")]
    Right,
    #[strum(serialize = "LEFT")]
    Left,
}

/*
#[derive(Debug, Serialize, Deserialize)]
pub struct StbPosts {
    pub stb_post_list: Vec<StbPost>,
}

impl StbPosts {
    pub fn new() -> StbPosts {
        StbPosts {
            stb_post_list: Vec::new(),
        }
    }
}
*/

/*
#[derive(Debug, Serialize, Deserialize)]
pub struct StbBeams {
    pub stb_beam_list: Vec<StbBeam>,
}

impl StbBeams {
    pub fn new() -> StbBeams {
        StbBeams {
            stb_beam_list: Vec::new(),
        }
    }
}
*/

/*
#[derive(Debug, Serialize, Deserialize)]
pub struct StbSlabs {
    pub stb_slab_list: Vec<StbSlab>,
}

impl StbSlabs {
    pub fn new() -> StbSlabs {
        StbSlabs {
            stb_slab_list: Vec::new(),
        }
    }
}
*/

#[derive(Debug, EnumString, Serialize, Deserialize)]
pub enum SlabStructureKind {
    #[strum(serialize = "RC")]
    RC,
    #[strum(serialize = "DECK")]
    Deck,
    #[strum(serialize = "PRECAST")]
    Precast,
}

#[derive(Debug, EnumString, Serialize, Deserialize)]
pub enum SlabKind {
    #[strum(serialize = "NORMAL")]
    Normal,
    #[strum(serialize = "CANTI")]
    Canti,
}

/*
#[derive(Debug, Serialize, Deserialize)]
pub struct StbBraces {
    pub stb_brace_list: Vec<StbBrace>,
}

impl StbBraces {
    pub fn new() -> StbBraces {
        StbBraces {
            stb_brace_list: Vec::new(),
        }
    }
}
*/

#[derive(Debug, EnumString, Serialize, Deserialize)]
pub enum BraceStructureKind {
    #[strum(serialize = "RC")]
    RC,
    #[strum(serialize = "S")]
    S,
    #[strum(serialize = "SRC")]
    SRC,
}
