use crate::{
    ability::Ability,
    commons::{Damage, Distance, Reference, Time},
};
#[derive(Debug)]
pub struct Spellcasting {
    modifier: i8,
    spell_attack: i8,
    save_dc: i8,
}
#[derive(Debug)]
pub struct Spell {
    name: String,
    reference: Option<Reference>,
    level: u8,
    school: School,
    casting_time: Time,
    range: Range,
    size: Size,
    components: Components,
    duration: Time,
    concentration: bool,
    prepared: bool,
    known: bool,
    description: String,
    consumes_slot: bool,
    upcast: u8,
    saving_throw: Option<Ability>,
    base_damage: Option<Vec<Damage>>,
    damage_increase: Option<Damage>,
}

impl Spell {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        reference: Option<Reference>,
        level: u8,
        school: School,
        casting_time: Time,
        range: Range,
        size: Size,
        components: Components,
        duration: Time,
        concentration: bool,
        prepared: bool,
        known: bool,
        description: String,
        consumes_slot: bool,
        upcast: u8,
        saving_throw: Option<Ability>,
        base_damage: Option<Vec<Damage>>,
        damage_increase: Option<Damage>,
    ) -> Self {
        Self {
            name,
            reference,
            level,
            school,
            casting_time,
            range,
            size,
            components,
            duration,
            concentration,
            prepared,
            known,
            description,
            consumes_slot,
            upcast,
            saving_throw,
            base_damage,
            damage_increase,
        }
    }
}
#[derive(Debug)]
pub enum School {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
}
#[derive(Debug)]
pub struct Range {
    range_type: RangeType,
    distance: Option<Distance>,
}
#[derive(Debug)]
enum RangeType {
    Touch,
    Self_,
    Point,
}
#[derive(Debug)]
pub struct Size {
    size_type: SizeType,
    distance: Option<Distance>,
}

#[derive(Debug, Default)]
enum SizeType {
    #[default]
    Point,
    Sphere,
    Cone,
    Line,
    Square,
}
#[derive(Debug)]
pub struct Components {
    verbal: bool,
    somatic: bool,
    material: Option<String>,
}
