use darling::{ast::NestedMeta, Error, FromMeta};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Ident, Type};

mod parse;
use car_core::command::{Argument, Parameter, ParameterType};
use parse::FunctionParse;

#[derive(Debug, FromMeta)]
struct CommandMacroArgs {
    name: String,
    description: String,
    #[darling(default, multiple)]
    parameter: Vec<ParameterMacroArgs>,
}

#[derive(Debug, FromMeta)]
struct ParameterMacroArgs {
    name: String,
    description: String,
}

#[proc_macro_attribute]
pub fn command(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(attr.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };
    let attr_args = match CommandMacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };
    let FunctionParse {
        attributes: _,
        visibility,
        name: fn_name,
        fn_parameters,
        output,
        body,
    } = parse_macro_input!(input as FunctionParse);

    assert!(attr_args.parameter.len() == fn_parameters.len() - 1);

    let mut parameters = Vec::new();
    let mut fn_parameter_names = Vec::new();
    let mut arg_conversions = Vec::new();

    for (fn_parameter, parameter_macro_args) in
        (&fn_parameters[1..]).iter().zip(&attr_args.parameter)
    {
        let (required, arg_type, parameter_type) = match &fn_parameter.kind {
            Type::Path(ty) => {
                let ident_str = ty.to_token_stream().to_string();
                (
                    ident_str.starts_with("Option"),
                    Argument::quote_from_fn_parameter(ident_str.as_str()),
                    ParameterType::from_fn_parameter(ident_str.as_str()),
                )
            }
            _ => panic!("invalid parameter type"),
        };

        parameters.push(
            Parameter::builder()
                .name(&parameter_macro_args.name)
                .description(&parameter_macro_args.description)
                .kind(parameter_type)
                .required(required)
                .build(),
        );

        fn_parameter_names.push(&fn_parameter.name);

        let var_name = &fn_parameter.name;
        let arg_name = &parameter_macro_args.name;
        arg_conversions.push(quote! {
            let mut #var_name = match args.remove(#arg_name).unwrap() {
                #arg_type(x) => x,
                _ => panic!("argument type mismatched")
            };
        });
    }

    let build = Ident::new(&format!("car_build_{}", &fn_name), fn_name.span());
    let wrap = Ident::new(&format!("car_wrap_{}", &fn_name), fn_name.span());
    let function = Ident::new(&format!("car_fn_{}", &fn_name), fn_name.span());

    let command_name = attr_args.name;
    let command_description = attr_args.description;

    (quote! {
        #visibility fn #build() -> car::Command {
            car::Command::builder()
                .name(#command_name)
                .description(#command_description)
                #(.parameter(#parameters))*
                .function(#wrap as car::CommandFunction)
                .build()
        }
        #visibility fn #wrap(ctx: car::Context, mut args: std::collections::HashMap<String, car::Argument>) -> car::BoxedFuture<#output> {
            #(#arg_conversions)*
            #function(ctx, #(#fn_parameter_names),*)
        }
        #visibility fn #function(#(#fn_parameters),*) -> car::BoxedFuture<#output> {
            Box::pin(async move {
                #(#body)*
            })
        }
    }).into()
}
