use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::utils::is_dyn;

pub fn derive(input: TokenStream) -> TokenStream {
    let item: syn::DeriveInput = syn::parse2(input).expect("expect struct or enum");
    let item_ident = &item.ident;
    let item_data = &item.data;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
    let output = match item_data {
        syn::Data::Struct(data) => cmp_struct(data),
        syn::Data::Enum(data) => cmp_enum(data),
        syn::Data::Union(_) => panic!("cannot derive dyn traits for unions"),
    };
    quote! {
        impl #impl_generics PartialEq for #item_ident #ty_generics #where_clause {
            fn eq(&self, other: &Self) -> bool {
                #output
            }
        }
    }
}

fn cmp_field_iter<'i, T: Iterator<Item = (String, TokenStream, &'i syn::Field)>>(iter: T, wrap: fn(TokenStream) -> TokenStream) -> Option<(TokenStream, TokenStream, TokenStream)> {
    iter.fold(None, |acc, (name, pref, field)| {
        let lhs = format_ident!("l_{}", name.to_string());
        let rhs = format_ident!("r_{}", name.to_string());
        let expr = if is_dyn(&field.ty) {
            quote! { #lhs.dyn_eq(dyn_traits::Dyn::as_any(#rhs.as_ref())) }
        } else {
            quote! { #lhs.eq(#rhs) }
        };
        let lhs = quote! { #pref #lhs };
        let rhs = quote! { #pref #rhs };
        acc.map(|(acc0, acc1, acc2)| (
            quote! { #acc0, #lhs },
            quote! { #acc1, #rhs },
            quote! { #acc2 && #expr },
        )).or(Some((lhs, rhs, expr)))
    }).map(|(lhs, rhs, expr)| (wrap(lhs), wrap(rhs), expr))
}

fn cmp_fields(fields: &syn::Fields) -> Option<(TokenStream, TokenStream, TokenStream)> {
    match &fields {
        syn::Fields::Named(fields) => {
            cmp_field_iter(fields.named.iter().map(|f| {
                let ident = f.ident.as_ref().expect("ident");
                (ident.to_string(), quote!(#ident:), f)
            }), |ts| quote! { {#ts} })
        },
        syn::Fields::Unnamed(fields) => {
            cmp_field_iter(fields.unnamed.iter().enumerate().map(|(index, f)| {
                (index.to_string(), quote!(), f)
            }), |ts| quote! { (#ts) })
        },
        syn::Fields::Unit => None,
    }
}

fn cmp_struct(data: &syn::DataStruct) -> TokenStream {
    match cmp_fields(&data.fields) {
        Some((lhs, rhs, expr)) => quote! {
            match (self, other) {
                (Self #lhs, Self #rhs) => #expr,
            }
        },
        None => quote! { true },
    }
}

fn cmp_enum(data: &syn::DataEnum) -> TokenStream {
    if data.variants.is_empty() {
        return quote! { true }
    }
    let iter = data.variants.iter().map(|variant| {
        let ident = &variant.ident;
        match cmp_fields(&variant.fields) {
            Some((lhs, rhs, expr)) => quote! {
                (Self::#ident #lhs, Self::#ident #rhs) => #expr,
            },
            None => quote! {
                (Self::#ident, Self::#ident) => true,
            },
        }
    });
    quote! {
        match (self, other) {
            #(#iter)*
        }
    }
}
