pub use core::ops::Add;

use arrayvec::ArrayString;
pub use typenum::{Const, IsLessOrEqual, ToUInt, B1, U};

#[diagnostic::on_unimplemented(
    message = "Out buffer is not large enough for the formatted arguments",
    label = "The maximum size of formatted arguments would overflow the provided out buffer.",
    note = "Increase the size of the provided ArrayString buffer, or reduce the number/size of arguments."
)]
pub trait BufferFits {}

impl BufferFits for B1 {}

pub trait ArrayStringLike {
    fn new() -> Self;
    fn push_str(&mut self, s: &str);
}

impl<const N: usize> ArrayStringLike for ArrayString<N> {
    fn new() -> Self {
        Self::new()
    }

    fn push_str(&mut self, s: &str) {
        self.push_str(s);
    }
}

pub trait TypeNumToArrayString {
    type ArrayString: ArrayStringLike;
}

typenum_mappings::impl_typenum_mapping!(
    impl<const CAP: usize = 0..=1024> TypeNumToArrayString for #TypeNumName {
        type ArrayString = ArrayString<CAP>;
    }
);

pub type RunAdd<T1, T2> = <T1 as Add<T2>>::Output;
pub type RunTypeToArrayString<T> = <T as TypeNumToArrayString>::ArrayString;
