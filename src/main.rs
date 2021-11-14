use serde::Deserialize;
use std::collections::HashMap;

use clap::{App, Arg};


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
    sparkline_7d: Sparkline,
}

#[derive(Debug, Deserialize)]
struct Sparkline {
    price: Vec<f32>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Crypto Price Checker")
        .version("1.0")
        .author("Lucas B. ")
        .about("Checks price of your portfolio and other crypto from coingecko.com")
        .arg(
            Arg::with_name("search")
                .help("Crypto you want to view")
                .takes_value(true)
                .short("s")
                .long("search")
                .multiple(true),
        )
        .get_matches();

    let crypto_names = matches.values_of("search").unwrap();

    for elem in crypto_names {
        println!("{}", elem);
    }

    let resp = reqwest::get("https://api.coingecko.com/api/v3/coins/bitcoin?tickers=false&market_data=true&community_data=false&developer_data=false&sparkline=true")
        .await?
        .json::<Coin>()
        .await?;
    println!("{:#?}", resp);

    println!("{}", resp.get_current_price("usd"));

    Ok(())
}
