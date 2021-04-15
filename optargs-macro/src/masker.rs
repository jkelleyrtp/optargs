use std::str::FromStr;

use syn::{
    parse::{discouraged::Speculative, ParseBuffer},
    GenericArgument, PatType, PathArguments,
};

use {
    proc_macro::TokenStream,
    proc_macro2::{Span, TokenStream as TokenStream2},
    quote::{quote, ToTokens, TokenStreamExt},
    syn::{
        ext::IdentExt,
        parse::{Parse, ParseStream},
        token, Error, Expr, ExprClosure, Ident, LitBool, LitStr, Path, Result, Token,
    },
};

use syn::{
    parse_macro_input, Attribute, Block, FnArg, Generics, Item, ItemFn, ReturnType, Type,
    Visibility,
};

pub struct Masker {
    e: Expr,
}
impl Parse for Masker {
    fn parse(input: ParseStream) -> Result<Self> {
        let e = input.parse::<syn::Expr>()?;
        Ok(Self { e })
    }
}

impl ToTokens for Masker {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        // panic!("{:?}", self.e);
        let toks = quote! {
            (a.unwrap(), b)
        };
        self.e.to_tokens(tokens)
    }
}
