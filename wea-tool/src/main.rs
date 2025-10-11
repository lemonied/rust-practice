mod utils;

use std::error::Error;

use clap::Parser;
use utils::{get_weather, Args, GetValue};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let args = Args::parse();
  let result = get_weather(&args.city.expect("no city")).await?;
  let getter = GetValue::new(&result)
    .chain("lives")
    .chain(0);

  let province = getter.chain("province").to_str();
  let city = getter.chain("city").to_str();
  let weather = getter.chain("weather").to_str();
  let temperature_float = getter.chain("temperature_float").to_str();
  let reporttime = getter.chain("reporttime").to_str();

  println!(
    "省份:{province} 城市:{city}\n天气:{weather} 温度:{temperature_float}\n时间:{reporttime}"
  );

  Ok(())
}
