
# Crypto 
Checks price of your portfolio (WIP) and other cryptocurrencies from coingecko.com.


## Usage

```console
$ crypto-summary --help

USAGE:
    crypto-summary [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --search <search>...    Crypto you want to view
```

## Examples

```console 
$ crypto-summary -s cardano bitcoin litecoin

Cardano
Symbol: ada
Market price: 2.03 USD
Bitcoin
Symbol: btc
Market price: 64429 USD
Litecoin
Symbol: ltc
Market price: 270.04 USD
```

## Attribution

[![Coingecko logo](https://static.coingecko.com/s/coingecko-logo-white-3f2aeb48e13428b7199395259dbb96280bf47ea05b2940ef7d3e87c61e4d8408.png)](https://www.coingecko.com/en/api/documentation)