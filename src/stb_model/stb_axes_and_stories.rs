pub struct StbAxes {
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

pub struct StbStories {
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
