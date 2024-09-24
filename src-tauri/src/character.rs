use crate::{
    ability::{Ability, Skill},
    commons::Distance,
    description::Background,
    equipment::Item,
    spell::Spell,
};

pub struct Character {
    name: String,
    class: String,
    level: u32,
    player_name: String,
    species: String,
    abilities: Vec<Ability>,
    skills: Vec<Skill>,
    passives: Vec<Skill>,
    background: Background,
    senses: Vec<Sense>,
    languages: Vec<String>,
    exhaustion: Exhaustion,
    equipment: Vec<Item>,
    spell_list: Vec<Spell>,
}

pub struct Sense {
    sense_type: SenseType,
    range: Distance,
}

pub enum SenseType {
    Blindsight,
    Darkvision,
    Tremorsense,
    Truesight,
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

pub enum Exhaustion {
    None,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}
