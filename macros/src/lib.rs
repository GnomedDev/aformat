#![warn(clippy::pedantic, rust_2018_idioms)]

use std::array;

use bytestring::ByteString;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Error, Ident, Result, Token};

#[derive(Debug)]
enum Piece {
    Literal(ByteString),
    Argument(Ident),
}

impl quote::ToTokens for Piece {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Literal(str) => str.to_tokens(tokens),
            Self::Argument(argument) => quote!(&#argument.to_arraystring()).to_tokens(tokens),
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
        let arg_missing = move || create_err("Not enough arguments for format string");

        let format_string = ByteString::from(format_str.value());
        let mut current: &str = &*format_string;

        let mut str_base_len = 0;
        let mut pieces = Vec::new();

        loop {
            let Some((text, rest)) = current.split_once('{') else {
                str_base_len += current.len();
                pieces.push(Piece::Literal(format_string.slice_ref(current)));
                break;
            };

            let (arg_name, rest) = rest.split_once('}').ok_or_else(unterminated_fmt)?;

            let ident = if arg_name.is_empty() {
                let argument = input.parse::<Option<_>>()?.ok_or_else(arg_missing)?;
                if input.parse::<Option<Token![,]>>()?.is_none() && !input.is_empty() {
                    return Err(Error::new(input.span(), "Missing argument seperator (`,`)"));
                };

                argument
            } else {
                Ident::new(arg_name, format_str.span())
            };

            current = rest;

            str_base_len += text.len();
            pieces.push(Piece::Literal(format_string.slice_ref(text)));
            pieces.push(Piece::Argument(ident));
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

fn aformat_impl(
    Arguments {
        str_base_len,
        pieces,
    }: Arguments,
    write_into: Option<syn::Ident>,
) -> proc_macro2::TokenStream {
    let arguments_iter = pieces.iter().filter_map(|p| {
        if let Piece::Argument(ident) = p {
            Some(ident)
        } else {
            None
        }
    });

    let argument_count = arguments_iter.clone().count();
    let [arguments_iter_1, arguments_iter_2, arguments_iter_3, arguments_iter_4] =
        array::from_fn(|_| arguments_iter.clone());

    let type_args = (0..argument_count).map(|i| format_ident!("T{i}"));
    let [type_args_1, type_args_2, type_args_3, type_args_4, type_args_5] =
        array::from_fn(|_| type_args.clone());

    let calc_required_len = {
        let type_args = type_args.clone();
        quote!(#str_base_len + #(#type_args::MAX_LENGTH)+*)
    };

    let final_expr = match write_into {
        Some(ident) => quote!(aformat_into_inner(&mut #ident, #(#arguments_iter_1),*)),
        None => quote!(
            fn aformat_inner<#(#type_args_4: ToArrayString),*>(
                #(#arguments_iter_2: #type_args_5),*
            ) -> ArrayString<{ #calc_required_len }> {
                let mut out_buffer = ArrayString::new();
                aformat_into_inner(&mut out_buffer, #(#arguments_iter_3),*);
                out_buffer
            }

            aformat_inner(#(#arguments_iter_4),*)
        ),
    };

    quote!({
        use ::aformat::{ArrayString, ToArrayString};

        const fn check_args_fits<const BUF_CAP: usize, #(#type_args_2: ToArrayString),*>() {
            assert!(BUF_CAP >= (#calc_required_len), "Buffer is not large enough to format into")
        }

        fn aformat_into_inner<const BUF_CAP: usize, #(#type_args: ToArrayString),*>(
            out_buffer: &mut ArrayString<BUF_CAP>,
            #(#arguments_iter: #type_args_1),*
        ) {
            const { check_args_fits::<BUF_CAP, #(#type_args_3),*>() };
            #(out_buffer.push_str(#pieces);)*
        }

        #final_expr
    })
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
    let arguments = match syn::parse(tokens) {
        Ok(args) => args,
        Err(err) => return err.into_compile_error().into(),
    };

    aformat_impl(arguments, None).into()
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
        arguments,
        write_into,
    } = match syn::parse(tokens) {
        Ok(args) => args,
        Err(err) => return err.into_compile_error().into(),
    };

    aformat_impl(arguments, Some(write_into)).into()
}
