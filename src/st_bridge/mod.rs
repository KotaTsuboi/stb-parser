pub mod stb_common;
pub mod stb_extensions;
pub mod stb_model;

pub mod st_bridge {
    use super::stb_common::StbCommon;
    use super::stb_extensions::StbExtensions;
    use super::stb_model::StbModel;

    struct StBridge {
        version: String,
        stb_common: StbCommon,
        stb_model: StbModel,
        stb_extensions: StbExtensions,
    }
}
