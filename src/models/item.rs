use core::fmt;
use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    custom_serde::{bool_from_int, bool_from_int_str, ok_or_default},
    models::SmmoModel,
};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Item {
    pub id: ItemId,

    pub name: String,

    #[serde(rename = "type")]
    pub item_type: ItemType,

    pub description: Option<String>,

    #[serde(deserialize_with = "bool_from_int_str::deserialize")]
    pub equipable: bool,

    pub level: u64,

    pub rarity: ItemRarity,

    pub value: u64,

    #[serde(deserialize_with = "ok_or_default::deserialize")]
    pub stat1: Option<ItemStat>,

    #[serde(default)]
    pub stat1modifier: u32,

    #[serde(deserialize_with = "ok_or_default::deserialize")]
    pub stat2: Option<ItemStat>,

    #[serde(default)]
    pub stat2modifier: u32,

    #[serde(deserialize_with = "ok_or_default::deserialize")]
    pub stat3: Option<ItemStat>,

    #[serde(default)]
    pub stat3modifier: u32,

    #[serde(deserialize_with = "bool_from_int::deserialize")]
    pub custom_item: bool,

    #[serde(deserialize_with = "bool_from_int::deserialize")]
    pub tradable: bool,

    #[serde(deserialize_with = "bool_from_int::deserialize")]
    pub locked: bool,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Deserialize, Serialize)]
pub struct ItemId(u64);

impl SmmoModel for Item {
    const TYPE_NAME: &'static str = "Item";
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&serde_json::to_string_pretty(&self).map_err(|_| fmt::Error)?)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ItemType {
    Weapon,
    Helmet,
    Amulet,
    Armour,
    Shield,
    Greaves,
    Boots,
    Special,
    Pet,
    #[serde(rename(deserialize = "Wood Axe"))]
    WoodAxe,
    Pickaxe,
    #[serde(rename(deserialize = "Fishing Rod"))]
    FishingRod,
    Shovel,
    Material,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Elite,
    Legendary,
    Exotic,
    Celestial,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Deserialize, Serialize)]
#[serde(untagged)]
#[serde(rename_all = "lowercase")]
pub enum ItemStat {
    Str,
    Def,
    Dex,
    Crit,
}
