#[derive(Debug, Clone)]
pub struct Reference {
    source_name: String,
    long_source_name: Option<String>,
    page: u32,
}

impl Reference {
    pub fn phb_spells() -> Self {
        Self {
            source_name: "PHB".to_string(),
            long_source_name: Some("Player's Handbook".to_string()),
            page: 211,
        }
    }

    pub fn dmg_items() -> Self {
        Self {
            source_name: "DMG".to_string(),
            long_source_name: Some("Dungeon Master's Guide".to_string()),
            page: 150,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Time {
    Instant,
    BonusAction,
    Action,
    Reaction,
    Minute(u32),
    Hour(u32),
}
#[derive(Debug, Clone, Copy)]
pub enum Distance {
    Inches(u32),
    Feet(u32),
    Miles(u32),
    Centimeters(u32),
    Meters(u32),
    Kilometers(u32),
}
#[derive(Debug, Clone, Copy)]
pub struct Damage {
    dmg_type: DamageType,
    roll: Roll,
}
#[derive(Debug, Clone, Copy)]
pub enum DamageType {
    Acid,
    Bludgeoning,
    Cold,
    Fire,
    Force,
    Lightning,
    Necrotic,
    Piercing,
    Poison,
    Psychic,
    Radiant,
    Slashing,
    Thunder,
}

#[derive(Debug, Clone, Copy)]
pub struct Roll {
    dice: Dice,
    bonus: i32,
    reroll: Reroll,
}

impl Roll {
    pub fn d20(bonus: i32, reroll: Reroll) -> Self {
        Self {
            dice: Dice::D20(1),
            bonus,
            reroll,
        }
    }

    pub fn flat_d20(bonus: i32) -> Self {
        Self {
            dice: Dice::D20(1),
            bonus,
            reroll: Reroll::Flat,
        }
    }
}

impl Default for Roll {
    fn default() -> Self {
        Self {
            dice: Dice::D20(1),
            bonus: 0,
            reroll: Reroll::Flat,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Dice {
    D4(u32),
    D6(u32),
    D8(u32),
    D10(u32),
    D12(u32),
    D20(u32),
    D100(u32),
}

#[derive(Debug, Clone, Copy)]
pub enum Reroll {
    Advantage,
    Disadvantage,
    Flat,
}

#[derive(Debug, Clone)]
pub struct Charges {
    current: u32,
    max: u32,
    recharge: Option<Recharge>,
}

impl Charges {
    pub fn empty(max: u32, recharge: Option<Recharge>) -> Self {
        Self {
            current: 0,
            max,
            recharge,
        }
    }

    pub fn full(max: u32, recharge: Option<Recharge>) -> Self {
        Self {
            current: max,
            max,
            recharge,
        }
    }

    pub fn empty_attun_slots() -> Self {
        Self {
            current: 0,
            max: 3,
            recharge: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Recharge {
    short_rest: bool,
    long_rest: bool,
    desc: String,
    regained_charges: Option<Roll>,
}

#[derive(Debug, Clone, Copy)]
pub enum Weight {
    Pounds(u32),
    Grams(u32),
    Kilograms(u32),
}

impl Default for Weight {
    fn default() -> Self {
        Self::Pounds(0)
    }
}
