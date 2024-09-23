use crate::commons::{Currency, Damage, Reference};

pub struct Item {
    name: String,
    reference: Reference,
    rarity: Rarity,
    wondrous: bool,
    weight: Option<Weight>,
    attunement: Attunement,
    description: String,
    value: Currency,
    weapon: Option<Weapon>,
}

pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    VeryRare,
    Legendary,
    Artifact,
    None,
}

pub struct Weight {
    number: u32,
    unit: WeightUnit,
}

pub enum WeightUnit {
    Pounds,
    Grams,
    Kilograms,
}

pub struct Attunement {
    required: bool,
    by: Option<String>,
}

pub struct Weapon {
    damage: Vec<Damage>,
    properties: Vec<WeaponProperty>,
}

pub enum WeaponProperty {
    Ammunition,
    Finesse,
    Heavy,
    Light,
    Loading,
    Range,
    Reach,
    Special,
    Thrown,
    TwoHanded,
    Versatile,
}
