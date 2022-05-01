mod stb_common;
mod stb_extensions;
mod stb_model;

use crate::stb_common::StbCommon;
use crate::stb_extensions::StbExtensions;
use crate::stb_model::StbCommon;

mod st_bridge {
    struct StBridge {
        version: String,
        stb_common: StbCommon,
        stb_model: StbModel,
        stb_extensions: StbExtensions,
    }
}
