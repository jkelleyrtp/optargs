use std::str::FromStr;

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::{DeriveInput, Error, GenericArgument, Ident, Path, PathArguments, Result, Type};

type BuilderField = (Ident, Box<Type>);

pub struct OptStruct {
    conf: OptStructConf,
}

pub(crate) struct OptStructConf {
    pub(crate) name: Ident,
    pub(crate) required_args: Vec<BuilderField>,
    pub(crate) optional_args: Vec<BuilderField>,
}

impl OptStructConf {
    /*

    We care about:
    - all the fields
    - that all "optional" fields come after required/positional fields
    - the original
    - fields that are required
    - fields that are optional
    */

    pub(crate) fn parse(input: ParseStream) -> Result<Self> {
        let input: DeriveInput = input.parse()?;

        let data = match &input.data {
            syn::Data::Struct(a) => Ok(a),
            syn::Data::Enum(_) | syn::Data::Union(_) => Err(syn::Error::new(
                input.ident.span(),
                "Only structs can be created with the optional pattern.",
            )),
        }?;

        let name = input.ident.clone();

        let mut parsing_optionals = false;
        let (mut required_args, mut optional_args) = (Vec::new(), Vec::new());

        for field in &data.fields {
            let syn::Field { ident, ty, .. } = field;

            let ident = ident.clone().ok_or(Error::new_spanned(
                &name,
                "Non-optional values must be placed before optionals",
            ))?;

            // todo: allow attrs like #[default] to be placed on fields that don't have an obvious option
            let is_optional = match ty {
                Type::Path(p) => {
                    if let Some(arg) = p.path.segments.first() {
                        arg.ident.to_string() == "Option"
                    } else {
                        false
                    }
                }
                _ => false,
            };

            match (is_optional, parsing_optionals) {
                (true, _) => {
                    optional_args.push((ident.clone(), extract_type_from_option(ty)?.clone()));
                    parsing_optionals = true;
                }
                (false, false) => required_args.push((ident.clone(), Box::new(ty.clone()))),
                (false, true) => {
                    return Err(Error::new_spanned(
                        &name,
                        "Non-optional values must be placed before optionals",
                    ));
                }
            };
        }

        Ok(Self {
            name,
            optional_args,
            required_args,
        })
    }
}

impl OptStructConf {
    pub(crate) fn prepare_generate_tokens(
        &self,
    ) -> (
        &Ident,
        impl Iterator<Item = TokenStream2> + '_,
        impl Iterator<Item = TokenStream2> + '_,
        impl Iterator<Item = TokenStream2> + '_,
        TokenStream2,
    ) {
        let OptStructConf {
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
                        $src.#id = Some($key);
                    };
                    (@setter_helper $src:ident #arg $key:ident $value:expr) => {
                        $src.#id = Some($value);
                    };
                }
            });

        let inners_body = required_args
            .iter()
            .chain(optional_args.iter())
            .map(|_| quote! {None,});

        let call_body = required_args
            .iter()
            .map(|f| (true, f))
            .chain(optional_args.iter().map(|f| (false, f)))
            .enumerate()
            .map(|(id, (required, (name, _ty)))| {
                let id = syn::Index::from(id);
                match required {
                    true => quote! {
                        #name: inners.#id.unwrap(),
                    },
                    false => quote! {
                        #name: inners.#id,
                    },
                }
            });

        let validator =
            GenericGenerator::new(required_args.len()).generate(required_args, optional_args);

        (name, helper_defs, inners_body, call_body, validator)
    }
}

impl Parse for OptStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        OptStructConf::parse(input).map(|conf| Self { conf })
    }
}

impl ToTokens for OptStruct {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let (name, helper_defs, inners_body, call_body, validator) = OptStructConf::prepare_generate_tokens(&self.conf);

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

                            #name{
                                #( #call_body )*
                            }
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
fn extract_type_from_option(ty: &Type) -> Result<Box<Type>> {
    // todo: allow other option types (probably generated by macro)
    fn path_is_option(path: &Path) -> bool {
        path.leading_colon.is_none()
            && path.segments.len() == 1
            && path.segments.iter().next().unwrap().ident == "Option"
    }

    match ty {
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
