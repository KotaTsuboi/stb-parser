pub struct StbCommon {
    stb_reinforcement_strength_list: StbReinforcementStrengthList,
}

struct StbReinforcementStrengthList {
    list: Vec<StbReinforcementStrength>,
}

struct StbReinforcementStrength {
    d: String,
    sd: String,
}
