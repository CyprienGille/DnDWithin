use crate::{
    ability::Ability,
    commons::{Damage, Distance, Reference, Time},
};

pub struct Spell {
    name: String,
    reference: Reference,
    level: u32,
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
    saving_throw: Option<Ability>,
    base_damage: Option<Vec<Damage>>,
    damage_increase: Option<DamageIncrease>,
}

impl Spell {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        reference: Reference,
        level: u32,
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
        saving_throw: Option<Ability>,
        base_damage: Option<Vec<Damage>>,
        damage_increase: Option<DamageIncrease>,
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
            saving_throw,
            base_damage,
            damage_increase,
        }
    }
}

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

pub struct Range {
    range_type: RangeType,
    distance: Option<Distance>,
}

enum RangeType {
    Touch,
    Self_,
    Point,
}

pub struct Size {
    size_type: SizeType,
    distance: Option<Distance>,
}

enum SizeType {
    Point,
    Sphere,
    Cone,
    Line,
    Square,
}

pub struct Components {
    verbal: bool,
    somatic: bool,
    material: Option<String>,
}

pub struct DamageIncrease {
    active: bool,
    times: u32,
    to_add: Damage,
}
