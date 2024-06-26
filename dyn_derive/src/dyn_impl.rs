use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub fn main(input: TokenStream) -> TokenStream {
    let mut item: syn::ItemTrait = syn::parse2(input).expect("expect trait");
    for param in &mut item.supertraits {
        let syn::TypeParamBound::Trait(bound) = param else {
            continue;
        };
        if bound.path.is_ident("Clone") {
            bound.path = syn::parse_quote! { dyn_traits::DynClone };
        } else if bound.path.is_ident("PartialEq") {
            bound.path = syn::parse_quote! { dyn_traits::DynPartialEq };
        } else if bound.path.is_ident("PartialOrd") {
            bound.path = syn::parse_quote! { dyn_traits::DynPartialEq };
        } else if bound.path.is_ident("Neg") {
            bound.path = syn::parse_quote! { dyn_traits::DynNeg };
        } else if bound.path.is_ident("Not") {
            bound.path = syn::parse_quote! { dyn_traits::DynNot };
        } else if bound.path.is_ident("Add") {
            bound.path = syn::parse_quote! { dyn_traits::DynAdd };
        } else if bound.path.is_ident("Sub") {
            bound.path = syn::parse_quote! { dyn_traits::DynSub };
        } else if bound.path.is_ident("Mul") {
            bound.path = syn::parse_quote! { dyn_traits::DynMul };
        } else if bound.path.is_ident("Div") {
            bound.path = syn::parse_quote! { dyn_traits::DynDiv };
        } else if bound.path.is_ident("Rem") {
            bound.path = syn::parse_quote! { dyn_traits::DynRem };
        } else if bound.path.is_ident("BitAnd") {
            bound.path = syn::parse_quote! { dyn_traits::DynBitAnd };
        } else if bound.path.is_ident("BitOr") {
            bound.path = syn::parse_quote! { dyn_traits::DynBitOr };
        } else if bound.path.is_ident("BitXor") {
            bound.path = syn::parse_quote! { dyn_traits::DynBitXor };
        } else if bound.path.is_ident("Shl") {
            bound.path = syn::parse_quote! { dyn_traits::DynShl };
        } else if bound.path.is_ident("Shr") {
            bound.path = syn::parse_quote! { dyn_traits::DynShr };
        } else if bound.path.is_ident("AddAssign") {
            bound.path = syn::parse_quote! { dyn_traits::DynAddAssign };
        } else if bound.path.is_ident("SubAssign") {
            bound.path = syn::parse_quote! { dyn_traits::DynSubAssign };
        } else if bound.path.is_ident("MulAssign") {
            bound.path = syn::parse_quote! { dyn_traits::DynMulAssign };
        } else if bound.path.is_ident("DivAssign") {
            bound.path = syn::parse_quote! { dyn_traits::DynDivAssign };
        } else if bound.path.is_ident("RemAssign") {
            bound.path = syn::parse_quote! { dyn_traits::DynRemAssign };
        } else if bound.path.is_ident("BitAndAssign") {
            bound.path = syn::parse_quote! { dyn_traits::DynBitAndAssign };
        } else if bound.path.is_ident("BitOrAssign") {
            bound.path = syn::parse_quote! { dyn_traits::DynBitOrAssign };
        } else if bound.path.is_ident("BitXorAssign") {
            bound.path = syn::parse_quote! { dyn_traits::DynBitXorAssign };
        } else if bound.path.is_ident("ShlAssign") {
            bound.path = syn::parse_quote! { dyn_traits::DynShlAssign };
        } else if bound.path.is_ident("ShrAssign") {
            bound.path = syn::parse_quote! { dyn_traits::DynShrAssign };
        }
    }
    item.to_token_stream()
}
