use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

mod masker;
mod optbuilder;
mod optfn;
mod optfn2;
mod optstruct;

/// The html! macro makes it easy for developers to write jsx-style markup in their components.
/// We aim to keep functional parity with html templates.
// #[proc_macro_attribute]
// pub fn optfn(_attr: TokenStream, s: TokenStream) -> TokenStream {
//     match syn::parse::<optfn::OptFn>(s) {
//         Err(e) => e.to_compile_error().into(),
//         Ok(s) => s.to_token_stream().into(),
//     }
// }

#[proc_macro_attribute]
pub fn optfn(_attr: TokenStream, s: TokenStream) -> TokenStream {
    match syn::parse::<optfn2::OptFn2>(s) {
        Err(e) => e.to_compile_error().into(),
        Ok(s) => s.to_token_stream().into(),
    }
}

#[proc_macro_derive(OptStruct, attributes(builder))]
pub fn optstruct(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // todo!()
    let input = parse_macro_input!(input as syn::DeriveInput);
    input.to_token_stream().into()
    // match props::impl_my_derive(&input) {
    //     Ok(output) => output.into(),
    //     Err(error) => error.to_compile_error().into(),
    // }
}

#[proc_macro]
pub fn masker(s: TokenStream) -> TokenStream {
    match syn::parse::<masker::Masker>(s) {
        Err(e) => e.to_compile_error().into(),
        Ok(s) => s.to_token_stream().into(),
    }
}
