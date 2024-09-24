use crate::commons::{Charges, Reference};

pub struct Feature {
    name: String,
    reference: Option<Reference>,
    source: String,
    description: String,
    charges: Option<Charges>,
}
