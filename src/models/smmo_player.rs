use std::fmt::{self, Display};

use crate::{
    custom_serde::{bool_from_int, date_time_option::*},
    models::SmmoModel,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SmmoPlayer {
    pub id: UserId,
    pub name: String,
    pub level: u32,
    pub motto: String,
    pub profile_number: String,
    pub exp: u32,
    pub gold: u32,
    pub steps: u32,
    pub npc_kills: u32,
    pub user_kills: u32,
    pub quests_complete: u32,
    pub dex: u32,
    pub def: u32,
    pub str: u32,
    pub bonus_dex: u32,
    pub bonus_def: u32,
    pub bonus_str: u32,
    pub hp: u32,
    pub max_hp: u32,
    #[serde(rename = "safeMode")]
    #[serde(deserialize_with = "bool_from_int::deserialize")]
    pub safe_mode: bool,
    #[serde(rename = "safeModeTime")]
    #[serde(deserialize_with = "deserialize_option_datefmt")]
    pub safe_mode_time: Option<DateTime<Utc>>,
    pub background: u32,
    pub membership: u32,
    pub guild: Option<SmmoPlayerGuild>,
}

impl SmmoModel for SmmoPlayer {
    const TYPE_NAME: &'static str = "SmmoPlayer";
}

impl Display for SmmoPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&serde_json::to_string_pretty(&self).map_err(|_| fmt::Error)?)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserId(u32);

#[derive(Debug, Serialize, Deserialize)]
pub struct SmmoPlayerGuild {
    pub id: u32,
    pub name: String,
}
