use std::cell::Cell;

use aformat::{aformat, aformat_into, astr, ArrayString};
use utils::OnlyFormatOnce;

mod utils;

#[test]
pub fn basic_aformat() {
    let name = astr!("Walter Hartwell White");
    let street_num = 308_u16;

    let out = aformat!("My name is {name}, I live at {street_num} Negra Aroyo Lane.");
    assert_eq!(
        out.as_str(),
        "My name is Walter Hartwell White, I live at 308 Negra Aroyo Lane."
    );
}

#[test]
pub fn expr_aformat() {
    let out = aformat!("2 + 2 = {}", 2_u8 + 2);
    assert_eq!(out.as_str(), "2 + 2 = 4");
}

#[test]
pub fn duplicated_arguments() {
    let was_formatted = Cell::new(false);
    let num = OnlyFormatOnce(1_u8, &was_formatted);

    let out = aformat!("{num} {}", num);
    assert_eq!(out.as_str(), "1 1");
}

#[test]
pub fn aformat_into() {
    let mut out_buf = ArrayString::<32>::new();

    let age = 18_u8;
    aformat_into!(out_buf, "You are {} years old!", age);

    assert_eq!(out_buf.as_str(), "You are 18 years old!");
}

#[test]
pub fn astr() {
    const STR: &str = "Hello World";

    let const_val = astr!(STR);
    let lit_val = astr!("Hello World");

    for val in [const_val, lit_val] {
        assert_eq!(val.as_str(), "Hello World");
        assert_eq!(val.capacity(), val.len());
    }
}
