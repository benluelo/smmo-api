use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub(crate) fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + Default,
    D: Deserializer<'de>,
{
    let v: Value = Deserialize::deserialize(deserializer)?;
    Ok(T::deserialize(v).unwrap_or_default())
}

#[cfg(test)]
mod test_ok_or_default_deserializing {
    use serde::Deserialize;

    #[derive(Deserialize, PartialEq, Eq, Debug)]
    struct Test {
        #[serde(deserialize_with = "super::deserialize")]
        test: Option<u8>,
    }

    #[test]
    fn test_successful() {
        assert_eq!(
            serde_json::from_str::<Test>(r#"{ "test": 1 }"#).unwrap(),
            Test { test: Some(1) }
        );
    }

    #[test]
    fn test_null_to_none() {
        assert_eq!(
            serde_json::from_str::<Test>(r#"{ "test": null }"#).unwrap(),
            Test { test: None }
        );
    }

    #[test]
    fn test_unsuccessfull_to_none() {
        assert_eq!(
            serde_json::from_str::<Test>(r#"{ "test": "whoops" }"#).unwrap(),
            Test { test: None }
        );
    }

    #[test]
    fn test_missing_fails() {
        assert!(serde_json::from_str::<Test>(r#"{}"#).is_err());
    }
}
