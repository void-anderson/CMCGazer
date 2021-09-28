use reqwest::header::ACCEPT;
use std::path::Path;
use std::collections::HashMap;
use serde::{Deserialize};

// Data Structures
#[derive(Deserialize, Debug)]
struct Platform {
    id: i64,
    name: String,
    slug: String,
    symbol: String,
    token_address: String,
}
#[derive(Deserialize, Debug)]
struct CurrencyClass {
    fully_diluted_market_cap: f64,
    last_updated: String,
    market_cap: f64,
    market_cap_dominance: f64,
    percent_change_1h: f64,
    percent_change_24h: f64,
    percent_change_30d: f64,
    percent_change_60d: f64,
    percent_change_7d: f64,
    percent_change_90d: f64,
    price: f64, 
    volume_24h: f64,
}

#[derive(Deserialize, Debug)]
struct Quote {
    USD: CurrencyClass
}

#[derive(Deserialize, Debug)]
struct CoinItem {
    id: i64,
    circulating_supply: f64,
    max_supply: Option<f64>,
    total_supply: f64,
    cmc_rank: i64,
    num_market_pairs: i64,
    date_added: String,
    last_updated: String,
    name: String,
    platform: Option<Platform>,
    tags: Vec<String>,
    slug: String,
    symbol: String,
    quote: Quote,
}

#[derive(Deserialize, Debug)]
struct CoinList {
    data: Vec<CoinItem>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut settings = config::Config::default();
    let home = std::env::var("HOME").unwrap();
    let path_str = format!("{}/.gazer.toml", home);
    settings.merge(config::File::from(Path::new(&path_str)))
        .expect("Configuration File Not Found");
    let settings_map = settings.try_into::<HashMap<String, String>>().unwrap();

    let url = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/listings/latest";

    let mut output: String = String::from("");
    let watchlist: Vec<&str> = settings_map["watch_list"].split(",").collect();
    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .header(ACCEPT, "application/json")
        .header("X-CMC_PRO_API_KEY", &settings_map["api_key"])
        .query(&[
           ("start", &settings_map["start"]),
           ("limit", &settings_map["limit"]),
           ("convert", &settings_map["base_currency"])
        ])
        .send()
        .await?
        .json::<CoinList>()
        .await?;

    for symbol in watchlist.iter() {
        let item: &CoinItem = resp.data.iter().find(|&x| &x.symbol == symbol).unwrap();
        let out = format!("{}: {:.2} ", symbol, item.quote.USD.price);
        output.push_str(&out);
    }
    println!("{}", output);
    Ok(())
}
