use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

fn make_dyn(ident: &impl ToTokens, is_ref: bool) -> syn::Type {
    if is_ref {
        syn::parse_quote! { dyn #ident }
    } else {
        syn::parse_quote! { Box<dyn #ident> }
    }
}

pub struct GenericsData {
    pub name: TokenStream,
    pub items: Vec<syn::TraitItem>,
    pub data: HashMap<String, syn::TraitItemType>,
}

impl GenericsData {
    pub fn from(name: TokenStream, fact: &mut syn::ItemTrait) -> Self {
        let mut data = HashMap::new();
        let mut items = Vec::new();
        for item in &mut fact.items {
            let syn::TraitItem::Type(ty) = item else {
                items.push(item.clone());
                continue;
            };
            let index = ty.attrs.iter().position(|attr| {
                attr.meta.path().is_ident("dyn_trait")
            });
            if let Some(index) = index {
                ty.attrs.remove(index);
            } else {
                items.push(syn::TraitItem::Type(ty.clone()));
                continue;
            }
            // todo: multiple bounds
            let mut ty = ty.clone();
            for bound in &mut ty.bounds {
                match bound {
                    syn::TypeParamBound::Trait(bound) => {
                        let last = bound.path.segments.last_mut().unwrap();
                        last.ident = format_ident!("{}Instance", last.ident);
                    },
                    _ => {},
                }
            }
            data.insert(ty.ident.to_string(), ty);
        }
        Self { name, items, data }
    }

    pub fn test(&self, path: &syn::TypePath, is_ref: bool) -> Option<(syn::Type, TokenStream)> {
        if path.qself.is_some() {
            return None
        }
        let first = path.path.segments.first().unwrap();
        if first.ident != "Self" || path.path.segments.len() > 2 {
            return None
        }
        if path.path.segments.len() == 1 {
            return Some((
                make_dyn(&self.name, is_ref),
                quote! { ::dyn_std::Instance::<Factory> },
            ))
        }
        let last = path.path.segments.last().unwrap();
        let Some(g) = self.data.get(&last.ident.to_string()) else {
            return None
        };
        let ident = &g.ident;
        let bounds = &g.bounds;
        return Some((
            make_dyn(bounds, is_ref),
            quote! { ::dyn_std::Instance::<Factory::#ident> },
        ))
    }
}
