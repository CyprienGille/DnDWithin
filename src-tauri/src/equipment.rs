use crate::commons::{Charges, Damage, Distance, Reference, Weight};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    name: String,
    reference: Option<Reference>,
    rarity: Rarity,
    wondrous: bool,
    weight: Option<Weight>,
    attunement: Attunement,
    description: String,
    value: Currency,
    weapon: Option<Weapon>,
    armor: Option<Armor>,
    charges: Option<Charges>,
    equipped: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum Rarity {
    #[default]
    Common,
    Uncommon,
    Rare,
    VeryRare,
    Legendary,
    Artifact,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attunement {
    required: bool,
    by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weapon {
    damage: Vec<Damage>,
    category: WeaponCategory,
    properties: Vec<WeaponProperty>,
    range: Option<Range>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeaponCategory {
    Martial,
    Simple,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    base: Distance,
    long: Option<Distance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Armor {
    armor_type: ArmorType,
    ac: u32,
    ac_bonus: String,
    strength_requirement: Option<u32>,
    stealth_disadvantage: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArmorType {
    Light,
    Medium,
    Heavy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
    copper: Option<i32>,
    silver: Option<i32>,
    gold: Option<i32>,
    platinum: Option<i32>,
}

impl Currency {
    pub fn copper_only(amount: i32) -> Self {
        Self {
            copper: Some(amount),
            silver: None,
            gold: None,
            platinum: None,
        }
    }
    pub fn silver_only(amount: i32) -> Self {
        Self {
            copper: None,
            silver: Some(amount),
            gold: None,
            platinum: None,
        }
    }
    pub fn gold_only(amount: i32) -> Self {
        Self {
            copper: None,
            silver: None,
            gold: Some(amount),
            platinum: None,
        }
    }
    pub fn platinum_only(amount: i32) -> Self {
        Self {
            copper: None,
            silver: None,
            gold: None,
            platinum: Some(amount),
        }
    }
}

impl Default for Currency {
    fn default() -> Self {
        Self {
            copper: Some(0),
            silver: Some(0),
            gold: Some(0),
            platinum: Some(0),
        }
    }
}
