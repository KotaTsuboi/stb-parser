pub mod stb_common;
pub mod stb_extensions;
pub mod stb_model;

use self::stb_common::StbCommon;
use self::stb_extensions::StbExtensions;
use self::stb_model::StbModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StBridge {
    pub version: String,
    pub stb_common: StbCommon,
    pub stb_model: StbModel,
    pub stb_extensions: StbExtensions,
}
