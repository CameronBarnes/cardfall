use ahash::HashMap;
use card_face::CardFace;
use cfg_if::cfg_if;
use chrono::NaiveDate;
use color::Color;
use finishes::Finishes;
use image_status::ImageStatus;
use legality::Legalities;
use prices::Prices;
use rarity::Rarity;
use related_card::RelatedCard;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared_str::SharedStr;

pub mod rarity;
pub mod legality;
pub mod color;
pub mod finishes;
pub mod image_status;

#[allow(clippy::module_name_repetitions)]
pub mod related_card;
#[allow(clippy::module_name_repetitions)]
pub mod card_face;
pub mod prices;

cfg_if! {
    if #[cfg(feature = "shared_str")] {
        type StoredStr = SharedStr;
        type BorrowedStr = SharedStr;
        type OutStr = SharedStr;
    } else {
        type StoredStr = String;
        type BorrowedStr = &str;
        type OutStr = String;
    }
}

#[allow(
    clippy::struct_field_names,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools
)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Card {
    // Core fields
    id: Uuid,
    lang: StoredStr,
    layout: StoredStr,
    oracle_id: Option<Uuid>,
    // TODO: Consider changing these to actually be a URI
    prints_search_uri: String,
    rulings_uri: String,
    uri: String,
    // Gameplay fields
    all_parts: Option<Vec<RelatedCard>>,
    card_faces: Option<Vec<CardFace>>,
    cmc: Option<f32>,
    color_identity: Vec<Color>,
    color_indicator: Option<Vec<Color>>,
    colors: Option<Vec<Color>>,
    power: Option<StoredStr>,
    defence: Option<StoredStr>,
    toughness: Option<StoredStr>,
    edhrec_rank: Option<u64>,
    hand_modifier: Option<String>,
    keywords: Vec<StoredStr>,
    legalities: Legalities,
    life_modifier: Option<StoredStr>,
    loyalty: Option<StoredStr>,
    mana_cost: Option<StoredStr>,
    name: String,
    oracle_text: Option<String>,
    penny_rank: Option<u64>,
    produced_mana: Option<Vec<Color>>,
    reserved: bool,
    type_line: Option<StoredStr>,
    // Print Fields
    artist: Option<StoredStr>,
    artist_ids: Option<Vec<Uuid>>,
    booster: bool,
    border_color: StoredStr,
    card_back_id: Option<Uuid>,
    collector_number: String,
    content_warning: Option<bool>,
    digital: bool,
    finishes: Vec<Finishes>,
    flavor_name: Option<String>,
    flavor_text: Option<String>,
    frame_effects: Option<Vec<StoredStr>>,
    frame: StoredStr,
    full_art: bool,
    games: Vec<StoredStr>,
    highres_image: bool,
    illustration_id: Option<Uuid>,
    image_status: ImageStatus,
    image_uris: Option<Box<HashMap<StoredStr, String>>>,
    oversized: bool,
    prices: Prices,
    printed_name: Option<String>,
    printed_text: Option<String>,
    printed_type_line: Option<String>,
    promo: bool,
    promo_types: Option<Vec<StoredStr>>,
    purchase_uris: Option<Box<HashMap<StoredStr, String>>>,
    rarity: Rarity,
    related_uris: HashMap<StoredStr, String>,
    released_at: NaiveDate,
    reprint: bool,
    scryfall_set_uri: String,
    set_name: StoredStr,
    set_type: StoredStr,
    set_uri: String,
    set: StoredStr,
    set_id: Uuid,
    story_spotlight: bool,
    textless: bool,
    variation: bool,
    variation_of: Option<Uuid>,
    security_stamp: Option<StoredStr>,
    attraction_lights: Option<Vec<u8>>,
}
