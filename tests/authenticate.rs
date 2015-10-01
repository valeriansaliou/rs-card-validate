extern crate card_authenticator;

use card_authenticator::Authenticate;

fn visa_numbers() -> Vec<&'static str> {
    vec![
        "4539571147647251",
        "4532983409238819",
        "4485600412608021",
        "4916252910064718",
        "4916738103790259"
    ]
}

fn amex_numbers() -> Vec<&'static str> {
    vec![
        "343380440754432",
        "377156543570043",
        "340173808718013",
        "375801706141502",
        "372728319416034"
    ]
}

fn mastercard_numbers() -> Vec<&'static str> {
    vec![
        "5236313877109142",
        "5431604665471808",
        "5571788302926264",
        "5411516521560216",
        "5320524083396284"
    ]
}

fn discover_numbers() -> Vec<&'static str> {
    vec![
        "6011297718292606",
        "6011993225918523",
        "6011420510510997",
        "6011618637473995",
        "6011786207277235"
    ]
}

fn mixture() -> Vec<&'static str> {
    let card_types = vec![
        visa_numbers(),
        amex_numbers(),
        mastercard_numbers(),
        discover_numbers()
    ];

    let mut mixture = Vec::with_capacity(20);

    for card_type in card_types {
        for number in card_type {
            mixture.push(number);
        }
    }

    mixture
}

#[test]
fn valid_card() {
    for number in mixture() {
        let result = Authenticate::new(number);
        assert_eq!(result.valid, true);
    }
}

#[test]
fn valid_length() {
    for number in mixture() {
        let result = Authenticate::new(number);
        assert_eq!(result.length_valid, true);
    }
}

#[test]
fn valid_luhn() {
    for number in mixture() {
        let result = Authenticate::new(number);
        assert_eq!(result.luhn_valid, true);
    }
}

#[test]
fn correct_visa_card_name() {
    for number in visa_numbers() {
        let result = Authenticate::new(number);
        assert_eq!(result.card_type.name(), "visa".to_string());
    }
}

#[test]
fn correct_amex_card_name() {
    for number in amex_numbers() {
        let result = Authenticate::new(number);
        assert_eq!(result.card_type.name(), "amex".to_string());
    }
}

#[test]
fn correct_mastercard_card_name() {
    for number in mastercard_numbers() {
        let result = Authenticate::new(number);
        assert_eq!(result.card_type.name(), "mastercard".to_string());
    }
}

#[test]
fn correct_discover_card_name() {
    for number in discover_numbers() {
        let result = Authenticate::new(number);
        assert_eq!(result.card_type.name(), "discover".to_string());
    }
}
