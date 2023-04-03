rs-card-validate
================

[![Test and Build](https://github.com/valeriansaliou/rs-card-validate/workflows/Test%20and%20Build/badge.svg?branch=master)](https://github.com/valeriansaliou/rs-card-validate/actions?query=workflow%3A%22Test+and+Build%22) [![Build and Release](https://github.com/valeriansaliou/rs-card-validate/workflows/Build%20and%20Release/badge.svg)](https://github.com/valeriansaliou/rs-card-validate/actions?query=workflow%3A%22Build+and+Release%22) [![Buy Me A Coffee](https://img.shields.io/badge/buy%20me%20a%20coffee-donate-yellow.svg)](https://www.buymeacoffee.com/valeriansaliou)

[Documentation](https://docs.rs/crate/card-validate)

[Crate](https://crates.io/crates/card-validate)

Detects and validates credit card numbers (type of card, number length and Luhn checksum).

**Important notice: this is a complete rework of [@rprotasov](https://github.com/rprotasov/creditcardvalidator) initial work, supporting more card providers and containing important validation fixes.**

**🇫🇷 Crafted in Brest, France.**

## Supported providers

**Debit cards:**

* Visa Electron
* Maestro
* Forbrugsforeningen
* Dankort

**Credit cards:**

* Visa
* MIR
* MasterCard
* American Express
* Diners Club
* Discover
* UnionPay
* JCB

## Install library

In your `Cargo.toml`:

```toml
[dependencies]
card-validate = "2.3"
```

## Validate a number

```rust
extern crate card_validate;

use card_validate::Validate;

let card_number = "5236313877109142";

match Validate::from(card_number) {
  Ok(result) => println!("Card type is: {}", result.card_type.name()),
  Err(err) => println!("Card is invalid: {:?}", err)
}
```
