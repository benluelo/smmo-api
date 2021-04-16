use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::SmmoModel;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct WorldBoss {
    pub id: u32,
    pub name: String,
    pub avatar: String,
    pub level: u32,
    pub god: u32,
    pub str: u32,
    pub def: u32,
    pub dex: u32,
    pub current_hp: u32,
    pub max_hp: u32,
    #[serde(with = "ts_seconds")]
    pub enable_time: DateTime<Utc>,
}

impl SmmoModel for WorldBoss {
    const TYPE_NAME: &'static str = "WorldBoss";
}
