use std::collections::HashSet;

use crate::{
    ability::{Ability, Skill},
    commons::{Charges, Currency, DamageType, Dice, Distance},
    description::{Background, Description},
    equipment::Item,
    feature::Feature,
    spell::{Spell, Spellcasting},
};

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Sense {
    Blindsight(Distance),
    Darkvision(Distance),
    Tremorsense(Distance),
    Truesight(Distance),
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Health {
    current_hp: u32,
    max_hp: u32,
    temp_hp: u32,
    hit_dice: Dice,
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
#[derive(Debug)]
pub enum Speed {
    Burrowing(Distance),
    Climbing(Distance),
    Flying(Distance),
    Swimming(Distance),
    Walking(Distance),
}
