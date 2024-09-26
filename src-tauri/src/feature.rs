use crate::commons::{Charges, Reference};

#[derive(Debug)]
pub struct Feature {
    name: String,
    reference: Option<Reference>,
    source: String,
    given_by: FeatureOrigin,
    description: String,
    charges: Option<Charges>,
}

#[derive(Debug)]
pub enum FeatureOrigin {
    Species,
    Class,
    Feat,
    Item,
    Other,
}
