use derive_more::derive::IsVariant;
use serde::{Deserialize, Serialize};

/// Enum defining the 5 colors of magic, plus colorless.
#[derive(Default, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug, IsVariant)]
#[allow(missing_docs)]
#[repr(u8)]
pub enum Color {
    #[default]
    #[serde(rename = "C")]
    Colorless = 0,
    #[serde(rename = "W")]
    White = 1 << 0,
    #[serde(rename = "U")]
    Blue = 1 << 1,
    #[serde(rename = "B")]
    Black = 1 << 2,
    #[serde(rename = "R")]
    Red = 1 << 3,
    #[serde(rename = "G")]
    Green = 1 << 4,
}
