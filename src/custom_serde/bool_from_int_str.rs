use serde::{
    de::{self, Unexpected},
    Deserialize, Deserializer,
};

pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let deserialized = String::deserialize(deserializer);
    match &*deserialized? {
        "0" => Ok(false),
        "1" => Ok(true),
        other => Err(de::Error::invalid_value(
            Unexpected::Str(other),
            &"zero or one",
        )),
    }
}

#[cfg(test)]
mod test_bool_from_int_str_deserializing {
    use serde::Deserialize;

    #[derive(Deserialize, PartialEq, Eq, Debug)]
    struct Test {
        #[serde(deserialize_with = "super::deserialize")]
        test: bool,
    }

    #[test]
    fn test_0_to_false() {
        assert_eq!(
            serde_json::from_str::<Test>(r#"{ "test": "0" }"#).unwrap(),
            Test { test: false }
        );
    }

    #[test]
    fn test_1_to_true() {
        assert_eq!(
            serde_json::from_str::<Test>(r#"{ "test": "1" }"#).unwrap(),
            Test { test: true }
        );
    }

    #[test]
    fn test_failure() {
        assert!(serde_json::from_str::<Test>(r#"{ "test": "3" }"#).is_err());
    }
}
