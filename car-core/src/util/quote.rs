use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub fn quote_option<T: ToTokens>(option: &Option<T>) -> TokenStream {
    if option.is_some() {
        quote! {Some(#option)}
    } else {
        quote! {None}
    }
}
