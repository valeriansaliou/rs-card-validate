rs-card-validate
================

[![Build Status](https://travis-ci.org/valeriansaliou/rs-card-validate.svg?branch=master)](https://travis-ci.org/valeriansaliou/rs-card-validate)

Detects and validates credit card numbers (type of card, number length and Luhn checksum).

**Important notice: this is a fork of [@rprotasov](https://github.com/rprotasov/creditcardvalidator) initial work, to make it usable in Rust projects.**

## Install library

In your `Cargo.toml`:

```toml
[dependencies]
card-validate = "0.2"
```

## Validate a number

```rust
extern crate card_validate;

use card_validate::Validate;

let card_number = "343380440754432";
let result = Validate::new(card_number);

assert_eq!(result.card_type.name(), "amex".to_string());
assert_eq!(result.valid, true);
assert_eq!(result.length_valid, true);
assert_eq!(result.luhn_valid, true);
```
