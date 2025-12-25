use serde::{Deserialize, Deserializer};
use std::fmt;

pub type OrderList = Vec<VCIOderBook>;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VCIOderBook {
    pub id: u64,
    pub timestamp: String, // hh:mm:ss
    pub price: f64,
    pub volume: i64,
    pub match_type: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct OrderData {
    pub id: u64,
    pub symbol: String,
    #[serde(rename = "truncTime")]
    pub trunc_time: String,
    #[serde(rename = "matchType")]
    pub match_type: String,
    #[serde(rename = "matchVol")]
    #[serde(deserialize_with = "de_f64_from_str_or_num")]
    pub match_vol: f64,
    #[serde(rename = "matchPrice")]
    #[serde(deserialize_with = "de_f64_from_str_or_num")]
    pub match_price: f64,
    #[serde(rename = "accumulatedVolume")]
    #[serde(deserialize_with = "de_f64_from_str_or_num")]
    pub accumulated_volume: f64,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "accumulatedValue")]
    #[serde(deserialize_with = "de_f64_from_str_or_num")]
    pub accumulated_value: f64,
}

pub fn de_f64_from_str_or_num<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    struct F64Visitor;
    impl serde::de::Visitor<'_> for F64Visitor {
        type Value = f64;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a float or a string containing a float")
        }
        fn visit_f64<E>(self, value: f64) -> Result<f64, E> {
            Ok(value)
        }
        fn visit_str<E>(self, value: &str) -> Result<f64, E>
        where
            E: serde::de::Error,
        {
            value.parse::<f64>().map_err(E::custom)
        }
        fn visit_string<E>(self, value: String) -> Result<f64, E>
        where
            E: serde::de::Error,
        {
            value.parse::<f64>().map_err(E::custom)
        }
        fn visit_u64<E>(self, value: u64) -> Result<f64, E> {
            Ok(value as f64)
        }
        fn visit_i64<E>(self, value: i64) -> Result<f64, E> {
            Ok(value as f64)
        }
    }
    deserializer.deserialize_any(F64Visitor)
}
