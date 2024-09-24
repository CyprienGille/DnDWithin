use crate::commons::{Charges, Reference};

pub struct Feature {
    name: String,
    reference: Option<Reference>,
    source: String,
    given_by: FeatureOrigin,
    description: String,
    charges: Option<Charges>,
}

pub enum FeatureOrigin {
    Species,
    Class,
    Feat,
    Item,
    Other,
}
