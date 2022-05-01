#[derive(Debug)]
pub struct StbCommon<'a> {
    stb_reinforcement_strength_list: StbReinforcementStrengthList<'a>,
}

impl<'a> StbCommon<'a> {
    pub fn new() -> StbCommon<'static> {
        let list = Vec::new();
        let stb_reinforcement_strength_list = StbReinforcementStrengthList { list };
        StbCommon {
            stb_reinforcement_strength_list,
        }
    }

    pub fn push(&mut self, value: StbReinforcementStrength<'a>) {
        self.stb_reinforcement_strength_list.list.push(value);
    }
}

#[derive(Debug)]
struct StbReinforcementStrengthList<'a> {
    list: Vec<StbReinforcementStrength<'a>>,
}

#[derive(Debug)]
pub struct StbReinforcementStrength<'a> {
    pub d: &'a str,
    pub sd: &'a str,
}
