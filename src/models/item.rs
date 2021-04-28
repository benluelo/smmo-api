use core::fmt;
use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    custom_serde::{bool_from_int, bool_from_int_str, empty_string_option, ok_or_default},
    models::SmmoModel,
};

// #[cfg(feature = "sql")]
// use sqlx;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "sql", derive(sqlx::FromRow))]
pub struct Item {
    pub id: ItemId,

    pub name: String,

    #[cfg_attr(feature = "sql", sqlx(rename = "type"))]
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

// #[repr(transparent)]
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
    #[cfg_attr(feature = "sql", sqlx(rename = "Event Item"))]
    #[serde(rename = "Event Item")]
    EventItem,
}

impl Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            ItemType::Weapon => "Weapon",
            ItemType::Helmet => "Helmet",
            ItemType::Amulet => "Amulet",
            ItemType::Armour => "Armour",
            ItemType::Shield => "Shield",
            ItemType::Greaves => "Greaves",
            ItemType::Boots => "Boots",
            ItemType::Special => "Special",
            ItemType::Pet => "Pet",
            ItemType::WoodAxe => "WoodAxe",
            ItemType::Pickaxe => "Pickaxe",
            ItemType::FishingRod => "FishingRod",
            ItemType::Shovel => "Shovel",
            ItemType::Material => "Material",
            ItemType::Food => "Food",
            ItemType::Other => "Other",
            ItemType::Collectable => "Collectable",
            ItemType::Avatar => "Avatar",
            ItemType::Sprite => "Sprite",
            ItemType::ItemSprite => "ItemSprite",
            ItemType::Grenade => "Grenade",
            ItemType::Book => "Book",
            ItemType::Background => "Background",
            ItemType::Diamonds => "Diamonds",
            ItemType::EventItem => "EventItem",
        })
    }
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

impl Display for ItemRarity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            ItemRarity::Common => "Common",
            ItemRarity::Uncommon => "Uncommon",
            ItemRarity::Rare => "Rare",
            ItemRarity::Epic => "Epic",
            ItemRarity::Elite => "Elite",
            ItemRarity::Legendary => "Legendary",
            ItemRarity::Exotic => "Exotic",
            ItemRarity::Celestial => "Celestial",
        })
    }
}

impl ItemRarity {
    pub fn colour_hex(&self) -> u32 {
        match self {
            ItemRarity::Common => 0x34495E,
            ItemRarity::Uncommon => 0x2980B9,
            ItemRarity::Rare => 0xE67E22,
            ItemRarity::Epic => 0x8E44AD,
            ItemRarity::Elite => 0xC0392B,
            ItemRarity::Legendary => 0xF1C40F,
            ItemRarity::Exotic => 0x27AE60,
            ItemRarity::Celestial => 0x00F6FF,
        }
    }

    pub fn colour_rgb(&self) -> (u8, u8, u8) {
        let hex = self.colour_hex();

        let r = ((hex >> 16) & 255) as u8;
        let g = ((hex >> 8) & 255) as u8;
        let b = (hex & 255) as u8;

        (r, g, b)
    }
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
                description: None,
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
