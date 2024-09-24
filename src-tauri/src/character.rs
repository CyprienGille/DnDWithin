use std::collections::HashSet;

use crate::{
    ability::{Ability, Skill},
    commons::{Charges, Currency, Distance},
    description::{Background, Description},
    equipment::{Attunement, Item},
    spell::Spell,
};

pub struct Character {
    name: String,
    class: String,
    level: u8,
    player_name: String,
    species: String,
    abilities: Vec<Ability>,
    skills: Vec<Skill>,
    passives: Vec<Skill>,
    conditions: HashSet<Condition>,
    exhaustion: u8,
    background: Background,
    senses: HashSet<Sense>,
    languages: Vec<String>,
    description: Description,
    equipment: Vec<Item>,
    attunement_slots: Charges,
    currency: Currency,
    spell_list: Vec<Spell>,
}

pub enum Sense {
    Blindsight(Distance),
    Darkvision(Distance),
    Tremorsense(Distance),
    Truesight(Distance),
}

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
