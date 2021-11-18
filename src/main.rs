use clap::{App, Arg};
use std::collections::HashMap;

mod crytocurrency;
mod portfolio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // command line parser
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

    // grabbing cli search names
    let crypto_names = matches.values_of("search").unwrap().collect();

    //load coin structs into a vector
    let coins = crytocurrency::load_crypto(crypto_names).await?;

    // print vector of coin value
    for elem in coins.iter() {
        println!("{}", elem);
    }

    let mut records: Vec<portfolio::Record> = portfolio::get_records("input/input.csv");

    //calculate value of record
    let mut coins = HashMap::new();

    let mut total: f32 = 0.0;

    for record in records.iter() {
        let coin = crytocurrency::search_crypto(&record.name).await?;

        let value = coin.get_current_price("usd") * record.amount;

        println!(
            "Value of {}: {} USD in {}",
            coin.name, value, record.location
        );
        total += value;

        coins.insert(coin.name.clone().to_lowercase(), coin);
    }

    let mut sum = HashMap::new();

    for elem in records.iter() {
        let name = &elem.name;
        let counter = sum.entry(name).or_insert(0.0);

        *counter += elem.amount * coins.get(name).unwrap().get_current_price("usd");
    }

    println! {"{:?}", sum};

    println!("Total value: ${}", total);

    Ok(())
}
