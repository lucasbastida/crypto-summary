use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use url::{ParseError, Url};

#[derive(Debug, Deserialize)]
pub struct Coin {
    name: String,
    symbol: String,
    market_data: Market,
}

impl Coin {
    fn get_current_price(&self, currency: &str) -> f32 {
        *self.market_data.current_price.get(currency).unwrap()
    }

    fn get_7d(&self) -> &Vec<f32> {
        &self.market_data.sparkline_7d.price
    }
}

impl fmt::Display for Coin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\nSymbol: {}\nMarket price: {} USD",
            self.name.as_str(),
            self.symbol.as_str(),
            self.get_current_price("usd")
        )
    }
}

#[derive(Debug, Deserialize)]
struct Market {
    current_price: HashMap<String, f32>,
    sparkline_7d: Sparkline,
}

#[derive(Debug, Deserialize)]
struct Sparkline {
    price: Vec<f32>,
}

pub async fn load_crypto(crypto_names: Vec<&str>) -> Result<Vec<Coin>, Box<dyn std::error::Error>> {
    // Creating Url struct
    let mut crypto_url = Url::parse("https://api.coingecko.com/").unwrap();
    crypto_url
        .query_pairs_mut()
        .append_pair("tickers", "false")
        .append_pair("market_data", "true")
        .append_pair("community_data", "false")
        .append_pair("developer_data", "false")
        .append_pair("sparkline", "true");

    let mut coins = Vec::new();

    // for each crypto, deserialize json and place in vec
    for elem in crypto_names {
        let mut path = String::from("api/v3/coins/");
        path.push_str(elem);

        crypto_url.set_path(&path);

        let resp = reqwest::get(crypto_url.as_str())
            .await?
            .json::<Coin>()
            .await?;

        coins.push(resp);
    }

    Ok(coins)
}
