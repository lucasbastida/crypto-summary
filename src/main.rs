use clap::{App, Arg};

mod crytocurrency;
mod email;
mod portfolio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Crypto Price Checker")
        .version("1.0")
        .about("Checks price of your portfolio and other crypto from coingecko.com")
        .arg(
            Arg::new("file")
                .about("file input location")
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
            crytocurrency::search(crypto_names).await?;
        }
        None => {
            let file = match matches.value_of("file") {
                Some(file) => {
                    print!("Using file {}", file);
                    file
                }
                None => {
                    println!("Using file input/input.csv");
                    "input/input.csv"
                }
            };

            let records: Vec<portfolio::Record> = portfolio::get_records(file);
            let portfolio_string = portfolio::records_summary(records).await?;

            // email
            let email = email::create_email(portfolio_string);
            email::send_email(email);
        } // If no subcommand was used it'll match the tuple ("", None)
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }

    Ok(())
}


