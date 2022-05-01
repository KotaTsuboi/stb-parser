pub struct StbNodes {
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
