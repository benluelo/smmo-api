use core::fmt;
use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    custom_serde::{bool_from_int, bool_from_int_str, empty_string_option, ok_or_default},
    models::SmmoModel,
};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Eq, PartialOrd, Ord)]
pub struct Item {
    pub id: ItemId,

    pub name: String,

    #[serde(rename = "type")]
    pub item_type: ItemType,

    #[serde(deserialize_with = "empty_string_option::deserialize")]
    pub description: Option<String>,

    #[serde(deserialize_with = "bool_from_int_str::deserialize")]
    pub equipable: bool,

    pub level: u32,

    pub rarity: ItemRarity,

    pub value: u32,

    #[serde(deserialize_with = "ok_or_default::deserialize")]
    pub stat1: Option<ItemStat>,

    #[serde(deserialize_with = "ok_or_default::deserialize")]
    pub stat1modifier: u32,

    #[serde(deserialize_with = "ok_or_default::deserialize")]
    pub stat2: Option<ItemStat>,

    #[serde(deserialize_with = "ok_or_default::deserialize")]
    pub stat2modifier: u32,

    #[serde(deserialize_with = "ok_or_default::deserialize")]
    pub stat3: Option<ItemStat>,

    #[serde(deserialize_with = "ok_or_default::deserialize")]
    pub stat3modifier: u32,

    #[serde(deserialize_with = "bool_from_int::deserialize")]
    pub custom_item: bool,

    #[serde(deserialize_with = "bool_from_int::deserialize")]
    pub tradable: bool,

    #[serde(deserialize_with = "bool_from_int::deserialize")]
    pub locked: bool,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Deserialize, Serialize)]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(transparent))]
pub struct ItemId(u32);

impl ItemId {
    pub fn inner(self) -> u32 {
        self.0
    }
}

impl SmmoModel for Item {
    const TYPE_NAME: &'static str = "Item";
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&serde_json::to_string_pretty(&self).map_err(|_| fmt::Error)?)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Deserialize, Serialize)]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(type_name = "item_type"))]
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
    #[cfg_attr(feature = "sql", sqlx(rename = "Wood Axe"))]
    #[serde(rename = "Wood Axe")]
    WoodAxe,
    Pickaxe,
    #[cfg_attr(feature = "sql", sqlx(rename = "Fishing Rod"))]
    #[serde(rename = "Fishing Rod")]
    FishingRod,
    Shovel,
    Material,
    Food,
    Other,
    Collectable,
    Avatar,
    Sprite,
    #[cfg_attr(feature = "sql", sqlx(rename = "Item Sprite"))]
    #[serde(rename = "Item Sprite")]
    ItemSprite,
    // see: item 2087
    Grenade,
    Book,
    Background,
    // see: item 12653
    Diamonds,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Deserialize, Serialize)]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(type_name = "item_rarity"))]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    #[serde(alias = "Elilte")]
    Elite,
    #[serde(alias = "Lengendary")]
    Legendary,
    Exotic,
    Celestial,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Deserialize, Serialize)]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[cfg_attr(feature = "sql", sqlx(rename_all = "lowercase"))]
#[cfg_attr(feature = "sql", sqlx(type_name = "item_stat"))]
#[serde(rename_all = "lowercase")]
pub enum ItemStat {
    Str,
    Def,
    Dex,
    Crit,
    Hp,
}

#[cfg(test)]
mod test_item_deserialize {
    use super::*;

    #[test]
    fn test_item() {
        let json = r#"{
            "id": 1,
            "name": "Wooden Stick",
            "type": "Weapon",
            "description": "",
            "equipable": "1",
            "level": 1,
            "rarity": "Common",
            "value": 20,
            "stat1": "str",
            "stat1modifier": 1,
            "stat2": null,
            "stat2modifier": 0,
            "stat3": null,
            "stat3modifier": null,
            "custom_item": 0,
            "tradable": 1,
            "locked": 0
        }"#;
        assert_eq!(
            serde_json::from_str::<Item>(json).unwrap(),
            Item {
                id: ItemId(1),
                name: "Wooden Stick".to_string(),
                item_type: ItemType::Weapon,
                description: Some("".to_string()),
                equipable: true,
                level: 1,
                rarity: ItemRarity::Common,
                value: 20,
                stat1: Some(ItemStat::Str),
                stat1modifier: 1,
                stat2: None,
                stat2modifier: 0,
                stat3: None,
                stat3modifier: 0,
                custom_item: false,
                tradable: true,
                locked: false
            }
        )
    }

    #[test]
    fn test_item_type() {
        let json = r#""Weapon""#;
        assert_eq!(
            serde_json::from_str::<ItemType>(json).unwrap(),
            ItemType::Weapon
        )
    }

    #[test]
    fn test_item_type_ser() {
        let json = r#""Weapon""#;
        assert_eq!(
            serde_json::to_string::<ItemType>(&ItemType::Weapon).unwrap(),
            json
        )
    }
}
