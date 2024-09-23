use crate::commons::{Currency, Damage, Distance, Reference};

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
    armor: Option<Armor>,
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
    category: WeaponCategory,
    properties: Vec<WeaponProperty>,
    range: Option<Range>,
}

pub enum WeaponCategory {
    Martial,
    Simple,
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
    Thrown(Distance, Distance),
    TwoHanded(Damage),
    Versatile,
}

pub struct Range {
    base: Distance,
    long: Option<Distance>,
}

pub struct Armor {
    armor_type: ArmorType,
    ac: u32,
    ac_bonus: String,
    strength_requirement: Option<u32>,
    stealth_disadvantage: bool,
}

pub enum ArmorType {
    Light,
    Medium,
    Heavy,
}
