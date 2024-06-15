//! A no-std and no-alloc version of [`format!`] using [`ToArrayString`].
//!
//! ## Example
//!
//! ```
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
//! This is currently `1.79`, and is considered a breaking change to increase.
#![cfg_attr(not(doc), no_std)]
#![warn(clippy::pedantic, rust_2018_idioms)]

#[doc(no_inline)]
pub use arrayvec::ArrayString;
#[doc(no_inline)]
pub use to_arraystring::ToArrayString;

pub use aformat_macros::{aformat, aformat_into};

#[doc(hidden)]
pub mod __internal;

/// A simple and easy way to make a perfectly fitting [`ArrayString`] from a literal.
///
/// ## Expansion
/// ```rust
/// use aformat::astr;
///
/// let my_string = astr!("Hello World");
/// ```
/// expands to
/// ```rust
/// let my_string = {
///     const STR_LEN: usize = str::len("Hello World");
///     aformat::ArrayString::<STR_LEN>::from("Hello World").unwrap();
/// };
/// ```
#[macro_export]
macro_rules! astr {
    ($val:expr) => {{
        const STR_LEN: usize = ::core::primitive::str::len($val);
        ::aformat::ArrayString::<STR_LEN>::from($val).unwrap()
    }};
}

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
