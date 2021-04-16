use crate::{
    custom_serde::{bool_from_int, date_time_option::*},
    models::SmmoModel,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct SmmoPlayer {
    id: UserId,
    name: String,
    level: u32,
    motto: String,
    profile_number: String,
    exp: u32,
    gold: u32,
    steps: u32,
    npc_kills: u32,
    user_kills: u32,
    quests_complete: u32,
    dex: u32,
    def: u32,
    str: u32,
    bonus_dex: u32,
    bonus_def: u32,
    bonus_str: u32,
    hp: u32,
    max_hp: u32,
    #[serde(rename = "safeMode")]
    #[serde(deserialize_with = "bool_from_int::deserialize")]
    safe_mode: bool,
    #[serde(rename = "safeModeTime")]
    #[serde(
        deserialize_with = "deserialize_option_datefmt",
        serialize_with = "serialize_option_datefmt"
    )]
    safe_mode_time: Option<DateTime<Utc>>,
    background: u32,
    membership: u32,
    guild: Option<SmmoPlayerGuild>,
}

impl SmmoModel for SmmoPlayer {
    const TYPE_NAME: &'static str = "SmmoPlayer";
}

#[derive(Debug, Deserialize)]
pub struct UserId(u32);

#[derive(Debug, Deserialize)]
pub(crate) struct SmmoPlayerGuild {
    id: u32,
    name: String,
}
