pub struct StbCommon {
    stb_reinforcement_strength_list: StbReinforcementStrengthList,
}

impl StbCommon {
    pub fn new() -> StbCommon {
        let list = Vec::new();
        let stb_reinforcement_strength_list = StbReinforcementStrengthList { list };
        StbCommon {
            stb_reinforcement_strength_list,
        }
    }

    pub fn push(&self, strength: StbReinforcementStrength) {
        self.stb_reinforcement_strength_list.list.push(strength);
    }
}

struct StbReinforcementStrengthList {
    list: Vec<StbReinforcementStrength>,
}

struct StbReinforcementStrength {
    d: String,
    sd: String,
}
