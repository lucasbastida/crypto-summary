
# Crypto 
Checks price of your portfolio and other cryptocurrencies from coingecko.com. Created to practice Rust.


## Usage

```console
$ crypto-summary --help

Crypto Price Checker 1.0

Checks price of your portfolio and other crypto from coingecko.com

USAGE:
    crypto-summary [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -e, --email <email>    who to send your summary
    -f, --file <file>      file input location
    -h, --help             Print help information
    -V, --version          Print version information

SUBCOMMANDS:
    help      Print this message or the help of the given subcommand(s)
    search    Search one or more crypto
```

## Examples

```console 
$ crypto-summary search bitcoin litecoin cardano solana
Bitcoin (btc) Market price: 58244 USD
Litecoin (ltc) Market price: 219.65 USD
Cardano (ada) Market price: 1.87 USD
Solana (sol) Market price: 214.98 USD
```

```console
$ crypto-summary -f input/input.csv
Using file input/input.csv
-----portfolio------
Value of Bitcoin: 581.17 USD in 
Value of Bitcoin: 581.17 USD in moon
Value of Solana: 214.93 USD in wallet
Value of Solana: 429.86 USD in 
Total value of bitcoin : 1162.34
Total value of solana : 644.79
Total value: $1807.13
--------------------
```

### Sending an email
Uses SMTP. You need to update your .env file with email sender user and password.

example:
```
EMAIL_SMTP_USERNAME=sender@example.com
EMAIL_SMTP_PW=1234
```

If using a google account, you will need to [set up google SMTP server](https://support.google.com/a/answer/176600?hl=en#zippy=%2Cuse-the-gmail-smtp-server) by creating and using app specific password or turing on less secure apps


```console
$ crypto-summary -f input/input.csv -e email@example.com
Using file input/input.csv
-----portfolio------
Value of Bitcoin: 580.93 USD in 
Value of Bitcoin: 580.93 USD in moon
Value of Solana: 214.93 USD in wallet
Value of Solana: 429.86 USD in 
Total value of bitcoin : 1161.86
Total value of solana : 644.79
Total value: $1806.65
--------------------
MailerConfig { username: "email@sender.com", pw: "1234" }
Email sent successfully!
```

## Bugs
Sometimes gets stuck when retrieving crypto info from coingecko

## Attribution

[![Coingecko logo](https://static.coingecko.com/s/coingecko-logo-white-3f2aeb48e13428b7199395259dbb96280bf47ea05b2940ef7d3e87c61e4d8408.png)](https://www.coingecko.com/en/api/documentation)