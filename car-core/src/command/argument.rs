use proc_macro2::TokenStream;
use quote::quote;

pub enum Argument {
    String(String),
    OptionalString(Option<String>),
    Int(i64),
    OptionalInt(Option<i64>),
    Bool(bool),
    OptionalBool(Option<bool>),
    Number(f64),
    OptionalNumber(Option<f64>),
}

impl Argument {
    pub fn quote_from_fn_parameter(parameter_type: &str) -> TokenStream {
        match parameter_type {
            "String" => quote! { car::Argument::String },
            "Option < String >" => quote! { car::Argument::OptionalString },
            "i64" => quote! { car::Argument::Int },
            "Option < i64 >" => quote! { car::Argument::OptionalInt },
            "f64" => quote! { car::Argument::Number },
            "Option < f64 >" => quote! { car::Argument::OptionalNumber },
            "bool" => quote! { car::Argument::Bool },
            "Option < bool >" => quote! { car::Argument::OptionalBool },
            _ => panic!("invalid parameter type"),
        }
    }
}
