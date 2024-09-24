pub struct Ability {
    name: String,
    short_name: String,
    score: i32,
    saving_throw: SavingThrowInfo,
}

impl Ability {
    pub fn new(
        name: String,
        short_name: String,
        score: i32,
        saving_throw: SavingThrowInfo,
    ) -> Self {
        Self {
            name,
            short_name,
            score,
            saving_throw,
        }
    }

    pub fn init(name: String) -> Self {
        let short_name = name[0..3].to_uppercase(); // Strength -> STR
        Self {
            name,
            short_name,
            score: 10,
            saving_throw: SavingThrowInfo::default(),
        }
    }

    pub fn classic() -> Vec<Self> {
        vec![
            Self::init("Strength".to_string()),
            Self::init("Dexterity".to_string()),
            Self::init("Constitution".to_string()),
            Self::init("Intelligence".to_string()),
            Self::init("Wisdom".to_string()),
            Self::init("Charisma".to_string()),
        ]
    }
}

#[derive(Debug, Default)]
pub struct SavingThrowInfo {
    proficiency: Proficiency,
    bonus: i32,
}

pub struct Skill {
    name: String,
    proficiency: Proficiency,
    bonus: i32,
}

impl Skill {
    pub fn new(name: String, proficiency: Proficiency, bonus: i32) -> Self {
        Self {
            name,
            proficiency,
            bonus,
        }
    }

    pub fn init(name: String) -> Self {
        Self {
            name,
            proficiency: Proficiency::default(),
            bonus: 0,
        }
    }

    pub fn classic() -> Vec<Self> {
        vec![
            Self::init("Acrobatics".to_string()),
            Self::init("Animal Handling".to_string()),
            Self::init("Arcana".to_string()),
            Self::init("Athletics".to_string()),
            Self::init("Deception".to_string()),
            Self::init("History".to_string()),
            Self::init("Insight".to_string()),
            Self::init("Intimidation".to_string()),
            Self::init("Investigation".to_string()),
            Self::init("Medicine".to_string()),
            Self::init("Nature".to_string()),
            Self::init("Perception".to_string()),
            Self::init("Performance".to_string()),
            Self::init("Persuation".to_string()),
            Self::init("Religion".to_string()),
            Self::init("Sleight of Hand".to_string()),
            Self::init("Stealth".to_string()),
            Self::init("Survival".to_string()),
        ]
    }
    pub fn passives() -> Vec<Self> {
        vec![
            Self::init("Insight".to_string()),
            Self::init("Investigation".to_string()),
            Self::init("Perception".to_string()),
        ]
    }
}

#[derive(Debug, Default)]
pub enum Proficiency {
    #[default]
    Not,
    Half,
    Proficient,
    Expert,
}
