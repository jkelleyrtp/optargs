use proc_macro::TokenStream;
use quote::ToTokens;

mod optfn;
mod optstruct;
mod optstruct_arc;

#[proc_macro_attribute]
pub fn optfn(_attr: TokenStream, s: TokenStream) -> TokenStream {
    match syn::parse::<optfn::OptFn>(s) {
        Err(e) => e.to_compile_error().into(),
        Ok(s) => s.to_token_stream().into(),
    }
}

#[proc_macro_derive(OptStruct, attributes(builder))]
pub fn optstruct(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match syn::parse::<optstruct::OptStruct>(input) {
        Err(e) => e.to_compile_error().into(),
        Ok(s) => s.to_token_stream().into(),
    }
}

#[proc_macro_derive(OptStructArc, attributes(builder))]
pub fn optstruct_arc(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match syn::parse::<optstruct_arc::OptStructArc>(input) {
        Err(e) => e.to_compile_error().into(),
        Ok(s) => s.to_token_stream().into(),
    }
}
