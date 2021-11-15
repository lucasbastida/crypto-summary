use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    amount: f32,
    location: Option<String>,
}

pub fn print_portfolio(filename: &str) {
    let file = File::open(filename).unwrap();
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.deserialize() {
        let record: Record = result.unwrap();
        println!("{:?}", record);
    }
}
