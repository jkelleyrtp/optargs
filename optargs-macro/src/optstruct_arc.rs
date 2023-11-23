use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::Result;

use crate::optstruct::OptStructConf;

pub struct OptStructArc {
    conf: OptStructConf,
}

impl Parse for OptStructArc {
    fn parse(input: ParseStream) -> Result<Self> {
        OptStructConf::parse(input).map(|conf| Self { conf })
    }
}

impl ToTokens for OptStructArc {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let (name, helper_defs, inners_body, call_body, validator) =
            OptStructConf::prepare_generate_tokens(&self.conf);

        ToTokens::to_tokens(
            &quote! {

                #[doc(hidden)]
                #[macro_export]
                macro_rules! #name {
                    ($($key:ident $(: $value:expr)? ), * $(,)?) => {
                        {
                            #[allow(unused_mut)]
                            let mut inners = (#( #inners_body )*);
                            { $( #name! (@setter_helper inners $key $key $($value)? ); )* }
                            #validator

                            #[allow(unused_mut)]
                            let mut validator = Validator::builder();
                            validator $(.$key())* .build();

                            std::sync::Arc::new(#name{
                                #( #call_body )*
                            })
                        }
                    };
                    #( #helper_defs )*
                }

            },
            tokens,
        );
    }
}
