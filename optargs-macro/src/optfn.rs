use std::str::FromStr;

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::{Error, FnArg, GenericArgument, Ident, ItemFn, Path, PathArguments, Result, Type};

type BuilderField = (Ident, Box<Type>);

pub struct OptFn {
    original: ItemFn,
    required_args: Vec<BuilderField>,
    optional_args: Vec<BuilderField>,
    name: Ident,
}

impl Parse for OptFn {
    /*

    We care about:
    - all the fields
    - that all "optional" fields come after required/positional fields
    - the original
    - fields that are required
    - fields that are optional
    */

    fn parse(input: ParseStream) -> Result<Self> {
        let orig: ItemFn = input.parse()?;

        // start by parsing positionals
        // optionals must come after positionals
        let mut parsing_optionals = false;
        let (mut required_args, mut optional_args) = (Vec::new(), Vec::new());

        for arg in orig.sig.inputs.clone() {
            match arg {
                FnArg::Typed(arg) => Ok(arg),
                FnArg::Receiver(r) => Err(Error::new_spanned(r, "optfn cannot be used on methods")),
            }
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

        Ok(Self {
            name: orig.sig.ident.clone(),
            original: orig,
            required_args,
            optional_args,
        })
    }
}

impl ToTokens for OptFn {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let OptFn {
            original,
            required_args,
            optional_args,
            name,
            ..
        } = self;

        let helper_defs = required_args
            .iter()
            .chain(optional_args.iter())
            .enumerate()
            .map(|(id, (arg, _ty))| {
                let id = syn::Index::from(id);
                quote! {
                    (@setter_helper $src:ident #arg $key:ident) => {
                        $src.#id = ::core::option::Option::Some($key);
                    };
                    (@setter_helper $src:ident #arg $key:ident $value:expr) => {
                        $src.#id = ::core::option::Option::Some($value);
                    };
                }
            });

        let inners_body = required_args
            .iter()
            .chain(optional_args.iter())
            .map(|_| quote! {::core::option::Option::None,});

        let call_body = required_args
            .iter()
            .map(|f| (true, f))
            .chain(optional_args.iter().map(|f| (false, f)))
            .enumerate()
            .map(|(id, (required, (_, ty)))| {
                let id = syn::Index::from(id);
                match required {
                    true => quote! {inners.#id.unwrap() as #ty,},
                    false => quote! { inners.#id, },
                }
            });

        let validator =
            GenericGenerator::new(required_args.len()).generate(required_args, optional_args);

        let ty_expanse = required_args
            .iter()
            .chain(optional_args.iter())
            .map(|(_, ty)| quote! { ::core::option::Option<#ty>, });

        ToTokens::to_tokens(
            &quote! {
                #original

                #[doc(hidden)]
                #[macro_export]
                macro_rules! #name {
                    ($($key:ident $(: $value:expr)? ), * $(,)?) => {
                        {
                            #[allow(unused_mut)]
                            let mut inners: (#( #ty_expanse)*) = (#( #inners_body )*);
                            { $( #name! (@setter_helper inners $key $key $($value)? ); )* }
                            #validator

                            #[allow(unused_mut)]
                            let mut validator = Validator::builder();
                            validator $(.$key())* .build();
                            #name(#( #call_body )*)
                        }
                    };
                    #( #helper_defs )*
                }

            },
            tokens,
        );
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

/*
This struct lets us generate the correct const generics form depending on the arguments.
---
So we can turn this function:

    fn blah(a: u32, b: Option<u32>){}

into

    impl Builder<false> {
                ^^^^^^^ -- this gets generated from a method
        fn a(self) -> Builder<true> {
                             ^^^^^^ -- this gets generated from a method
            ...
        }
    }
--
It's important to keep the original generics, and add any lifetimes for fields that start with &'_os.
To do this, we always generate a borrowed lifetime and let the builder automatically add in the '_os lifetime
*/
struct GenericGenerator {
    num_args: usize,
}

impl GenericGenerator {
    fn new(num_args: usize) -> Self {
        Self { num_args }
    }

    // generate the generics for an all-generic const
    // used in the struct position
    /*
        pub struct ExampleBuilder<const M0: bool> {
                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^ this bit gets generated
            a: Option<u32>,
            b: Option<&'_o str>,
        }
    */
    fn gen_all_generic(&self, exclude: usize) -> TokenStream2 {
        let mut inner = quote! {};
        for id in 0..self.num_args {
            let idref = TokenStream2::from_str(format!("M{}", id).as_str()).unwrap();
            if id != exclude {
                inner.append_all(quote! { const #idref: bool, });
            }
        }
        quote! { <#inner> }
    }

    // generate all the const generics with the same marker
    /*
    impl ExampleBuilder<true, M1> {
                       ^^^^^^^^^^^^^^^^^^^^ generate this part
        fn call(self) #ret {
            #inner(#callerargs)
        }
    }
    */
    fn gen_all(&self, marker: bool) -> TokenStream2 {
        let mut inner = quote! {};
        for _ in 0..self.num_args {
            inner.append_all(quote! { #marker, });
        }
        quote! { <#inner> }
    }

    // generate all as generic, except for a single position with the marker
    /*
    impl<const A: bool> ExampleBuilder<false, A> {
                                      ^^^^^^^^^^^^^^^ gen this
        fn call(self, val: #ty) -> ExampleBuilder<true, A> {
            ...
        }
    }
    */
    fn gen_positional(&self, position: usize, marker: bool) -> TokenStream2 {
        //
        let mut inner = quote! {};
        for id in 0..self.num_args {
            if id == position {
                inner.append_all(quote! { #marker, });
            } else {
                let mtok = TokenStream2::from_str(format!("M{}", id).as_str()).unwrap();
                inner.append_all(quote! { #mtok, });
            }
        }

        quote! { <#inner> }
    }

    fn generate(
        &self,
        required_args: &Vec<BuilderField>,
        optional_args: &Vec<BuilderField>,
    ) -> TokenStream2 {
        let impl_generics = self.gen_all_generic(usize::MAX);
        let ty_gen = self.gen_all(false);
        let builder_builder = quote! {
            #[derive(Default)]
            struct Validator #impl_generics;
            impl Validator #ty_gen {
                fn builder() -> Validator #ty_gen { Validator::default() }
            }
        };

        let mut builders = TokenStream2::new();
        for (id, (name, _ty)) in required_args.iter().enumerate() {
            let impl_generics = self.gen_all_generic(id);
            let ty_gen_in = self.gen_positional(id, false);
            let ty_gen_out = self.gen_positional(id, true);
            builders.append_all(quote! {
                impl #impl_generics Validator #ty_gen_in {
                    fn #name(self) -> Validator #ty_gen_out { unsafe {::core::mem::transmute(self)} }
                }
            })
        }

        let impl_generics = self.gen_all_generic(usize::MAX);
        let ty_gen = self.gen_positional(usize::MAX, false);
        for (name, _ty) in optional_args {
            builders.append_all(quote! {
                impl #impl_generics Validator #ty_gen {
                    #[allow(unused)]
                    fn #name(self) -> Validator #ty_gen { self }
                }
            })
        }

        let ty_gen = self.gen_all(true);
        let caller = quote! {
            impl Validator #ty_gen {
                fn build(self) {}
            }
        };

        quote! {
            #builder_builder
            #builders
            #caller
        }
    }
}
