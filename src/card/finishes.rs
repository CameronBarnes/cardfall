use derive_more::derive::{Display, IsVariant};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize, Display, IsVariant)]
#[serde(rename_all = "lowercase")]
pub enum Finishes {
    Foil,
    NonFoil,
    Etched,
}
