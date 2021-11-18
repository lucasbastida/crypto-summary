use clap::{App, Arg};
use std::collections::HashMap;

mod crytocurrency;
mod portfolio;

use serde::Deserialize;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

#[derive(Deserialize, Debug)]
struct MailerConfig {
    username: String,
    pw: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // command line parser
    // let matches = App::new("Crypto Price Checker")
    //     .version("1.0")
    //     .author("Lucas B. ")
    //     .about("Checks price of your portfolio and other crypto from coingecko.com")
    //     .arg(
    //         Arg::with_name("search")
    //             .help("Crypto you want to view")
    //             .takes_value(true)
    //             .short("s")
    //             .long("search")
    //             .multiple(true),
    //     )
    //     .get_matches();

    let matches = App::new("Crypto Price Checker")
        .version("1.0")
        .author("Lucas B. ")
        .about("Checks price of your portfolio and other crypto from coingecko.com")
        .arg(
            Arg::new("file")
                .about("file input")
                .takes_value(true)
                .short('f')
                .long("file"),
        )
        .subcommand(
            App::new("search").about("Search one or more crypto").arg(
                Arg::new("name")
                    .multiple_occurrences(true)
                    .takes_value(true)
                    .required(true),
            ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("search", search_matches)) => {
            // Now we have a reference to clone's matches
            let crypto_names = search_matches.values_of("name").unwrap().collect();
            search(crypto_names).await?;
        }
        None => println!("No subcommand was used"), // If no subcommand was used it'll match the tuple ("", None)
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }

    //-------------------PORTFOLIO--------------------------------
    let records: Vec<portfolio::Record> = portfolio::get_records("input/input.csv");

    //calculate value of record
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

        println!("{}", record_value);

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

    println! {"{:?}", sum};

    let total = format!("Total value: ${}", total);
    portfolio_string.push_str(&total);

    println!("{}", &total);

    println!("{}", portfolio_string);

    dotenv::dotenv().expect("Failed to read .env file");
    let config = match envy::prefixed("EMAIL_SMTP_").from_env::<MailerConfig>() {
        Ok(config) => {
            println!("{:?}", config);
            config
        }
        Err(e) => panic!("Couldn't read mailer config ({})", e),
    };

    let email = Message::builder()
        .from("NoBody <nobody@domain.tld>".parse().unwrap())
        .to("Hei <test@example.com>".parse().unwrap())
        .subject("Happy new year")
        .body(portfolio_string)
        .unwrap();

    let creds = Credentials::new(config.username, config.pw);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }

    Ok(())
}

async fn search(names: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    //load coin structs into a vector
    let coins = crytocurrency::load_crypto(names).await?;

    // print vector of coin value
    for elem in coins.iter() {
        println!("{}", elem);
    }

    Ok(())
}
