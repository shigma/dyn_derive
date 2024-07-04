use std::mem::replace;

use pretty_assertions::assert_eq;
use prettyplease::unparse;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::dyn_trait::transform;

fn format(input: TokenStream) -> String {
    unparse(&syn::parse2(input).unwrap())
}

pub fn assert_transform(input: TokenStream, output: TokenStream) {
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
