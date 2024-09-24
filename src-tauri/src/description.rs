use crate::{commons::Reference, equipment::Weight};

pub struct Description {
    alignment: Option<Alignment>,
    gender: String,
    eyes: String,
    size: Size,
    hair: String,
    height: String,
    faith: String,
    skin: String,
    age: u64,
    weight: Weight,
    personality: Vec<String>,
    ideals: Vec<String>,
    bonds: Vec<String>,
    flaws: Vec<String>,
    organizations: Vec<String>,
    allies: Vec<String>,
    enemies: Vec<String>,
    backstory: String,
    other_notes: String,
}

#[derive(Default, Debug)]
pub enum Alignment {
    Unaligned,
    LawfulGood,
    LawfulNeutral,
    LawfulEvil,
    NeutralGood,
    #[default]
    Neutral,
    NeutralEvil,
    ChaoticGood,
    ChaoticNeutral,
    ChaoticEvil,
}

#[derive(Default)]
pub enum Size {
    Tiny,
    Small,
    #[default]
    Medium,
    Large,
    Huge,
    Gargantuan,
}

pub struct Background {
    name: String,
    reference: Option<Reference>,
    description: String,
}
