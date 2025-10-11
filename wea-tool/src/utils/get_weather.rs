use std::{error::Error};

use serde_json::Value;

use super::constants::API_KEY;

pub async fn get_weather(city: &str) -> Result<Value, Box<dyn Error>> {
  let res = reqwest::get(format!(
    "https://restapi.amap.com/v3/weather/weatherInfo?key={}&city={}",
    API_KEY, city
  )).await?;
  let json = res.json::<Value>().await?;

  Ok(json)
}
