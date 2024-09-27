use crate::{
    ability::Ability,
    commons::{Damage, Distance, Reference, Time},
};

#[derive(Debug, Default, Clone)]
pub struct Spellcasting {
    modifier: i8,
    spell_attack: i8,
    save_dc: i8,
}

#[derive(Debug, Clone)]
pub struct Spell {
    name: String,
    reference: Option<Reference>,
    level: u8,
    school: School,
    casting_time: Time,
    range: Range,
    size: Option<Size>,
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
        size: Option<Size>,
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

    pub fn placeholder() -> Self {
        Self {
            name: "Spell name".to_string(),
            reference: Some(Reference::phb_spells()),
            level: 0,
            school: School::Abjuration,
            casting_time: Time::Action,
            range: Range {
                range_type: RangeType::Self_,
                distance: None,
            },
            size: None,
            components: Components {
                verbal: true,
                somatic: true,
                material: None,
            },
            duration: Time::Hour(1),
            concentration: true,
            prepared: false,
            known: true,
            description: "Description of the spell".to_string(),
            consumes_slot: true,
            upcast: 0,
            saving_throw: None,
            base_damage: None,
            damage_increase: None,
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Range {
    range_type: RangeType,
    distance: Option<Distance>,
}

#[derive(Debug, Clone)]
enum RangeType {
    Touch,
    Self_,
    Point,
}

#[derive(Debug, Clone)]
pub struct Size {
    size_type: SizeType,
    distance: Option<Distance>,
}

#[derive(Debug, Default, Clone)]
enum SizeType {
    #[default]
    Point,
    Sphere,
    Cone,
    Line,
    Square,
}

#[derive(Debug, Clone)]
pub struct Components {
    verbal: bool,
    somatic: bool,
    material: Option<String>,
}
