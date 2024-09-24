use crate::commons::Reference;

pub struct Description {
    alignment: Option<Alignment>,
    gender: String,
    eyes: String,
    size: Size,
    hair: String,
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
