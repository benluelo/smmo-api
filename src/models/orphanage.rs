use std::fmt::{self, Display};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::{smmo_player::UserId, SmmoModel};

#[derive(Debug, Serialize, Deserialize)]
pub struct Orphanage {
    current_amount: u64,
    max_amount: u64,
    recent_donators: Vec<RecentDonator>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentDonator {
    user_id: UserId,
    amount: u64,
    created_at: DateTime<Utc>,
}

impl SmmoModel for Orphanage {
    const TYPE_NAME: &'static str = "Orphanage";
}

impl Display for Orphanage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&serde_json::to_string_pretty(&self).map_err(|_| fmt::Error)?)
    }
}
