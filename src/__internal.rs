pub use core::ops::Add;

use arrayvec::ArrayString;
pub use typenum::{Const, ToUInt, U};

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

typenum_mappings::impl_typenum_mapping!(
    pub impl<const CAP: usize = 0..=1024> TypeNumToArrayString for #TypeNumName {
        type ArrayString: ArrayStringLike = ArrayString<CAP>;
    }
);

pub type RunAdd<T1, T2> = <T1 as Add<T2>>::Output;
pub type RunTypeToArrayString<T> = <T as TypeNumToArrayString>::ArrayString;
