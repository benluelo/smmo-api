use crate::models::{orphanage::Orphanage, world_boss::WorldBosses};
use serde::Deserialize;

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
                            Err(_) => Err(SmmoError::JsonDecodeError(text, req_url)),
                        }
                    }
                    Err(why) => {
                        log::error!(target: "smmo_api", "url: {}, error: {}", url, why.to_string());
                        Err(SmmoError::ApiError(why))
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

// #[derive(Debug, Deserialize)]
// #[serde(untagged)] /* : SmmoModel<'s> */
pub type SmmoResult<T> = Result<T, SmmoError<T>>;

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SmmoError<T /* : SmmoModel<'s> */> {
    /// Error from the api; means the api_key is not valid.
    Unauthenticated,
    /// Something went wrong internally, check the logs.
    #[serde(skip)]
    InternalError,
    /// Unable to deserialize the api response; most likely means that the response structure changed.
    #[serde(skip)]
    JsonDecodeError(String, String),
    /// Something went wrong when fetching from the smmo api.
    #[serde(skip)]
    ApiError(reqwest::Error),
    /// Used to appease the typechecker. should never be constructed.
    #[serde(skip)]
    PhantomData(PhantomData<T>),
}

impl<'s, T: SmmoModel> Display for SmmoError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}",
            match self {
                SmmoError::Unauthenticated => {
                    "Authentication error with the SMMO api. Check the api key.".into()
                }
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

                SmmoError::ApiError(error) => format!("Error with the SMMO api: ```{}```", error),

                SmmoError::PhantomData(_) => {
                    log::error!("PhantomData variant should never be constructed.");
                    unsafe { std::hint::unreachable_unchecked() }
                }
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
