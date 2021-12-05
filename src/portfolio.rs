use serde::Deserialize;
use std::fs::File;

use std::collections::HashMap;

use crate::crytocurrency;

#[derive(Debug, Deserialize)]
pub struct Record {
    pub name: String,
    pub amount: f32,
    #[serde(default)]
    pub location: String,
}

pub fn get_records(filename: &str) -> Vec<Record> {
    let file = File::open(filename).unwrap();
    let mut rdr = csv::Reader::from_reader(file);

    rdr.deserialize().map(|r| r.unwrap()).collect()
}

pub async fn records_summary(records: Vec<Record>) -> Result<String, Box<dyn std::error::Error>> {
    let mut sum = HashMap::new();

    let mut crypto_prices = String::new();
    let mut total: f32 = 0.0;

    for record in records.iter() {
        let coin = crytocurrency::get_crypto(&record.name).await?;

        let record_value = coin.current_price * record.amount;

        let name = &record.name;
        let total_record = sum.entry(name).or_insert(0.0);
        *total_record += record_value;

        total += record_value;

        //adding current crypto value
        if record.location.is_empty() {
            crypto_prices.push_str(
                format!(
                    "Value of {} in unknown: {} USD\n",
                    coin.name, record_value
                )
                .as_str(),
            );
        } else {
            crypto_prices.push_str(
                format!(
                    "Value of {} in {}: {} USD\n",
                    coin.name, record.location, record_value
                )
                .as_str(),
            );
        }
    }

    let mut portfolio_value = String::new();
    for (key, value) in sum.iter() {
        portfolio_value.push_str(&format!("Total {} value: {} USD\n", key, value));
    }

    let total = format!("Total value: ${}\n", total);

    let mut out_string = total;
    out_string.push_str("---------\n");
    out_string += &portfolio_value;
    out_string.push_str("---------\n");
    out_string += &crypto_prices;

    println!("-----portfolio------");
    println!("{}", out_string);
    println!("--------------------");

    Ok(out_string)
}
