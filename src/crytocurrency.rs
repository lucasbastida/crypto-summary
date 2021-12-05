use serde::Deserialize;
use std::fmt;
use url::Url;

#[derive(Debug, Deserialize)]
struct Coin {
    name: String,
    symbol: String,
    market_data: Market,
}

#[derive(Debug, Deserialize)]
struct Market {
    current_price: UsdPrice,
}

#[derive(Debug, Deserialize)]
struct UsdPrice {
    usd: f32,
}

pub struct Crypto {
    pub name: String,
    pub symbol: String,
    pub current_price: f32,
}

impl From<Coin> for Crypto {
    fn from(i: Coin) -> Self {
        Self {
            name: i.name,
            symbol: i.symbol,
            current_price: i.market_data.current_price.usd,
        }
    }
}

impl fmt::Display for Crypto {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} ({}) Market price: {} USD",
            self.name.as_str(),
            self.symbol.as_str(),
            self.current_price,
        )
    }
}

fn get_coin_url() -> Url {
    let mut crypto_url = Url::parse("https://api.coingecko.com/").unwrap();
    crypto_url
        .query_pairs_mut()
        .append_pair("tickers", "false")
        .append_pair("market_data", "true")
        .append_pair("community_data", "false")
        .append_pair("developer_data", "false")
        .append_pair("sparkline", "false");

    crypto_url.set_path("api/v3/coins/empty");

    return crypto_url;
}

pub async fn load_crypto(
    crypto_names: Vec<&str>,
) -> Result<Vec<Crypto>, Box<dyn std::error::Error>> {
    // Creating Url struct
    let mut crypto_url = get_coin_url();

    let mut coins = Vec::new();

    // for each crypto, deserialize json and place in vec
    for elem in crypto_names {
        crypto_url
            .path_segments_mut()
            .map_err(|_| "cannot be base")?
            .pop()
            .push(elem);

        let resp = reqwest::get(crypto_url.as_str())
            .await?
            .json::<Coin>()
            .await?;

        coins.push(Crypto::from(resp));
    }

    Ok(coins)
}

pub async fn search_crypto(coin: &str) -> Result<Crypto, Box<dyn std::error::Error>> {
    let mut crypto_url = get_coin_url();
    crypto_url
        .path_segments_mut()
        .map_err(|_| "cannot be base")?
        .pop()
        .push(coin);

    let resp = reqwest::get(crypto_url.as_str())
        .await?
        .json::<Coin>()
        .await?;

    Ok(Crypto::from(resp))
}

pub async fn search(names: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    //load coin structs into a vector
    let coins = load_crypto(names).await?;

    // print vector of coin value
    for elem in coins.iter() {
        println!("{}", elem);
    }

    Ok(())
}
