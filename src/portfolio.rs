use serde::Deserialize;
use std::fs::File;

use std::collections::HashMap;

use crate::{crytocurrency};


#[derive(Debug, Deserialize)]
pub struct Record {
    pub name: String,
    pub amount: f32,
    #[serde(default)]
    pub location: String,
}

pub async fn records_summary(records: Vec<Record>) -> Result<String, Box<dyn std::error::Error>> {
    let mut coins = HashMap::new();

    let mut portfolio_string = String::new();

    let mut total: f32 = 0.0;

    for record in records.iter() {
        let coin = crytocurrency::search_crypto(&record.name).await?;

        let value = coin.get_current_price("usd") * record.amount;

        let record_value = format!(
            "Value of {}: {} USD in {}",
            coin.name, value, record.location
        );


        portfolio_string.push_str(&record_value);
        portfolio_string.push_str("\n");

        total += value;

        coins.insert(coin.name.clone().to_lowercase(), coin);
    }

    let mut sum = HashMap::new();

    for elem in records.iter() {
        let name = &elem.name;
        let counter = sum.entry(name).or_insert(0.0);

        *counter += elem.amount * coins.get(name).unwrap().get_current_price("usd");
    }

    for (key, value) in sum.iter() {
        portfolio_string.push_str(&format!("Total value of {} : {}\n", key, value));
    }


    let total = format!("Total value: ${}", total);
    portfolio_string.push_str(&total);


    println!("-----portfolio------");
    println!("{}", portfolio_string);
    println!("--------------------");

    Ok(portfolio_string)
}

pub fn get_records(filename: &str) -> Vec<Record> {
    let file = File::open(filename).unwrap();
    let mut rdr = csv::Reader::from_reader(file);

    rdr.deserialize().map(|r| r.unwrap()).collect()
}
