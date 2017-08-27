extern crate regex;

use regex::Regex;

pub enum Type {
    Visa,
    Discover,
    Amex,
    MasterCard,
    Other,
}

impl Type {
    pub fn name(&self) -> String {
        match *self {
            Type::Visa => "visa",
            Type::Discover => "discover",
            Type::Amex => "amex",
            Type::MasterCard => "mastercard",
            Type::Other => "other",
        }.to_string()
    }

    pub fn pattern(&self) -> Regex {
        Regex::new(match *self {
            Type::Visa => r"^4+[0-9]+$",
            Type::Discover => r"^[6011]+[0-9]+$",
            Type::Amex => r"^[37]+[0-9]+$",
            Type::MasterCard => r"^5+[1-5]+[0-9]+$",
            Type::Other => r"^[0-9]+$",
        }).unwrap()
    }

    pub fn length(&self) -> Regex {
        Regex::new(match *self {
            Type::Visa => r"^[0-9]{13}|[0-9]{16}$",
            Type::Discover => r"^[0-9]{16}$",
            Type::Amex => r"^[0-9]{15}$",
            Type::MasterCard => r"^[0-9]{16}$",
            Type::Other => r"^[0-9]{12,19}$",
        }).unwrap()
    }

    pub fn valid(&self) -> bool {
        match *self {
            Type::Other => false,
            _ => true,
        }
    }

    fn all() -> Vec<Type> {
        vec![Type::Visa, Type::Discover, Type::Amex, Type::MasterCard]
    }
}

pub struct Validate {
    pub card_type: Type,
    pub valid: bool,
    pub length_valid: bool,
    pub luhn_valid: bool,
}

impl Validate {
    pub fn new(card_number: &str) -> Validate {
        let card_type = Validate::evaluate_type(&card_number);
        let length_valid = Validate::is_length_valid(&card_number, &card_type);
        let luhn_valid = Validate::is_luhn_valid(&card_number);
        let valid = length_valid && luhn_valid && card_type.valid();

        Validate {
            card_type: card_type,
            valid: valid,
            length_valid: length_valid,
            luhn_valid: luhn_valid,
        }
    }

    fn evaluate_type(card_number: &str) -> Type {
        let mut card_type: Type = Type::Other;

        for card in Type::all() {
            match card.pattern().is_match(&card_number) {
                true => {
                    card_type = card;
                    break;
                }
                false => continue,
            }
        }

        return card_type;
    }

    fn is_length_valid(card_number: &str, card_type: &Type) -> bool {
        card_type.length().is_match(&card_number)
    }

    fn is_luhn_valid(card_number: &str) -> bool {
        Validate::calculate_luhn(&card_number) % 10 == 0
    }

    fn calculate_luhn(card_number: &str) -> i32 {
        let card_length = card_number.len();
        let mut digits = Vec::with_capacity(card_length);
        for digit in card_number.chars() {
            digits.push(digit as u8);
        }

        let mut odd: bool = true;
        let mut sum: i32 = 0;
        for index in card_length..0 {
            let digit = digits[index] as i32;

            sum += match odd {
                true => digit,
                false => digit * digit,
            };

            odd = !odd;
        }

        return sum;
    }
}
