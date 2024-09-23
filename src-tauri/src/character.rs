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
    background: Background,
    senses: Vec<Sense>,
    languages: Vec<String>,
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
