use serde::{Serialize};

#[derive(Serialize)]
pub struct PokemonDescriptionResponse {
    pub(crate) name: String,
    pub(crate) description: String,
}