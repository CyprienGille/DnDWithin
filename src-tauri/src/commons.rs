pub struct Reference {
    source_name: String,
    long_source_name: Option<String>,
    page: u32,
}

pub enum Time {
    Instant,
    BonusAction,
    Action,
    Reaction,
    Minute(u32),
    Hour(u32),
}

pub enum Distance {
    Inches(u32),
    Feet(u32),
    Miles(u32),
    Centimeters(u32),
    Meters(u32),
    Kilometers(u32),
}

pub struct Damage {
    dmg_type: DamageType,
    roll: Roll,
}

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

pub struct Roll {
    dice: Dice,
    dice_amount: u32,
    bonus: i32,
}

pub enum Dice {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100,
}

pub struct Currency {
    copper: Option<i32>,
    silver: Option<i32>,
    gold: Option<i32>,
    platinum: Option<i32>,
}

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
}

pub struct Recharge {
    short_rest: bool,
    long_rest: bool,
    desc: String,
    regained_charges: Option<Roll>,
}
