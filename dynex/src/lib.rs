use proc_macro::TokenStream;

mod derive;
mod dyn_impl;

#[proc_macro_derive(PartialEqFix)]
pub fn derive_partial_eq(input: TokenStream) -> TokenStream {
    derive::partial_eq::derive(input.into()).into()
}

#[proc_macro_attribute]
pub fn dyn_impl(attrs: TokenStream, input: TokenStream) -> TokenStream {
    if !attrs.is_empty() {
        panic!("dyn_impl attribute does not accept any arguments")
    }
    dyn_impl::main(input.into()).into()
}
