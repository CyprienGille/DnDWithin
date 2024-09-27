use crate::commons::{Charges, Reference};

#[derive(Debug, Clone)]
pub struct Feature {
    name: String,
    reference: Option<Reference>,
    source: String,
    given_by: FeatureOrigin,
    description: String,
    charges: Option<Charges>,
}

#[derive(Debug, Clone)]
pub enum FeatureOrigin {
    Species,
    Class,
    Feat,
    Item,
    Other,
}
