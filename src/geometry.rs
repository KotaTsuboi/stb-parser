pub struct Node {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Node {
    pub fn new(x: f64, y: f64, z: f64) -> Node {
        Node { x, y, z }
    }

    pub fn distance(ni: Node, nj: Node) -> f64 {
        ((ni.x - nj.x).powi(2) + (ni.y - nj.y).powi(2) + (ni.z - nj.z).powi(2)).sqrt()
    }
}

pub struct Shape {
    pub base: Node,
    pub node_list: Vec<Node>,
}

impl Shape {
    pub fn new(base: Node, node_list: Vec<Node>) -> Shape {
        Shape { base, node_list }
    }
}

pub struct ExtrudeGeometry {
    pub shape: Shape,
    pub length: f64,
    pub theta_x: f64,
    pub theta_y: f64,
    pub theta_z: f64,
}

impl ExtrudeGeometry {
    pub fn new(shape: Shape, ni: Node, nj: Node, rotate: f64) -> ExtrudeGeometry {
        let xr = nj.x - ni.x;
        let yr = nj.y - ni.y;
        let zr = nj.z - ni.z;

        ExtrudeGeometry {
            shape,
            length: Node::distance(ni, nj),
            theta_x: (yr / zr).atan(),
            theta_y: (xr / (yr.powi(2) + zr.powi(2)).sqrt()).atan(),
            theta_z: rotate,
        }
    }
}
