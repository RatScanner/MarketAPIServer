#[derive(Debug, serde::Deserialize)]
pub struct MarketItem {
    #[serde(rename = "bsgId")]
    pub uid: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "shortName")]
    pub short_name: String,

    #[serde(rename = "slots")]
    // TODO remove when api gets fixed
    #[serde(deserialize_with = "helper::deserialize_number_from_string")]
    pub slots: i32,

    #[serde(rename = "price")]
    pub price: i32,

    #[serde(rename = "avg24hPrice")]
    pub avg_24h_price: i32,

    #[serde(rename = "avg7daysPrice")]
    pub avg_7d_price: i32,

    #[serde(rename = "diff24h")]
    pub diff24h: f32,

    #[serde(rename = "diff7days")]
    pub diff7days: f32,

    #[serde(rename = "traderName")]
    pub trader_name: String,

    #[serde(rename = "traderPrice")]
    pub trader_price: i32,

    #[serde(rename = "traderPriceCur")]
    pub trader_currency: String,

    #[serde(rename = "wikiLink")]
    pub wiki_link: String,

    #[serde(rename = "img")]
    #[serde(deserialize_with = "helper::deserialize_img")]
    pub img_link: String,

    #[serde(rename = "updated")]
    #[serde(deserialize_with = "helper::deserialize_timestamp_from_date")]
    pub timestamp: i32,
}

mod helper {
    use std::fmt::Display;
    use std::str::FromStr;

    use serde::{Deserialize, Deserializer};

    pub fn deserialize_number_from_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr + serde::Deserialize<'de>,
        <T as FromStr>::Err: Display,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrInt<T> {
            String(String),
            Number(T),
        }

        match StringOrInt::<T>::deserialize(deserializer)? {
            StringOrInt::String(s) => s.parse::<T>().map_err(serde::de::Error::custom),
            StringOrInt::Number(i) => Ok(i),
        }
    }

    pub fn deserialize_img<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut img = String::deserialize(deserializer)?;

        let patterns = [".PNG", ".png", ".GIF", ".gif", ".JPG", ".jpg"];
        for pat in &patterns {
            if let Some(pos) = img.find(pat) {
                img = img[..pos + pat.len()].to_string();
                break;
            }
        }

        Ok(img.replace("/thumb", ""))
    }

    pub fn deserialize_timestamp_from_date<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(chrono::DateTime::<chrono::Utc>::deserialize(deserializer)?.timestamp() as i32)
    }
}
