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

type BuilderField = (Ident, Box<Type>);
pub struct OptFn2 {
    original: ItemFn,
    required_args: Vec<BuilderField>,
    optional_args: Vec<BuilderField>,

    // generics: GenericGenerator,
    vis: Visibility,
    name: Ident,
    return_type: ReturnType,
}

impl Parse for OptFn2 {
    /*

    We care about:
    - all the fields
    - that all "optional" fields come after required/positional fields
    - the original
    - any generics (need to be mapped to the builder)
    - fields that are required
    - fields that are optional
    */

    fn parse(input: ParseStream) -> Result<Self> {
        let orig: ItemFn = input.parse()?;

        // start by parsing positionals
        // optionals must come after positionals
        let mut parsing_optionals = false;
        let mut required_args = Vec::new();
        let mut optional_args = Vec::new();

        for arg in orig.sig.inputs.clone() {
            let g = Ok(arg);
            g.and_then(|f| match f {
                FnArg::Typed(arg) => Ok(arg),
                FnArg::Receiver(r) => Err(Error::new_spanned(r, "optfn cannot be used on methods")),
            })
            .and_then(|f| match (&f).pat.as_ref() {
                syn::Pat::Ident(iden) => Ok((iden.clone(), f)),
                other => Err(Error::new_spanned(other, "optfn cannot struct fields")),
            })
            .map(|(name, pat)| {
                let is_optional = match pat.ty.as_ref() {
                    Type::Path(p) => {
                        if let Some(arg) = p.path.segments.first() {
                            arg.ident.to_string() == "Option"
                        } else {
                            false
                        }
                    }
                    _ => false,
                };
                (name, pat, is_optional)
            })
            .and_then(|(name, pat, is_optional)| {
                match (is_optional, parsing_optionals) {
                    (false, false) => {
                        required_args.push((name.ident, pat.ty));
                        Ok(())
                    }
                    (false, true) => Err(Error::new_spanned(
                        name,
                        "Non-optional values must be placed before optionals",
                    )),
                    (true, _) => {
                        optional_args.push((name.ident, extract_type_from_option(pat.ty)?));
                        parsing_optionals = true;
                        Ok(())
                    }
                }
            })?;
        }

        // let generics =
        //     GenericGenerator::from_generics(orig.sig.generics.clone(), required_args.len());

        Ok(Self {
            vis: orig.vis.clone(),
            return_type: orig.sig.output.clone(),
            name: orig.sig.ident.clone(),
            // generics,
            original: orig,
            required_args,
            optional_args,
        })
    }
}

impl ToTokens for OptFn2 {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let OptFn2 {
            original,
            required_args,
            optional_args,
            vis,
            name,
            return_type,
            ..
        } = self;

        // let hidden_name

        let toks = quote! {
            #original
            // use crate::#name;
            mod builder {
                #[doc(hidden)]
                #[macro_export]
                macro_rules! #name {
                    ($($key:ident $(: $value:expr)? ), *) => {
                        ();
                        // ExampleBuilder::default()
                        // $(.$key( some_helper!($key $(, $value)?)  ))*
                        // .build()
                    };
                }

                #[macro_export]
                macro_rules! some_helper {
                    ($key:ident, $value:expr) => {
                        $value
                    };
                    ($key:ident) => {
                        $key
                    };
                }

                #[macro_export]
                macro_rules! specialfield {
                    ($key:expr, $value:expr) => {
                        $value
                    };
                    ($key:expr) => {
                        $key
                    };
                }
            }
        };

        toks.to_tokens(tokens);
    }
}

// https://stackoverflow.com/questions/55271857/how-can-i-get-the-t-from-an-optiont-when-using-syn
fn extract_type_from_option(ty: Box<Type>) -> Result<Box<Type>> {
    // todo: allow other option types (probably generated by macro)
    fn path_is_option(path: &Path) -> bool {
        path.leading_colon.is_none()
            && path.segments.len() == 1
            && path.segments.iter().next().unwrap().ident == "Option"
    }

    match ty.as_ref() {
        Type::Path(typepath) if typepath.qself.is_none() && path_is_option(&typepath.path) => {
            // Get the first segment of the path (there is only one, in fact: "Option"):

            let type_params = &typepath.path.segments.iter().next().unwrap().arguments;

            // It should have only on angle-bracketed param ("<String>"):
            let generic_arg = match type_params {
                PathArguments::AngleBracketed(params) => params.args.iter().next().unwrap(),
                _ => panic!("TODO: error handling"),
            };

            // This argument must be a type:
            match generic_arg {
                GenericArgument::Type(ty) => Ok(Box::new(ty.clone())),
                _ => panic!("TODO: error handling"),
            }
        }
        _ => panic!("TODO: error handling"),
    }
}
