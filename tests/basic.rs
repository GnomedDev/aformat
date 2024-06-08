#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use std::hint::black_box;

use aformat::{aformat, aformat_into, astr, ArrayString};

#[test]
pub fn basic_aformat() {
    let name = astr!("Walter Hartwell White");
    let street_num = 308_u16;

    let out = aformat!(
        "My name is {}, I live at {} Negra Aroyo Lane.",
        name,
        street_num
    );

    black_box(out);
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
