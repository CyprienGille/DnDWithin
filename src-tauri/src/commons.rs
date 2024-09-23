pub struct Reference {
    source_name: String,
    long_source_name: Option<String>,
    page: u32,
}

pub struct Time {
    number: u32,
    unit: TimeUnit,
}

pub enum TimeUnit {
    Instant,
    BonusAction,
    Action,
    Reaction,
    Minute,
    Hour,
}

pub struct Distance {
    number: u32,
    unit: DistanceUnit,
}

impl Distance {
    pub fn new(number: u32, unit: DistanceUnit) -> Self {
        Self { number, unit }
    }

    pub fn feet(number: u32) -> Self {
        Self {
            number,
            unit: DistanceUnit::Feet,
        }
    }

    pub fn miles(number: u32) -> Self {
        Self {
            number,
            unit: DistanceUnit::Miles,
        }
    }
}

#[derive(Default)]
pub enum DistanceUnit {
    #[default]
    Feet,
    Miles,
    Meters,
    Kilometers,
}

pub struct Damage {
    dmg_type: DamageType,
    dice_number: u32,
    dice_type: Dice,
    bonus: i32,
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
    electrum: Option<i32>,
    gold: Option<i32>,
    platinum: Option<i32>,
}
