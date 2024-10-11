use ahash::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{color::Color, StoredStr};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CardFace {
    artist: Option<StoredStr>,
    artist_id: Option<Uuid>,
    cmc: Option<f32>,
    #[serde(default)]
    color_indicator: Vec<Color>,
    #[serde(default)]
    colors: Vec<Color>,
    power: Option<StoredStr>,
    defence: Option<StoredStr>,
    toughness: Option<StoredStr>,
    flavor_text: Option<String>,
    illustration_id: Option<Uuid>,
    image_uris: Option<Box<HashMap<StoredStr, String>>>,
    layout: Option<StoredStr>,
    loyalty: Option<StoredStr>,
    mana_cost: StoredStr,
    name: String,
    oracle_id: Option<Uuid>,
    oracle_text: Option<String>,
    printed_name: Option<String>,
    printed_text: Option<String>,
    printed_type_line: Option<String>,
    type_line: Option<StoredStr>,
    //watermark: Option<SharedStr>,
}
