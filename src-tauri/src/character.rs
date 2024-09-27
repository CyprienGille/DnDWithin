use std::{collections::HashSet, sync::Mutex};

use serde::{Deserialize, Serialize};

use crate::{
    ability::{Ability, Skill},
    commons::{Charges, DamageType, Dice, Distance},
    description::{Background, Description},
    equipment::{Currency, Item},
    feature::Feature,
    spell::{Spell, Spellcasting},
};

pub struct CharacterState {
    pub state: Mutex<Character>,
}

impl CharacterState {
    pub fn init() -> Self {
        Self {
            state: Mutex::new(Character::placeholder()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    name: String,
    class: String,
    level: u8,
    proficiency_bonus: u8,
    player_name: String,
    species: String,
    health: Health,
    abilities: Vec<Ability>,
    skills: Vec<Skill>,
    passives: Vec<Skill>,
    ac: u8,
    initiative: i8,
    initiative_fluff: String,
    speeds: Vec<Speed>,
    inspiration: bool,
    conditions: HashSet<Condition>,
    exhaustion: u8,
    background: Background,
    senses: HashSet<Sense>,
    languages: Vec<String>,
    weapon_profs: Vec<String>,
    armor_profs: Vec<String>,
    tool_profs: Vec<String>,
    features: Vec<Feature>,
    description: Description,
    equipment: Vec<Item>,
    attunement_slots: Charges,
    currency: Currency,
    spellcasting: Spellcasting,
    spell_list: Vec<Spell>,
}

impl Character {
    pub fn get_empty() -> Self {
        Self {
            name: "".to_string(),
            class: "".to_string(),
            level: 1,
            proficiency_bonus: 0,
            player_name: "".to_string(),
            species: "".to_string(),
            health: Health {
                current_hp: 0,
                max_hp: 0,
                temp_hp: 0,
                hit_dice: Vec::new(),
            },
            abilities: Ability::classic(),
            skills: Skill::classic(),
            passives: Skill::passives(),
            ac: 0,
            initiative: 0,
            initiative_fluff: "".to_string(),
            speeds: vec![Speed::Walking(Distance::Feet(30))],
            inspiration: false,
            conditions: HashSet::new(),
            exhaustion: 0,
            background: Background::get_empty(),
            senses: HashSet::new(),
            languages: Vec::new(),
            weapon_profs: Vec::new(),
            armor_profs: Vec::new(),
            tool_profs: Vec::new(),
            features: Vec::new(),
            description: Description::default(),
            equipment: Vec::new(),
            attunement_slots: Charges::empty_attun_slots(),
            currency: Currency::default(),
            spellcasting: Spellcasting::default(),
            spell_list: Vec::new(),
        }
    }
    pub fn placeholder() -> Self {
        Self {
            name: "Character Name".to_string(),
            class: "Class".to_string(),
            level: 1,
            proficiency_bonus: 0,
            player_name: "Player Name".to_string(),
            species: "Species".to_string(),
            health: Health {
                current_hp: 0,
                max_hp: 0,
                temp_hp: 0,
                hit_dice: Vec::new(),
            },
            abilities: Ability::classic(),
            skills: Skill::classic(),
            passives: Skill::passives(),
            ac: 0,
            initiative: 0,
            initiative_fluff: "".to_string(),
            speeds: vec![Speed::Walking(Distance::Feet(30))],
            inspiration: false,
            conditions: HashSet::new(),
            exhaustion: 0,
            background: Background::placeholder(),
            senses: HashSet::new(),
            languages: Vec::new(),
            weapon_profs: Vec::new(),
            armor_profs: Vec::new(),
            tool_profs: Vec::new(),
            features: Vec::new(),
            description: Description::default(),
            equipment: Vec::new(),
            attunement_slots: Charges::empty_attun_slots(),
            currency: Currency::default(),
            spellcasting: Spellcasting::default(),
            spell_list: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Hash)]
pub enum Sense {
    Blindsight(Distance),
    Darkvision(Distance),
    Tremorsense(Distance),
    Truesight(Distance),
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, Eq, PartialEq, Hash)]
pub enum Condition {
    Blinded,
    Charmed,
    Deafened,
    Frightened,
    Grappled,
    Incapacitated,
    Invisible,
    Paralyzed,
    Petrified,
    Poisoned,
    Prone,
    Restrained,
    Stunned,
    Unconscious,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Health {
    current_hp: u32,
    max_hp: u32,
    temp_hp: u32,
    hit_dice: Vec<Dice>,
}

#[derive(Debug)]
pub struct Defenses {
    dmg_interaction: Vec<DamageInteraction>,
    cond_immunities: Vec<Condition>,
    fluff: String,
}
#[derive(Debug)]
pub enum DamageInteraction {
    Resistant(DamageType),
    Immune(DamageType),
    Vulnerable(DamageType),
}
#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub enum Speed {
    Burrowing(Distance),
    Climbing(Distance),
    Flying(Distance),
    Swimming(Distance),
    Walking(Distance),
}
