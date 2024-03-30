//! card-validate detects and validates credit card numbers (type of card, number length and
//! Luhn checksum).

// Notice: this source code has been imported from: https://github.com/luhnmod10/rust
// In an effort to reduce the amount of dependencies in the `card-validate` library.

pub fn valid(number: &str) -> bool {
    let mut checksum = 0;

    let r_chars = number.chars().rev();
    let range = 1..=number.chars().count();
    let iter = range.zip(r_chars);

    for (i, c) in iter {
        let is_odd = i % 2 == 1;
        if is_odd {
            checksum += checksum_modifier_odd(c);
        } else {
            checksum += checksum_modifier_even(c);
        }
    }

    checksum % 10 == 0
}

fn checksum_modifier_odd(c: char) -> u32 {
    numeric_char_to_u32(c)
}

fn checksum_modifier_even(c: char) -> u32 {
    let n = numeric_char_to_u32(c);
    let d = n * 2;
    if d <= 9 {
        d
    } else {
        d - 9
    }
}

fn numeric_char_to_u32(c: char) -> u32 {
    (c as u32) - ('0' as u32)
}
