use std::{cell::Cell, panic::AssertUnwindSafe};

use aformat::ToArrayString;

#[derive(Clone, Copy)]
pub struct OnlyFormatOnce<'a, T>(pub T, pub &'a Cell<bool>);

impl<T: ToArrayString> ToArrayString for OnlyFormatOnce<'_, T> {
    type ArrayString = T::ArrayString;
    const MAX_LENGTH: usize = T::MAX_LENGTH;

    fn to_arraystring(self) -> Self::ArrayString {
        if self.1.get() {
            panic!("ToArrayString called twice!");
        }

        self.1.set(true);
        self.0.to_arraystring()
    }
}

#[test]
fn only_format_once() {
    let guard = Cell::new(false);
    let format_once = OnlyFormatOnce(1_u8, &guard);

    format_once.to_arraystring();
    std::panic::catch_unwind(AssertUnwindSafe(|| format_once.to_arraystring())).unwrap_err();
}
