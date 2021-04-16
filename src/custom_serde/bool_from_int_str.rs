use serde::{
    de::{self, Unexpected},
    Deserializer,
    Deserialize,
};

pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match &*String::deserialize(deserializer)? {
        "0" => Ok(false),
        "1" => Ok(true),
        other => Err(de::Error::invalid_value(
            Unexpected::Str(other),
            &"zero or one",
        )),
    }
}
