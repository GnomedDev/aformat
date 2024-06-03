#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use std::hint::black_box;

use aformat::{aformat, astr};

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

pub fn format_into(age: u8) -> arrayvec::ArrayString<22> {
    aformat!("You are {age} years old!",)
}
