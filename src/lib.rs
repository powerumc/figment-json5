//! A [`Format`] provider for JSON5.
//!
//! This crate provides a [Figment](https://docs.rs/figment/latest/figment/) provider for JSON5 files.
//! JSON5 is a superset of JSON that allows comments, trailing commas, and more.
//!
//! This provider uses the [`json5`] crate for parsing.
//!
//! # Example
//!
//! ```rust
//! use figment::Figment;
//! use figment::providers::Format;
//! use figment_json5::Json5;
//! use serde::Deserialize;
//!
//! #[derive(Deserialize)]
//! struct Config {
//!     name: String,
//!     description: String,
//!
//!     #[serde(rename = "leadingDecimalPoint")]
//!     leading_decimal_point: f64,
//!
//!     #[serde(rename = "andTrailing")]
//!     and_trailing: f64,
//!
//!     #[serde(rename = "positiveSign")]
//!     positive_sign: i32,
//!
//!     age: u32
//! }
//!
//! let config = r#"{
//!     // Allow comments
//!     "name": "powerumc",
//!     "age": 0x28, // Allow hexadecimal numbers
//!     "description": "This is a \
//! test config", // Allow multiline strings
//!     "leadingDecimalPoint": .8675309,
//!     "andTrailing": 8675309.,
//!     "positiveSign": +1,
//!     "address": "Seoul", // Allow trailing commas
//! }"#;
//!
//! let config: Config = Figment::new()
//!    .merge(Json5::string(config))
//!    .extract()
//!    .unwrap();
//!
//! assert_eq!(config.name, "powerumc");
//! assert_eq!(config.description, "This is a test config");
//! assert_eq!(config.leading_decimal_point, 0.8675309);
//! assert_eq!(config.and_trailing, 8675309.0);
//! assert_eq!(config.positive_sign, 1);
//! assert_eq!(config.age, 40);
//! ```

#[allow(unused_imports)]
use figment::providers::{Data, Format};
use figment::Provider;
use serde::de::DeserializeOwned;

#[allow(unused_macros)]
macro_rules! impl_format {
    ($name:ident $NAME:literal/$string:literal: $func:expr, $E:ty, $doc:expr) => (
        #[doc = $doc]
        pub struct $name;

        impl Format for $name {
            type Error = $E;

            const NAME: &'static str = $NAME;

            fn from_str<'de, T: DeserializeOwned>(s: &'de str) -> Result<T, $E> {
                $func(s)
            }
        }
    );

    ($name:ident $NAME:literal/$string:literal: $func:expr, $E:ty) => (
        impl_format!($name $NAME/$string: $func, $E, concat!(
            "A ", $NAME, " [`Format`] [`Data`] provider.",
            "\n\n",
            "Static constructor methods on `", stringify!($name), "` return a
            [`Data`] value with a generic marker of [`", stringify!($name), "`].
            Thus, further use occurs via methods on [`Data`].",
            "\n```\n",
            "\nuse figment::providers::Format;",
            "\nuse figment_json5::{", stringify!($name), "};",
            "\n\n// Source directly from a source string...",
            "\nlet provider = ", stringify!($name), r#"::string("source-string");"#,
            "\n\n// Or read from a file on disk.",
            "\nlet provider = ", stringify!($name), r#"::file("path-to-file");"#,
            "\n\n// Or configured as nested (via Data::nested()):",
            "\nlet provider = ", stringify!($name), r#"::file("path-to-file").nested();"#,
            "\n```",
            "\n\nSee also [`", stringify!($func), "`] for parsing details."
        ));
    )
}

impl_format!(Json5 "JSON5"/"json5": json5::from_str, json5::Error);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Json5;
    use figment::Figment;
    use serde::Deserialize;

    #[test]
    fn test_json5() {
        #[derive(Deserialize)]
        struct Config {
            name: String,
            description: String,

            #[serde(rename = "leadingDecimalPoint")]
            leading_decimal_point: f64,

            #[serde(rename = "andTrailing")]
            and_trailing: f64,

            #[serde(rename = "positiveSign")]
            positive_sign: i32,

            age: u32,
        }

        let config = r#"
            {
                // Allow comments
                "name": "powerumc",
                "age": 0x28, // Allow hexadecimal numbers
                "description": "This is a \
test config", // Allow multiline strings
                "leadingDecimalPoint": .8675309,
                "andTrailing": 8675309.,
                "positiveSign": +1,
                "address": "Seoul", // Allow trailing commas
            }
        "#;

        let config: Config = Figment::new()
            .merge(Json5::string(config))
            .extract()
            .unwrap();

        assert_eq!(config.name, "powerumc");
        assert_eq!(config.description, "This is a test config");
        assert_eq!(config.leading_decimal_point, 0.8675309);
        assert_eq!(config.and_trailing, 8675309.0);
        assert_eq!(config.positive_sign, 1);
        assert_eq!(config.age, 40);
    }
}
