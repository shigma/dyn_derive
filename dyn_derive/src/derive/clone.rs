use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::utils::is_dyn;

pub fn derive(input: TokenStream) -> TokenStream {
    let item: syn::DeriveInput = syn::parse2(input).expect("expect struct or enum");
    let item_ident = &item.ident;
    let item_data = &item.data;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
    let output = match item_data {
        syn::Data::Struct(data) => clone_struct(data),
        syn::Data::Enum(data) => clone_enum(data),
        syn::Data::Union(_) => panic!("cannot derive dyn traits for unions"),
    };
    quote! {
        impl #impl_generics Clone for #item_ident #ty_generics #where_clause {
            fn clone(&self) -> Self {
                #output
            }
        }
    }
}

fn clone_field_iter<'i, T: Iterator<Item = (syn::Ident, TokenStream, &'i syn::Field)>>(iter: T, wrap: fn(TokenStream) -> TokenStream) -> Option<(TokenStream, TokenStream)> {
    iter.fold(None, |acc, (name, pref, field)| {
        let expr = if is_dyn(&field.ty) {
            quote! { dyn_traits::ptr::convert_to_box(#name, dyn_traits::DynClone::dyn_clone) }
        } else {
            quote! { #name.clone() }
        };
        acc.map(|(acc0, acc1)| (
            quote! { #acc0, #name },
            quote! { #acc1, #pref #expr },
        )).or(Some((quote! { #name }, quote! { #pref #expr })))
    }).map(|(patt, expr)| (wrap(patt), wrap(expr)))
}

fn cmp_fields(fields: &syn::Fields) -> Option<(TokenStream, TokenStream)> {
    match &fields {
        syn::Fields::Named(fields) => {
            clone_field_iter(fields.named.iter().map(|f| {
                let ident = f.ident.as_ref().expect("ident");
                (ident.clone(), quote!(#ident:), f)
            }), |ts| quote! { {#ts} })
        },
        syn::Fields::Unnamed(fields) => {
            clone_field_iter(fields.unnamed.iter().enumerate().map(|(index, f)| {
                (format_ident!("v{}", index), quote!(), f)
            }), |ts| quote! { (#ts) })
        },
        syn::Fields::Unit => None,
    }
}

fn clone_struct(data: &syn::DataStruct) -> TokenStream {
    match cmp_fields(&data.fields) {
        Some((patt, expr)) => quote! {
            match self {
                Self #patt => Self #expr,
            }
        },
        None => quote! { true }, // todo
    }
}

fn clone_enum(data: &syn::DataEnum) -> TokenStream {
    if data.variants.is_empty() {
        return quote! { true }
    }
    let iter = data.variants.iter().map(|variant| {
        let ident = &variant.ident;
        match cmp_fields(&variant.fields) {
            Some((patt, expr)) => quote! {
                Self::#ident #patt => Self::#ident #expr,
            },
            None => quote! {
                Self::#ident => true,
            },
        }
    });
    quote! {
        match self {
            #(#iter)*
        }
    }
}
