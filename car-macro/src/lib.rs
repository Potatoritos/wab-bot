use darling::{ast::NestedMeta, Error, FromMeta};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Ident, Type};

mod parse;
use car_core::command::{Argument, Parameter, ParameterType, ParameterChoice};
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
    #[darling(default, multiple)]
    choice_string: Vec<ChoiceMacroArgs<String>>,
    #[darling(default, multiple)]
    choice_int: Vec<ChoiceMacroArgs<i64>>,
    #[darling(default, multiple)]
    choice_number: Vec<ChoiceMacroArgs<f64>>,
    min_value_int: Option<i64>,
    max_value_int: Option<i64>,
    min_value_number: Option<f64>,
    max_value_number: Option<f64>,
    min_length: Option<i32>,
    max_length: Option<i32>
}

#[derive(Debug, FromMeta)]
struct ChoiceMacroArgs<T> {
    name: String,
    value: T
}

impl Into<ParameterChoice<i64>> for ChoiceMacroArgs<i64> {
    fn into(self) -> ParameterChoice<i64> {
        ParameterChoice::<i64>::new(self.name, self.value)
    }
}
impl Into<ParameterChoice<f64>> for ChoiceMacroArgs<f64> {
    fn into(self) -> ParameterChoice<f64> {
        ParameterChoice::<f64>::new(self.name, self.value)
    }
}
impl Into<ParameterChoice<String>> for ChoiceMacroArgs<String> {
    fn into(self) -> ParameterChoice<String> {
        ParameterChoice::<String>::new(self.name, self.value)
    }
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
        (&fn_parameters[1..]).iter().zip(attr_args.parameter)
    {
        let (required, arg_type, parameter_type) = match &fn_parameter.kind {
            Type::Path(ty) => {
                let ident_str = ty.to_token_stream().to_string();
                (
                    !ident_str.starts_with("Option"),
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
                .choices_string(parameter_macro_args.choice_string.into_iter().map(|x| x.into()).collect())
                .choices_int(parameter_macro_args.choice_int.into_iter().map(|x| x.into()).collect())
                .choices_number(parameter_macro_args.choice_number.into_iter().map(|x| x.into()).collect())
                .min_value_int(parameter_macro_args.min_value_int)
                .max_value_int(parameter_macro_args.max_value_int)
                .min_value_number(parameter_macro_args.min_value_number)
                .max_value_number(parameter_macro_args.max_value_number)
                .min_length(parameter_macro_args.min_length)
                .max_length(parameter_macro_args.max_length)
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
