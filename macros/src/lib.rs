#![warn(clippy::pedantic, rust_2018_idioms)]

use std::array;

use bytestring::ByteString;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{punctuated::Punctuated, Error, Expr, Ident, Result, Token};

fn ident_to_expr(ident: Ident) -> syn::Expr {
    syn::Expr::Path(syn::ExprPath {
        attrs: Vec::new(),
        qself: None,
        path: syn::Path {
            leading_colon: None,
            segments: Punctuated::from_iter([syn::PathSegment {
                arguments: syn::PathArguments::None,
                ident,
            }]),
        },
    })
}

#[derive(Debug)]
enum Piece {
    Literal(ByteString),
    Argument { expr: Expr, ident: Ident },
}

impl Piece {
    fn as_ident(&self) -> Option<&Ident> {
        if let Self::Argument { ident, .. } = self {
            Some(ident)
        } else {
            None
        }
    }

    fn as_expr(&self) -> Option<&Expr> {
        if let Self::Argument { expr, .. } = self {
            Some(expr)
        } else {
            None
        }
    }
}

impl quote::ToTokens for Piece {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Literal(str) if str.is_empty() => {}
            Self::Literal(str) => {
                let str: &str = str;
                quote!(out.push_str(#str);).to_tokens(tokens)
            }
            Self::Argument { ident, .. } => {
                quote!(out.push_str(#ident.as_str());).to_tokens(tokens)
            }
        }
    }
}

#[derive(Debug)]
struct Arguments {
    str_base_len: usize,
    pieces: Vec<Piece>,
}

impl syn::parse::Parse for Arguments {
    fn parse(input: syn::parse::ParseStream<'_>) -> Result<Self> {
        let format_str = input.parse::<syn::LitStr>()?;
        input.parse::<Option<Token![,]>>()?;

        let create_err = |msg| Error::new(format_str.span(), msg);
        let unterminated_fmt = move || create_err("Unterminated format argument");

        let format_string = ByteString::from(format_str.value());
        let mut current: &str = &*format_string;

        let mut str_base_len = 0;
        let mut pieces = Vec::new();

        for arg_num in 0_u8.. {
            let Some((text, rest)) = current.split_once('{') else {
                str_base_len += current.len();
                pieces.push(Piece::Literal(format_string.slice_ref(current)));
                break;
            };

            let (arg_name, rest) = rest.split_once('}').ok_or_else(unterminated_fmt)?;

            let arg_expr = if arg_name.is_empty() {
                if input.is_empty() {
                    return Err(create_err("Not enough arguments for format string"));
                }

                let argument = input.parse::<syn::Expr>()?;
                if input.parse::<Option<Token![,]>>()?.is_none() && !input.is_empty() {
                    return Err(Error::new(input.span(), "Missing argument seperator (`,`)"));
                };

                argument
            } else {
                ident_to_expr(syn::Ident::new(arg_name, format_str.span()))
            };

            current = rest;

            str_base_len += text.len();
            pieces.push(Piece::Literal(format_string.slice_ref(text)));
            pieces.push(Piece::Argument {
                expr: arg_expr,
                ident: syn::Ident::new(&format!("arg_{arg_num}"), Span::call_site()),
            });
        }

        Ok(Self {
            str_base_len,
            pieces,
        })
    }
}

struct FormatIntoArguments {
    write_into: syn::Ident,
    arguments: Arguments,
}

impl syn::parse::Parse for FormatIntoArguments {
    fn parse(input: syn::parse::ParseStream<'_>) -> Result<Self> {
        let write_into = input.parse()?;
        input.parse::<Token![,]>()?;

        let arguments = Arguments::parse(input)?;

        Ok(Self {
            write_into,
            arguments,
        })
    }
}

/// A no-alloc version of [`format!`], producing an [`ArrayString`].
///
/// ## Usage
/// Usage is similar to `format!`, although there are multiple limitations:
/// - No support for formatting flags, as we are not reinventing all of `format!`.
/// - No support for `format!("{name}", name=username)` syntax, may be lifted in future.
/// - No support for arbitrary expressions, this may be lifted in future aswell.
///
// Workaround for a rustdoc bug.
/// [`format!`]: https://doc.rust-lang.org/stable/std/macro.format.html
/// [`ArrayString`]: https://docs.rs/arrayvec/latest/arrayvec/struct.ArrayString.html
#[proc_macro]
pub fn aformat(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Arguments {
        str_base_len,
        pieces,
    } = match syn::parse(tokens) {
        Ok(args) => args,
        Err(err) => return err.into_compile_error().into(),
    };

    let caller_arguments = pieces.iter().filter_map(Piece::as_expr);
    let [arguments_iter_1, arguments_iter_2] =
        array::from_fn(|_| pieces.iter().filter_map(Piece::as_ident));

    let argument_count = arguments_iter_1.count();
    let [const_args_1, const_args_2, const_args_3, const_args_4, const_args_5] =
        array::from_fn(|_| (0..argument_count).map(|i| format_ident!("N{i}")));

    let return_adder = const_args_1.fold(
        quote!(StrBaseLen),
        |current, ident| quote!(RunAdd<#current, U<#ident>>),
    );

    let return_close_angle_braces = (0..argument_count).map(|_| <Token![>]>::default());

    quote!({
        use ::aformat::{ArrayString, ToArrayString, __internal::*};

        #[allow(clippy::too_many_arguments)]
        fn aformat_inner<StrBaseLen, #(const #const_args_2: usize),*>(
            #(#arguments_iter_2: ArrayString<#const_args_3>),*
        ) -> RunTypeToArrayString<#return_adder>
        where
            Const<#str_base_len>: ToUInt<Output = StrBaseLen>,
            #(Const<#const_args_4>: ToUInt,)*
            StrBaseLen: #(Add<U<#const_args_5>, Output: )* TypeNumToArrayString #(#return_close_angle_braces)*
        {
            let mut out = ArrayStringLike::new();
            // Fixes type inferrence
            if false { return out; }

            #(#pieces)*
            out
        }

        aformat_inner(#(ToArrayString::to_arraystring(#caller_arguments)),*)
    })
    .into()
}

/// [`aformat!`], but you provide your own [`ArrayString`].
///
/// The length of the [`ArrayString`] is checked at compile-time to fit all the arguments, although is not checked to be optimal.
///
/// ## Usage
/// The first argument should be the identifier of the [`ArrayString`], then the normal [`aformat!`] arguments follow.
///
/// ## Examples
/// ```
/// let mut out_buf = ArrayString::<32>::new();
///
/// let age = 18_u8;
/// aformat_into!(out_buf, "You are {} years old!", age);
///
/// assert_eq!(out_buf.as_str(), "You are 18 years old!");
/// ```
///
/// ```compile_fail
/// // Buffer is too small, so compile failure!
/// let mut out_buf = ArrayString::<4>::new();
///
/// let age = 18_u8;
/// aformat_into!(out_buf, "You are {} years old!", age);
/// ```
///
// Workaround for a rustdoc bug.
/// [`ArrayString`]: https://docs.rs/arrayvec/latest/arrayvec/struct.ArrayString.html
#[proc_macro]
pub fn aformat_into(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let FormatIntoArguments {
        arguments: Arguments {
            str_base_len,
            pieces,
        },
        write_into,
    } = match syn::parse(tokens) {
        Ok(args) => args,
        Err(err) => return err.into_compile_error().into(),
    };

    let caller_arguments = pieces.iter().filter_map(Piece::as_expr);
    let [arguments_iter_1, arguments_iter_2] =
        array::from_fn(|_| pieces.iter().filter_map(Piece::as_ident));

    let argument_count = arguments_iter_1.clone().count();
    let [const_args_1, const_args_2, const_args_3, const_args_4] =
        array::from_fn(|_| (0..argument_count).map(|i| format_ident!("N{i}")));

    let return_close_angle_braces = (0..argument_count).map(|_| <Token![>]>::default());

    quote!({
        use ::aformat::{ArrayString, ToArrayString, __internal::*};

        fn aformat_into_inner<StrBaseLen, const OUT: usize, #(const #const_args_2: usize),*>(
            out: &mut ArrayString<OUT>,
            #(#arguments_iter_2: ArrayString<#const_args_3>),*
        )
        where
            Const<#str_base_len>: ToUInt<Output = StrBaseLen>,
            Const<OUT>: ToUInt,
            #(Const<#const_args_4>: ToUInt,)*

            StrBaseLen: #(Add<U<#const_args_1>, Output: )* IsLessOrEqual<U<OUT>, Output: BufferFits> #(#return_close_angle_braces)*
        {
            #(#pieces)*
        }

        aformat_into_inner(&mut #write_into, #(ToArrayString::to_arraystring(#caller_arguments)),*)
    })
    .into()
}
