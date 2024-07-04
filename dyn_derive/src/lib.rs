#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;

#[cfg(not(feature = "extra-cmp-impl"))]
mod derive;
mod dyn_trait;

/// This derive macro has the exact same behavior as `PartialEq`,
/// but it workarounds a strange behavior of the Rust compiler.
/// 
/// For other traits, you can just derive the original trait name.
/// 
/// ## Example
/// 
/// ```
/// use dyn_derive::*;
/// 
/// #[dyn_trait]
/// pub trait Meta: Clone + PartialEq {}
/// 
/// #[derive(Clone, PartialEqFix)]
/// pub struct Foo {
///     meta: Box<dyn Meta>,
/// }
/// ```
#[cfg(not(feature = "extra-cmp-impl"))]
#[proc_macro_derive(PartialEqFix)]
pub fn derive_partial_eq(input: TokenStream) -> TokenStream {
    derive::partial_eq::derive(input.into()).into()
}

/// This is a procedural macro for deriving object-unsafe traits.
/// 
/// ## Example
/// 
/// `Clone` is not object-safe, but with this macro, you can still use `dyn Meta`:
/// 
/// ```
/// use dyn_derive::*;
/// 
/// #[dyn_trait]
/// pub trait Meta: Clone {}
/// 
/// #[derive(Clone)]
/// pub struct Foo {
///     meta: Box<dyn Meta>,
/// }
/// ```
#[proc_macro_attribute]
pub fn dyn_trait(attr: TokenStream, input: TokenStream) -> TokenStream {
    let item = syn::parse2(input.into()).expect("expect trait");
    dyn_trait::transform(attr.into(), item).into()
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;
    use std::mem::replace;
    use std::path::Path;

    use pretty_assertions::assert_eq;
    use prettyplease::unparse;
    use proc_macro2::TokenStream;
    use quote::{quote, ToTokens};
    use walkdir::WalkDir;

    use crate::dyn_trait::transform;

    fn format(input: TokenStream) -> String {
        unparse(&syn::parse2(input).unwrap())
    }

    fn check_transform(input: TokenStream, output: TokenStream) {
        let mut item: syn::ItemTrait = syn::parse2(input).unwrap();
        let attrs = replace(&mut item.attrs, vec![]);
        assert_eq!(attrs.len(), 1);
        assert_eq!(attrs[0].path().to_token_stream().to_string(), "dyn_trait".to_string());
        let attr = match &attrs[0].meta {
            syn::Meta::Path(_) => quote! {},
            syn::Meta::List(list) => list.tokens.clone(),
            syn::Meta::NameValue(_) => unimplemented!(),
        };
        let actual = transform(attr, item);
        assert_eq!(format(actual), format(output));
    }

    #[test]
    fn fixtures() {
        let input_dir = "fixtures/input";
        let output_dir = "fixtures/output";
        for entry in WalkDir::new(input_dir).into_iter().filter_map(Result::ok) {
            let input_path = entry.path();
            if !input_path.is_file() || input_path.extension() != Some("rs".as_ref()) {
                continue;
            }
            let output_path = Path::new(output_dir).join(input_path.strip_prefix(input_dir).unwrap());
            let input = read_to_string(input_path).unwrap();
            let output = read_to_string(&output_path).unwrap();
            check_transform(input.parse().unwrap(), output.parse().unwrap());
        }
    }
}
