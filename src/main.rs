use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Coin {
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

#[derive(Debug, Deserialize)]
struct Market {
    current_price: HashMap<String, f32>,
    sparkline_7d: Sparkline
}

#[derive(Debug, Deserialize)]
struct Sparkline {
    price: Vec<f32>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://api.coingecko.com/api/v3/coins/bitcoin?tickers=false&market_data=true&community_data=false&developer_data=false&sparkline=true")
        .await?
        .json::<Coin>()
        .await?;
    println!("{:#?}", resp);

    println!("{}", resp.get_current_price("usd"));

    Ok(())
}
