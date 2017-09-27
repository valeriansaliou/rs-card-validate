rs-card-validate
================

[![Build Status](https://travis-ci.org/valeriansaliou/rs-card-validate.svg?branch=master)](https://travis-ci.org/valeriansaliou/rs-card-validate)

[Documentation](https://docs.rs/crate/card-validate)

[Crate](https://crates.io/crates/card-validate)

Detects and validates credit card numbers (type of card, number length and Luhn checksum).

**Important notice: this is a complete rework of [@rprotasov](https://github.com/rprotasov/creditcardvalidator) initial work, supporting more card providers and containing important validation fixes.**

## Supported providers

**Debit cards:**

* Visa Electron
* Maestro
* Forbrugsforeningen
* Dankort

**Credit cards:**

* Visa
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
card-validate = "2.1"
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
