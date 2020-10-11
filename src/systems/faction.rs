#[derive(PartialEq, Copy, Clone)]
pub enum Factions {
    Player,
    Aliens,
}

pub struct Faction {
    pub faction: Factions,
}
impl Faction {
    pub fn new(faction: Factions) -> Self {
        Self { faction }
    }
}
