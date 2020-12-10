extern crate card_validate;

use card_validate::{Validate, ValidateError};

fn visaelectron_numbers_valid() -> Vec<&'static str> {
    vec!["4917300800000000"]
}

fn maestro_numbers_valid() -> Vec<&'static str> {
    vec!["6759649826438453"]
}

fn forbrugsforeningen_numbers_valid() -> Vec<&'static str> {
    vec!["6007220000000004"]
}

fn dankort_numbers_valid() -> Vec<&'static str> {
    vec!["5019717010103742"]
}

fn visa_numbers_valid() -> Vec<&'static str> {
    vec![
        "4539571147647251",
        "4532983409238819",
        "4485600412608021",
        "4916252910064718",
        "4916738103790259",
    ]
}

fn amex_numbers_valid() -> Vec<&'static str> {
    vec![
        "343380440754432",
        "377156543570043",
        "340173808718013",
        "375801706141502",
        "372728319416034",
    ]
}

fn mastercard_numbers_valid() -> Vec<&'static str> {
    vec![
        "5236313877109142",
        "5431604665471808",
        "5571788302926264",
        "5411516521560216",
        "5320524083396284",
    ]
}

fn discover_numbers_valid() -> Vec<&'static str> {
    vec![
        "6011297718292606",
        "6011993225918523",
        "6011420510510997",
        "6011618637473995",
        "6011786207277235",
    ]
}

fn jcb_numbers_valid() -> Vec<&'static str> {
    vec!["3530111333300000", "3566002020360505"]
}

fn unionpay_numbers_valid() -> Vec<&'static str> {
    vec![
        "6271136264806203568",
        "6236265930072952775",
        "6204679475679144515",
        "6216657720782466507",
    ]
}

fn dinersclub_numbers_valid() -> Vec<&'static str> {
    vec![
        "30569309025904",
        "38520000023237",
        "36700102000000",
        "36148900647913",
    ]
}

fn gibberish_numbers_invalid() -> Vec<&'static str> {
    vec!["zduhehiudIHZHIUZHUI", "0292DYYEFYFEFYEFEFIUH"]
}

fn unknown_numbers_invalid() -> Vec<&'static str> {
    vec!["00002837743671762", "1136283774"]
}

fn known_numbers_invalid() -> Vec<&'static str> {
    vec!["424242424", "4242424242424244242424242", "523631387710914"]
}

fn numbers_invalid_luhn() -> Vec<&'static str> {
    vec!["5236313877109141", "6011420510510995"]
}

fn valid_mixture() -> Vec<&'static str> {
    let card_types = vec![
        visaelectron_numbers_valid(),
        maestro_numbers_valid(),
        forbrugsforeningen_numbers_valid(),
        dankort_numbers_valid(),
        visa_numbers_valid(),
        amex_numbers_valid(),
        mastercard_numbers_valid(),
        discover_numbers_valid(),
        jcb_numbers_valid(),
        unionpay_numbers_valid(),
        dinersclub_numbers_valid(),
    ];

    let mut mixture = Vec::new();

    for card_type in card_types {
        for number in card_type {
            mixture.push(number);
        }
    }

    mixture
}

#[test]
fn valid_card() {
    for number in valid_mixture() {
        assert_eq!(Validate::from(number).is_ok(), true);
    }
}

#[test]
fn gibberish_invalid() {
    for number in gibberish_numbers_invalid() {
        assert_eq!(
            Validate::from(number) == Err(ValidateError::InvalidFormat),
            true
        );
    }
}

#[test]
fn unknown_invalid() {
    for number in unknown_numbers_invalid() {
        assert_eq!(
            Validate::from(number) == Err(ValidateError::UnknownType),
            true
        );
    }
}

#[test]
fn known_invalid() {
    for number in known_numbers_invalid() {
        assert_eq!(
            Validate::from(number) == Err(ValidateError::InvalidLength),
            true
        );
    }
}

#[test]
fn invalid_luhn() {
    for number in numbers_invalid_luhn() {
        assert_eq!(
            Validate::from(number) == Err(ValidateError::InvalidLuhn),
            true
        );
    }
}

#[test]
fn correct_visaelectron_card_name() {
    for number in visaelectron_numbers_valid() {
        let result = Validate::from(number).unwrap();
        assert_eq!(result.card_type.name(), "visaelectron".to_string());
    }
}

#[test]
fn correct_maestro_card_name() {
    for number in maestro_numbers_valid() {
        let result = Validate::from(number).unwrap();
        assert_eq!(result.card_type.name(), "maestro".to_string());
    }
}

#[test]
fn correct_forbrugsforeningen_card_name() {
    for number in forbrugsforeningen_numbers_valid() {
        let result = Validate::from(number).unwrap();
        assert_eq!(result.card_type.name(), "forbrugsforeningen".to_string());
    }
}

#[test]
fn correct_dankort_card_name() {
    for number in dankort_numbers_valid() {
        let result = Validate::from(number).unwrap();
        assert_eq!(result.card_type.name(), "dankort".to_string());
    }
}

#[test]
fn correct_visa_card_name() {
    for number in visa_numbers_valid() {
        let result = Validate::from(number).unwrap();
        assert_eq!(result.card_type.name(), "visa".to_string());
    }
}

#[test]
fn correct_amex_card_name() {
    for number in amex_numbers_valid() {
        let result = Validate::from(number).unwrap();
        assert_eq!(result.card_type.name(), "amex".to_string());
    }
}

#[test]
fn correct_mastercard_card_name() {
    for number in mastercard_numbers_valid() {
        let result = Validate::from(number).unwrap();
        assert_eq!(result.card_type.name(), "mastercard".to_string());
    }
}

#[test]
fn correct_discover_card_name() {
    for number in discover_numbers_valid() {
        let result = Validate::from(number).unwrap();
        assert_eq!(result.card_type.name(), "discover".to_string());
    }
}

#[test]
fn correct_jcb_card_name() {
    for number in jcb_numbers_valid() {
        let result = Validate::from(number).unwrap();
        assert_eq!(result.card_type.name(), "jcb".to_string());
    }
}

#[test]
fn correct_unionpay_card_name() {
    for number in unionpay_numbers_valid() {
        let result = Validate::from(number).unwrap();
        assert_eq!(result.card_type.name(), "unionpay".to_string());
    }
}

#[test]
fn correct_dinersclub_card_name() {
    for number in dinersclub_numbers_valid() {
        let result = Validate::from(number).unwrap();
        assert_eq!(result.card_type.name(), "dinersclub".to_string());
    }
}
