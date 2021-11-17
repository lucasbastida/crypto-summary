use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct Record {
    pub name: String,
    pub amount: f32,
    location: Option<String>,
}



pub fn print_portfolio(records: &Vec<Record>) {
    for result in records.iter() {
        println!("{:?}", result);
    }
}

pub fn get_records(filename: &str) -> Vec<Record> {
    let file = File::open(filename).unwrap();
    let mut rdr = csv::Reader::from_reader(file);

    rdr.deserialize().map(|r| r.unwrap()).collect()
} 
