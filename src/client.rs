use crate::models::{item::Item, orphanage::Orphanage, world_boss::WorldBosses};
use serde::{Deserialize, Serialize};

#[cfg(feature = "env")]
use dotenv::dotenv;

use std::{
    error::Error,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::models::{smmo_player::SmmoPlayer, SmmoModel};

pub struct SmmoClient {
    api_key: String,
    inner: reqwest::Client,
}

impl SmmoClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            inner: reqwest::Client::new(),
        }
    }

    #[cfg(feature = "env")]
    pub fn from_env() -> Self {
        dotenv().ok();
        Self::new(
            dotenv::var("SMMO_API_TOKEN").expect("SMMO_API_TOKEN environment variable not set."),
        )
    }

    pub async fn get_player_by_smmo_id(&self, smmo_id: String) -> SmmoResult<SmmoPlayer> {
        let url = format!("https://api.simple-mmo.com/v1/player/info/{}", smmo_id);
        self.get_internal(&url).await
    }

    pub async fn get_world_bosses<'a, 'b>(&'a self) -> SmmoResult<WorldBosses> {
        let url = "https://api.simple-mmo.com/v1/worldboss/all";
        self.get_internal(url).await
    }

    pub async fn get_orphanage<'a, 'b>(&'a self) -> SmmoResult<Orphanage> {
        let url = "https://api.simple-mmo.com/v1/orphanage";
        self.get_internal(url).await
    }

    pub async fn get_item_by_id<'a, 'b>(&'a self, id: u32) -> SmmoResult<Item> {
        let url = format!("https://api.simple-mmo.com/v1/item/info/{}", id);
        self.get_internal(&url).await
    }

    async fn get_internal<T: SmmoModel>(&self, url: &str) -> SmmoResult<T> {
        match self
            .inner
            .post(&*url)
            .query(&[("api_key", &*self.api_key)])
            .send()
            .await
        {
            Ok(res) => {
                let req_url = res.url().to_string();
                match res.text().await {
                    Ok(text) => {
                        let serde_result = serde_json::from_str::<InternalSmmoResult<T>>(&text);
                        match serde_result {
                            Ok(json) => json.into(),
                            Err(why) => {
                                log::error!(target: "smmo_api", "url: {}, error: {}", url, why.to_string());
                                Err(SmmoError::JsonDecodeError(text, req_url))
                            }
                        }
                    }
                    Err(why) => {
                        log::error!(target: "smmo_api", "url: {}, error: {}", url, why.to_string());
                        Err(SmmoError::ReqwestError(why))
                    }
                }
            }
            Err(why) => {
                log::error!(target: "smmo_api", "url: {}, error: {}", url, why.to_string());
                Err(SmmoError::InternalError)
            }
        }
    }
}

pub type SmmoResult<T> = Result<T, SmmoError<T>>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum InternalSmmoResult<T> {
    Ok(T),
    Err(SmmoError<T>),
}

impl<T: SmmoModel> From<InternalSmmoResult<T>> for Result<T, SmmoError<T>> {
    fn from(val: InternalSmmoResult<T>) -> Self {
        match val {
            InternalSmmoResult::Ok(ok) => Ok(ok),
            InternalSmmoResult::Err(err) => Err(err),
        }
    }
}

// impl<'s, T: SmmoModel<'s> + Debug> Error for SmmoResult<'s, T> {}

// impl<'s, T: SmmoModel<'s> + Display> Display for SmmoResult<'s, T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let string = match self {
//             SmmoResult::Ok(t) => t.to_string(),
//             Err(err) => err.to_string(),
//         };
//         f.write_str(&string)
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SmmoError<T /* : SmmoModel<'s> */> {
    // #[serde(rename = "error")]
    ApiError {
        error: ApiErrorType,
    },
    /// Something went wrong internally, check the logs.
    #[serde(skip)]
    InternalError,
    /// Unable to deserialize the api response; most likely means that the response structure changed.
    #[serde(skip)]
    JsonDecodeError(String, String),
    /// Something went wrong when fetching from the smmo api.
    #[serde(skip)]
    ReqwestError(reqwest::Error),
    /// Used to appease the typechecker. should never be constructed.
    #[serde(skip)]
    PhantomData(PhantomData<T>),
}

impl<T> SmmoError<T> {
    // pub(crate) fn unauthenticated
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ApiErrorType {
    /// Error from the api; means the item was not found.
    /// TODO: Include the invalid item id in the variant.
    #[serde(alias = "item not found")]
    ItemNotFound,
    /// Error from the api; means the api_key is not valid.
    #[serde(alias = "unauthenticated")]
    Unauthenticated,
}

impl<'s, T: SmmoModel> Display for SmmoError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}",
            match self {
                SmmoError::InternalError => "Something went wrong! Check the logs!".into(),
                SmmoError::JsonDecodeError(original, url) => format!(
                    r#"JSON decode error.
URL: `{}`
JSON recieved from the api: ```{}```
Expected type: {}
"#,
                    url,
                    original,
                    T::TYPE_NAME
                ),
                SmmoError::ReqwestError(error) =>
                    format!("Error with the SMMO api: ```{}```", error),
                SmmoError::PhantomData(_) => {
                    log::error!("PhantomData variant should never be constructed.");
                    unsafe { std::hint::unreachable_unchecked() }
                }
                // SmmoError::ItemNotFound => "Item not found".to_string(),
                SmmoError::ApiError { .. } => {
                    todo!()
                } // SmmoError::Unauthenticated => {
                  //     "Authentication error with the SMMO api. Check the api key.".into()
                  // }
            },
        ))
    }
}

impl<'s, T: SmmoModel + Debug> Error for SmmoError<T> {}

// #[cfg(try_trait)]
// #[feature(try_trait)]

// impl<'s, T: SmmoModel<'s>> Try for SmmoResult<'s, T> {
//     type Ok = T;

//     type Error = SmmoError<'s, T>;

//     fn into_result(self) -> Result<<SmmoResult<T> as Try>::Ok, Self::Error> {
//         match self {
//             SmmoResult::Ok(ok) => Ok(ok),
//             Err(err) => Err(err),
//         }
//     }

//     fn from_error(v: Self::Error) -> Self {
//         Err(v)
//     }

//     fn from_ok(v: <SmmoResult<T> as Try>::Ok) -> Self {
//         SmmoResult::Ok(v)
//     }
// }

#[cfg(test)]
mod test_internal_smmo_result_deserialize {
    use super::*;

    #[test]
    fn test_item_not_found() {
        let json = r#"{
            "error": "item not found"
        }"#;

        assert!(matches!(
            serde_json::from_str::<InternalSmmoResult<Orphanage>>(json).unwrap(),
            InternalSmmoResult::Err(SmmoError::ApiError {
                error: ApiErrorType::ItemNotFound
            }),
        ));
    }

    #[test]
    fn test_item_not_found_api_error() {
        let json = r#"{
            "error": "item not found"
        }"#;

        assert!(matches!(
            serde_json::from_str::<SmmoError<Orphanage>>(json).unwrap(),
            SmmoError::ApiError {
                error: ApiErrorType::ItemNotFound
            },
        ));
    }

    #[test]
    fn test_api_error_type_item_not_found() {
        let json = r#""item not found""#;
        assert_eq!(
            serde_json::from_str::<ApiErrorType>(json).unwrap(),
            ApiErrorType::ItemNotFound
        );
    }

    #[test]
    fn test_api_error_type_unauthenticated() {
        let json = r#""unauthenticated""#;
        assert_eq!(
            serde_json::from_str::<ApiErrorType>(json).unwrap(),
            ApiErrorType::Unauthenticated
        );
    }

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
        // assert_eq!(
        serde_json::from_str::<InternalSmmoResult<Item>>(json).unwrap();
        //     ,InternalSmmoResult::Ok(Item {
        //         id: ItemId(1),
        //         name: "Wooden Stick".to_string(),
        //         item_type: ItemType::Weapon,
        //         description: Some("".to_string()),
        //         equipable: true,
        //         level: 1,
        //         rarity: ItemRarity::Common,
        //         value: 20,
        //         stat1: Some(ItemStat::Str),
        //         stat1modifier: 1,
        //         stat2: None,
        //         stat2modifier: 0,
        //         stat3: None,
        //         stat3modifier: 0,
        //         custom_item: false,
        //         tradable: true,
        //         locked: false
        //     })
        // )
    }
}
