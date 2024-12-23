use figment::providers::Format;
use figment::Figment;
use figment_json5::Json5;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
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

fn main() {
    let config: Config = Figment::new()
        .merge(Json5::file("./examples/config.json5"))
        .extract()
        .unwrap();

    println!("{:#?}", config)

    // print result
    /*
    Config {
        name: "powerumc",
        description: "This is a test config",
        leading_decimalPoint: 0.8675309,
        and_trailing: 8675309.0,
        positiveSign: 1,
        age: 40,
    }
        */
}
