use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::models::smmo_player::UserId;

#[derive(Debug, Deserialize)]
pub struct Orphanage {
    current_amount: u64,
    max_amount: u64,
    recent_donators: Vec<RecentDonator>,
}

#[derive(Debug, Deserialize)]
pub struct RecentDonator {
    user_id: UserId,
    amount: u64,
    created_at: DateTime<Utc>,
}
