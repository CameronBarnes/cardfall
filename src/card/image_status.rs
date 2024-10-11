use derive_more::derive::{Display, IsVariant};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize, Default, Display, IsVariant,
)]
#[serde(rename_all = "snake_case")]
pub enum ImageStatus {
    #[default]
    Missing,
    Placeholder,
    Lowres,
    HighresScan,
}
