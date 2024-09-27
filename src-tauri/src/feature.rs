use crate::commons::{Charges, Reference};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    name: String,
    reference: Option<Reference>,
    source: String,
    given_by: FeatureOrigin,
    description: String,
    charges: Option<Charges>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeatureOrigin {
    Species,
    Class,
    Feat,
    Item,
    Other,
}
