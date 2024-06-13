use std::hint::black_box;

use aformat::{aformat, astr};

#[test]
pub fn basic_aformat() {
    let name = astr!("Walter Hartwell White");
    let street_num = 308_u16;

    let out = {
        use ::aformat::{ArrayString, ToArrayString, __internal::*};
        fn aformat_inner<StrBaseLen, const N0: usize, const N1: usize>(
            name: ArrayString<N0>,
            street_num: ArrayString<N1>,
        ) -> RunTypeToArrayString<RunAdd<RunAdd<StrBaseLen, U<N0>>, U<N1>>>
        where
            Const<41usize>: ToUInt<Output = StrBaseLen>,
            Const<N0>: ToUInt,
            Const<N1>: ToUInt,
            StrBaseLen: Add<U<N0>, Output: Add<U<N1>, Output: TypeNumToArrayString>>,
        {
            let mut out = ArrayStringLike::new();
            out.push_str("My name is ");
            out.push_str(name.as_str());
            out.push_str(", I live at ");
            out.push_str(street_num.as_str());
            out.push_str(" Negra Aroyo Lane.");
            out
        }
        aformat_inner(
            ToArrayString::to_arraystring(name),
            ToArrayString::to_arraystring(street_num),
        )
    };

    black_box(out);
}

pub fn stress_test() {
    let [a0, a1, a2, a3, a4, a5, a6, a7, a8, a9] = [0_u64; 10];
    let [a10, a20, a30, a40, a50, a60, a70, a80, a90, a100] = [0_u64; 10];

    black_box({
        use ::aformat::{ArrayString, ToArrayString, __internal::*};
        fn aformat_inner<StrBaseLen,const N0:usize,const N1:usize,const N2:usize,const N3:usize,const N4:usize,const N5:usize,const N6:usize,const N7:usize,const N8:usize,const N9:usize,const N10:usize,const N11:usize,const N12:usize,const N13:usize,const N14:usize,const N15:usize,const N16:usize,const N17:usize,const N18:usize,const N19:usize>(a0:ArrayString<N0> ,a1:ArrayString<N1> ,a2:ArrayString<N2> ,a3:ArrayString<N3> ,a4:ArrayString<N4> ,a5:ArrayString<N5> ,a6:ArrayString<N6> ,a7:ArrayString<N7> ,a8:ArrayString<N8> ,a9:ArrayString<N9> ,a10:ArrayString<N10> ,a20:ArrayString<N11> ,a30:ArrayString<N12> ,a40:ArrayString<N13> ,a50:ArrayString<N14> ,a60:ArrayString<N15> ,a70:ArrayString<N16> ,a80:ArrayString<N17> ,a90:ArrayString<N18> ,a100:ArrayString<N19>) -> RunTypeToArrayString<RunAdd<RunAdd<RunAdd<RunAdd<RunAdd<RunAdd<RunAdd<RunAdd<RunAdd<RunAdd<RunAdd<RunAdd<RunAdd<RunAdd<RunAdd<RunAdd<RunAdd<RunAdd<RunAdd<RunAdd<StrBaseLen,U<N0>> ,U<N1>> ,U<N2>> ,U<N3>> ,U<N4>> ,U<N5>> ,U<N6>> ,U<N7>> ,U<N8>> ,U<N9>> ,U<N10>> ,U<N11>> ,U<N12>> ,U<N13>> ,U<N14>> ,U<N15>> ,U<N16>> ,U<N17>> ,U<N18>> ,U<N19>> >where Const<0usize> :ToUInt<Output = StrBaseLen> ,Const<N0> :ToUInt,Const<N1> :ToUInt,Const<N2> :ToUInt,Const<N3> :ToUInt,Const<N4> :ToUInt,Const<N5> :ToUInt,Const<N6> :ToUInt,Const<N7> :ToUInt,Const<N8> :ToUInt,Const<N9> :ToUInt,Const<N10> :ToUInt,Const<N11> :ToUInt,Const<N12> :ToUInt,Const<N13> :ToUInt,Const<N14> :ToUInt,Const<N15> :ToUInt,Const<N16> :ToUInt,Const<N17> :ToUInt,Const<N18> :ToUInt,Const<N19> :ToUInt,StrBaseLen:Add<U<N0> ,Output:Add<U<N1> ,Output:Add<U<N2> ,Output:Add<U<N3> ,Output:Add<U<N4> ,Output:Add<U<N5> ,Output:Add<U<N6> ,Output:Add<U<N7> ,Output:Add<U<N8> ,Output:Add<U<N9> ,Output:Add<U<N10> ,Output:Add<U<N11> ,Output:Add<U<N12> ,Output:Add<U<N13> ,Output:Add<U<N14> ,Output:Add<U<N15> ,Output:Add<U<N16> ,Output:Add<U<N17> ,Output:Add<U<N18> ,Output:Add<U<N19> ,Output:TypeNumToArrayString> > > > > > > > > > > > > > > > > > > >{
            let mut out = ArrayStringLike::new();
            if false {
                return out;
            }
            out.push_str(a0.as_str());
            out.push_str(a1.as_str());
            out.push_str(a2.as_str());
            out.push_str(a3.as_str());
            out.push_str(a4.as_str());
            out.push_str(a5.as_str());
            out.push_str(a6.as_str());
            out.push_str(a7.as_str());
            out.push_str(a8.as_str());
            out.push_str(a9.as_str());
            out.push_str(a10.as_str());
            out.push_str(a20.as_str());
            out.push_str(a30.as_str());
            out.push_str(a40.as_str());
            out.push_str(a50.as_str());
            out.push_str(a60.as_str());
            out.push_str(a70.as_str());
            out.push_str(a80.as_str());
            out.push_str(a90.as_str());
            out.push_str(a100.as_str());
            out.push_str("");
            out
        }
        aformat_inner(
            ToArrayString::to_arraystring(a0),
            ToArrayString::to_arraystring(a1),
            ToArrayString::to_arraystring(a2),
            ToArrayString::to_arraystring(a3),
            ToArrayString::to_arraystring(a4),
            ToArrayString::to_arraystring(a5),
            ToArrayString::to_arraystring(a6),
            ToArrayString::to_arraystring(a7),
            ToArrayString::to_arraystring(a8),
            ToArrayString::to_arraystring(a9),
            ToArrayString::to_arraystring(a10),
            ToArrayString::to_arraystring(a20),
            ToArrayString::to_arraystring(a30),
            ToArrayString::to_arraystring(a40),
            ToArrayString::to_arraystring(a50),
            ToArrayString::to_arraystring(a60),
            ToArrayString::to_arraystring(a70),
            ToArrayString::to_arraystring(a80),
            ToArrayString::to_arraystring(a90),
            ToArrayString::to_arraystring(a100),
        )
    });
}

// #[test]
// pub fn aformat_into() {
//     let mut out_buf = ArrayString::<32>::new();

//     let age = 18_u8;
//     aformat_into!(out_buf, "You are {} years old!", age);

//     assert_eq!(out_buf.as_str(), "You are 18 years old!");
// }

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
