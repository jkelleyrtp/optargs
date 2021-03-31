use syn::parse::{discouraged::Speculative, ParseBuffer};

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
pub struct OptFn {}

impl ToTokens for OptFn {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        todo!()
    }
}

impl Parse for OptFn {
    fn parse(input: ParseStream) -> Result<Self> {
        todo!()
    }
}
