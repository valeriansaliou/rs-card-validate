//! card-validate detects and validates credit card numbers (type of card, number length and
//! Luhn checksum).

#![feature(inclusive_range, range_contains)]

#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate luhnmod10;

use std::ops::RangeInclusive;
use regex::Regex;

// The card formats have been copied from: https://github.com/faaez/creditcardutils/\
//   blob/master/src/creditcardutils.coffee
lazy_static! {
    static ref VISAELECTRON_PATTERN_REGEX: Regex = Regex::new(
        r"^4(026|17500|405|508|844|91[37])").unwrap();
    static ref MAESTRO_PATTERN_REGEX: Regex = Regex::new(
        r"^(5(018|0[23]|[68])|6(39|7))").unwrap();
    static ref FORBRUGSFORENINGEN_PATTERN_REGEX: Regex = Regex::new(r"^600").unwrap();
    static ref DANKORT_PATTERN_REGEX: Regex = Regex::new(r"^5019").unwrap();
    static ref VISA_PATTERN_REGEX: Regex = Regex::new(r"^4").unwrap();
    static ref MASTERCARD_PATTERN_REGEX: Regex = Regex::new(r"^(5[1-5]|2[2-7])").unwrap();
    static ref AMEX_PATTERN_REGEX: Regex = Regex::new(r"^3[47]").unwrap();
    static ref DINERSCLUB_PATTERN_REGEX: Regex = Regex::new(r"^3[0689]").unwrap();
    static ref DISCOVER_PATTERN_REGEX: Regex = Regex::new(r"^6([045]|22)").unwrap();
    static ref UNIONPAY_PATTERN_REGEX: Regex = Regex::new(r"^(62|88)").unwrap();
    static ref JCB_PATTERN_REGEX: Regex = Regex::new(r"^35").unwrap();
    static ref OTHER_PATTERN_REGEX: Regex = Regex::new(r"^[0-9]+$").unwrap();
}

/// Card type. Maps recognized cards, and validates their pattern and length.
#[derive(PartialEq)]
pub enum Type {
    // Debit
    VisaElectron,
    Maestro,
    Forbrugsforeningen,
    Dankort,

    // Credit
    Visa,
    MasterCard,
    Amex,
    DinersClub,
    Discover,
    UnionPay,
    JCB,

    #[doc(hidden)]
    __NonExhaustive,
}

/// Validate error. Maps possible validation errors (eg. card number format invalid).
#[derive(Debug, PartialEq)]
pub enum ValidateError {
    InvalidFormat,
    InvalidLength,
    InvalidLuhn,
    UnknownType,

    #[doc(hidden)]
    __NonExhaustive,
}

impl Type {
    pub fn name(&self) -> String {
        match *self {
            Type::VisaElectron => "visaelectron",
            Type::Maestro => "maestro",
            Type::Forbrugsforeningen => "forbrugsforeningen",
            Type::Dankort => "dankort",
            Type::Visa => "visa",
            Type::MasterCard => "mastercard",
            Type::Amex => "amex",
            Type::DinersClub => "dinersclub",
            Type::Discover => "discover",
            Type::UnionPay => "unionpay",
            Type::JCB => "jcb",
            _ => "other",
        }.to_string()
    }

    fn pattern<'a>(&self) -> &'a Regex {
        match *self {
            Type::VisaElectron => &*VISAELECTRON_PATTERN_REGEX,
            Type::Maestro => &*MAESTRO_PATTERN_REGEX,
            Type::Forbrugsforeningen => &*FORBRUGSFORENINGEN_PATTERN_REGEX,
            Type::Dankort => &*DANKORT_PATTERN_REGEX,
            Type::Visa => &*VISA_PATTERN_REGEX,
            Type::MasterCard => &*MASTERCARD_PATTERN_REGEX,
            Type::Amex => &*AMEX_PATTERN_REGEX,
            Type::DinersClub => &*DINERSCLUB_PATTERN_REGEX,
            Type::Discover => &*DISCOVER_PATTERN_REGEX,
            Type::UnionPay => &*UNIONPAY_PATTERN_REGEX,
            Type::JCB => &*JCB_PATTERN_REGEX,
            _ => &*OTHER_PATTERN_REGEX,
        }
    }

    fn length<'a>(&self) -> RangeInclusive<usize> {
        match *self {
            Type::VisaElectron => RangeInclusive { start: 16, end: 16 },
            Type::Maestro => RangeInclusive { start: 12, end: 19 },
            Type::Forbrugsforeningen => RangeInclusive { start: 16, end: 16 },
            Type::Dankort => RangeInclusive { start: 16, end: 16 },
            Type::Visa => RangeInclusive { start: 13, end: 16 },
            Type::MasterCard => RangeInclusive { start: 16, end: 16 },
            Type::Amex => RangeInclusive { start: 15, end: 15 },
            Type::DinersClub => RangeInclusive { start: 14, end: 14 },
            Type::Discover => RangeInclusive { start: 16, end: 16 },
            Type::JCB => RangeInclusive { start: 16, end: 16 },
            Type::UnionPay => RangeInclusive { start: 16, end: 19 },
            _ => RangeInclusive { start: 12, end: 19 },
        }
    }

    fn all() -> Vec<Type> {
        // Debit cards must come first, since they have more specific patterns than their\
        //   credit-card equivalents.
        vec![
            Type::VisaElectron,
            Type::Maestro,
            Type::Forbrugsforeningen,
            Type::Dankort,
            Type::Visa,
            Type::MasterCard,
            Type::Amex,
            Type::DinersClub,
            Type::Discover,
            Type::UnionPay,
            Type::JCB,
        ]
    }
}

/// Card validation utility. Used to validate a provided card number (length and Luhn checksum).
#[derive(PartialEq)]
pub struct Validate {
    pub card_type: Type,
}

impl Validate {
    pub fn from(card_number: &str) -> Result<Validate, ValidateError> {
        let card_type = Validate::evaluate_type(&card_number)?;

        if Validate::is_length_valid(&card_number, &card_type) == false {
            return Err(ValidateError::InvalidLength);
        }
        if Validate::is_luhn_valid(&card_number) == false {
            return Err(ValidateError::InvalidLuhn);
        }

        Ok(Validate {
            card_type: card_type,
        })
    }

    fn evaluate_type(card_number: &str) -> Result<Type, ValidateError> {
        // Validate overall card number structure
        if OTHER_PATTERN_REGEX.is_match(&card_number) {
            for card in Type::all() {
                // Validate brand-specific card number structure
                if card.pattern().is_match(&card_number) {
                    return Ok(card);
                }
            }

            Err(ValidateError::UnknownType)
        } else {
            Err(ValidateError::InvalidFormat)
        }
    }

    fn is_length_valid(card_number: &str, card_type: &Type) -> bool {
        card_type.length().contains(card_number.len())
    }

    fn is_luhn_valid(card_number: &str) -> bool {
        luhnmod10::valid(&card_number)
    }
}
