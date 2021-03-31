use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

mod optbuilder;
mod optfn;

/// The html! macro makes it easy for developers to write jsx-style markup in their components.
/// We aim to keep functional parity with html templates.
#[proc_macro_attribute]
pub fn optfn(_attr: TokenStream, s: TokenStream) -> TokenStream {
    match syn::parse::<optfn::OptFn>(s) {
        Err(e) => e.to_compile_error().into(),
        Ok(s) => s.to_token_stream().into(),
    }
}

#[proc_macro_derive(Optbuilder, attributes(builder))]
pub fn derive_typed_builder(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    todo!()
    // let input = parse_macro_input!(input as syn::DeriveInput);
    // match props::impl_my_derive(&input) {
    //     Ok(output) => output.into(),
    //     Err(error) => error.to_compile_error().into(),
    // }
}
