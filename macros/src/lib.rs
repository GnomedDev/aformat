#![warn(clippy::pedantic, rust_2018_idioms)]

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

fn aformat_impl(tokens: proc_macro::TokenStream) -> Result<TokenStream> {
    let Arguments {
        str_base_len,
        pieces,
    } = syn::parse(tokens)?;

    let arguments_iter = pieces.iter().filter_map(|p| {
        if let Piece::Argument(ident) = p {
            Some(ident)
        } else {
            None
        }
    });

    let arguments_iter_1 = arguments_iter.clone();
    let argument_count = arguments_iter.clone().count();

    let type_args = (0..argument_count).map(|i| format_ident!("T{i}"));
    let type_args_1 = type_args.clone();
    let type_args_2 = type_args.clone();

    let out = quote!({
        use ::aformat::{ArrayString, ToArrayString};

        fn aformat_inner<#(#type_args: ToArrayString),*>(
            #(#arguments_iter: #type_args_1),*
        ) -> ArrayString<{ #str_base_len + #(#type_args_2::MAX_LENGTH)+* }>
        {
            let mut out_buffer = ArrayString::new();
            #(out_buffer.push_str(#pieces);)*
            out_buffer
        }

        aformat_inner(#(#arguments_iter_1),*)
    });

    Ok(out)
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
    match aformat_impl(tokens) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

fn astr_impl(tokens: proc_macro::TokenStream) -> Result<TokenStream> {
    let str = syn::parse::<syn::LitStr>(tokens)?;
    let str_len = str.value().len();
    let str = str.token();

    Ok(quote!(::aformat::ArrayString::<#str_len>::from(#str).unwrap()))
}

/// A simple and easy way to make a perfectly fitting [`ArrayString`] from a literal.
///
/// ## Expansion
/// ```rust
/// let my_string = astr!("Hello World");
/// ```
/// expands to
/// ```rust
/// let my_string = aformat::ArrayString::<11>::from("Hello World").unwrap();
/// ```
///
/// [`ArrayString`]: https://docs.rs/arrayvec/latest/arrayvec/struct.ArrayString.html
#[proc_macro]
pub fn astr(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match astr_impl(tokens) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.into_compile_error().into(),
    }
}
