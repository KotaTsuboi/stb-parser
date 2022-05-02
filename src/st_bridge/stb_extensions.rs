#[derive(Debug)]
pub struct StbExtensions {
    pub stb_extension_list: Vec<StbExtension>,
}

#[derive(Debug)]
pub struct StbExtension {
    pub identifier: String,
    pub description: String,
}
