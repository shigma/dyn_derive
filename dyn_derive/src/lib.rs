use proc_macro::TokenStream;

mod derive;

#[proc_macro_derive(DynPartialEq)]
pub fn derive_partial_eq(input: TokenStream) -> TokenStream {
    derive::partial_eq::derive(input.into()).into()
}
