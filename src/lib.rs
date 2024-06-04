//! A no-std and no-alloc version of [`format!`] using [`ToArrayString`].
//!
//! ## Example
//!
//! ```
//! #![feature(generic_const_exprs)]
//! #![allow(incomplete_features)]
//!
//! use aformat::{astr, aformat, CapStr};
//!
//! pub fn say_hello(name: &str, age: u8) {
//!     let name = CapStr::<256>(name);
//!
//!     let formatted = aformat!("Hello {name}, you are {age} years old!");
//!     println!("{}", formatted.as_str());
//! }
//!
//! say_hello("Walter White", 50);
//! ```
//!
//! ## Minimum Supported Rust Version
//!
//! This is currently latest `nightly`, and requires the `generic_const_exprs` feature.
#![cfg_attr(not(doc), no_std)]
#![warn(clippy::pedantic, rust_2018_idioms)]

#[doc(no_inline)]
pub use arrayvec::ArrayString;
#[doc(no_inline)]
pub use to_arraystring::ToArrayString;

pub use aformat_macros::{aformat, aformat_into, astr};

/// A transparent wrapper around `&str` to truncate the byte length to a compile time constant.
///
/// This implements [`ToArrayString`], allowing you to use it in [`aformat!`].
///
/// If you simply want to pass a string literal into [`aformat!`], use [`astr!`].
#[derive(Clone, Copy)]
pub struct CapStr<'a, const MAX_LENGTH: usize>(pub &'a str);

impl<const MAX_LENGTH: usize> core::ops::Deref for CapStr<'_, MAX_LENGTH> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        for i in (0..MAX_LENGTH).rev() {
            if let Some(valid_slice) = self.0.get(..i) {
                return valid_slice;
            }
        }

        ""
    }
}

impl<const MAX_LENGTH: usize> ToArrayString for CapStr<'_, MAX_LENGTH> {
    const MAX_LENGTH: usize = MAX_LENGTH;
    type ArrayString = ArrayString<MAX_LENGTH>;

    fn to_arraystring(self) -> Self::ArrayString {
        ArrayString::from(&self).unwrap()
    }
}
