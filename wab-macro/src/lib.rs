use darling::{ast::NestedMeta, Error, FromMeta};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Ident, Type};

mod parse;
use parse::{FunctionParse, StructParse};

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
    choice: Vec<ChoiceMacroArgs>,
    min_value_int: Option<i64>,
    max_value_int: Option<i64>,
    min_value_number: Option<f64>,
    max_value_number: Option<f64>,
    min_length: Option<i32>,
    max_length: Option<i32>,
}

#[derive(Debug, FromMeta)]
struct ChoiceMacroArgs {
    name: String,
    value_string: Option<String>,
    value_int: Option<i64>,
    value_number: Option<f64>,
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
        let required;
        let (arg_type, parameter_type) = match &fn_parameter.kind {
            Type::Path(ty) => {
                let ident_str = ty.to_token_stream().to_string();
                required = !ident_str.starts_with("Option");
                match ident_str.as_str() {
                    "String" | "Option < String >" => (
                        quote! {wab::Argument::String},
                        quote! {wab::ParameterType::String},
                    ),
                    "i64" | "Option < i64 >" => (
                        quote! {wab::Argument::Integer},
                        quote! {wab::ParameterType::Integer},
                    ),
                    "f64" | "Option < f64 >" => (
                        quote! {wab::Argument::Float},
                        quote! {wab::ParameterType::Float},
                    ),
                    "bool" | "Option < bool >" => (
                        quote! {wab::Argument::Boolean},
                        quote! {wab::ParameterType::Boolean},
                    ),
                    _ => panic!("invalid parameter type"),
                }
            }
            _ => panic!("invalid parameter type"),
        };

        let ParameterMacroArgs {
            name: arg_name,
            description,
            choice,
            min_value_int,
            max_value_int,
            min_value_number,
            max_value_number,
            min_length,
            max_length,
        } = parameter_macro_args;

        let choices: Vec<TokenStream2> = choice
            .into_iter()
            .map(|x| {
                let name = x.name;
                if let Some(value) = x.value_int {
                    quote! {wab::ParameterChoice::new(#name, wab::ParameterChoiceType::Integer(#value))}
                } else if let Some(value) = x.value_number {
                    quote! {wab::ParameterChoice::new(#name, wab::ParameterChoiceType::Float(#value))}
                } else if let Some(value) = x.value_string {
                    quote! {wab::ParameterChoice::new(#name, wab::ParameterChoiceType::String(String::from(#value)))}
                } else {
                    panic!("Expected value in parameter choice");
                }
            })
            .collect();

        let min_value_int = quote_option(&min_value_int);
        let max_value_int = quote_option(&max_value_int);
        let min_value_number = quote_option(&min_value_number);
        let max_value_number = quote_option(&max_value_number);
        let min_length = quote_option(&min_length);
        let max_length = quote_option(&max_length);

        parameters.push(quote! {
            wab::Parameter::builder()
                .name(#arg_name)
                .description(#description)
                .kind(#parameter_type)
                .required(#required)
                #(.choice(#choices))*
                .min_value_int(#min_value_int)
                .max_value_int(#max_value_int)
                .min_value_number(#min_value_number)
                .max_value_number(#max_value_number)
                .min_length(#min_length)
                .max_length(#max_length)
                .build()
        });
        fn_parameter_names.push(&fn_parameter.name);

        let var_name = &fn_parameter.name;
        let arg_conversion = if required {
            quote! {
                let mut #var_name = match args.remove(#arg_name) {
                    Some(#arg_type(x)) => x,
                    None | _ => panic!("argument type mismatched")
                };
            }
        } else {
            quote! {
                let mut #var_name = match args.remove(#arg_name) {
                    Some(#arg_type(x)) => Some(x),
                    None => None,
                    _ => panic!("argument type mismatched")
                };
            }
        };
        arg_conversions.push(arg_conversion);
    }

    let builder = Ident::new(&format!("wab_builder_{}", &fn_name), fn_name.span());
    let wrap = Ident::new(&format!("wab_wrap_{}", &fn_name), fn_name.span());
    let function = Ident::new(&format!("wab_fn_{}", &fn_name), fn_name.span());

    let command_name = attr_args.name;
    let command_description = attr_args.description;

    (quote! {
        #visibility fn #builder() -> wab::CommandBuilder {
            wab::Command::builder()
                .name(#command_name)
                .description(#command_description)
                #(.parameter(#parameters))*
                .function(#wrap as wab::CommandFunction)
        }
        fn #wrap(ctx: wab::Context, mut args: std::collections::HashMap<String, wab::Argument>) -> wab::BoxedFuture<#output> {
            #(#arg_conversions)*
            #function(ctx, #(#fn_parameter_names),*)
        }
        fn #function(#(#fn_parameters),*) -> wab::BoxedFuture<#output> {
            Box::pin(async move {
                #(#body)*
            })
        }
    }).into()
}

#[proc_macro_attribute]
pub fn box_async(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let FunctionParse {
        attributes: _,
        visibility,
        name,
        fn_parameters,
        output,
        body,
    } = parse_macro_input!(input as FunctionParse);
    (quote! {
        #visibility fn #name(#(#fn_parameters),*) -> wab::BoxedFuture<#output> {
            Box::pin(async move {
                #(#body)*
            })
        }
    })
    .into()
}

fn quote_option<T: ToTokens>(option: &Option<T>) -> TokenStream2 {
    if option.is_some() {
        quote! {Some(#option)}
    } else {
        quote! {None}
    }
}

#[derive(Debug)]
struct IdentList {
    idents: Vec<Ident>,
}
impl FromMeta for IdentList {
    fn from_list(items: &[NestedMeta]) -> Result<Self, darling::Error> {
        let mut idents = Vec::new();

        for item in items {
            match item {
                NestedMeta::Meta(syn::Meta::Path(path)) => {
                    if let Some(ident) = path.get_ident() {
                        idents.push(ident.clone());
                    } else {
                        return Err(darling::Error::unexpected_type(
                            &path.to_token_stream().to_string(),
                        ));
                    }
                }
                _ => {
                    return Err(darling::Error::unexpected_type(
                        &item.to_token_stream().to_string(),
                    ));
                }
            }
        }

        Ok(IdentList { idents })
    }
}

#[derive(Debug, FromMeta)]
struct GroupMacroArgs {
    category: String,
    commands: IdentList,
    init: Option<Ident>,
}

#[proc_macro_attribute]
pub fn group(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(attr.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };
    let attr_args = match GroupMacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };
    let StructParse {
        visibility,
        name: struct_name,
    } = parse_macro_input!(input as StructParse);

    let name_string = camel_to_snake_case(&struct_name.to_string());
    let name = Ident::new(&name_string.to_ascii_uppercase(), struct_name.span());
    let build_commands = Ident::new(
        &format!("wab_group_commands_{}", &name_string),
        struct_name.span(),
    );

    let command_builders: Vec<Ident> = attr_args
        .commands
        .idents
        .into_iter()
        .map(|x| Ident::new(&format!("wab_builder_{}", x.to_string()), x.span()))
        .collect();
    let category = attr_args.category;

    let init = quote_option(&attr_args.init);

    (quote! {
        fn #build_commands() -> Vec<wab::Command> {
            let mut commands = Vec::new();
            #(commands.push(
                #command_builders()
                    .category(#category)
                    .build()
            );)*
            commands
        }
        #visibility static #name: wab::Group = wab::Group {
            build_commands: #build_commands,
            init: #init
        };
    })
    .into()
}

fn camel_to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(c.to_ascii_lowercase());
    }
    result
}
