use strum_macros::EnumString;

#[derive(Debug)]
pub struct StbNodes {
    pub stb_node_list: Vec<StbNode>,
}

impl StbNodes {
    pub fn new() -> StbNodes {
        StbNodes {
            stb_node_list: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct StbNode {
    pub id: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub kind: StbNodeKind,
    pub id_member: Option<i32>,
}

#[derive(Debug, EnumString)]
pub enum StbNodeKind {
    #[strum(serialize = "ON_GIRDER")]
    OnGirder,
    #[strum(serialize = "ON_BEAM")]
    OnBeam,
    #[strum(serialize = "ON_COLUMN")]
    OnColumn,
    #[strum(serialize = "ON_POST")]
    OnPost,
    #[strum(serialize = "ON_GRID")]
    OnGrid,
    #[strum(serialize = "ON_CANTI")]
    OnCanti,
    #[strum(serialize = "ON_SLAB")]
    OnSlab,
    #[strum(serialize = "OTHER")]
    Other,
}
