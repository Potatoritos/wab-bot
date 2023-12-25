use darling::{ast::NestedMeta, Error, FromMeta};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Ident, Type};

mod parse;
use parse::{FunctionParse, StructParse};

#[derive(Debug, FromMeta)]
struct Test {}

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
    max_length: Option<i32>,
}

#[derive(Debug, FromMeta)]
struct ChoiceMacroArgs<T> {
    name: String,
    value: T,
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
                    "String" => (
                        quote! {car::Argument::String},
                        quote! {car::ParameterType::String},
                    ),
                    "Option < String >" => (
                        quote! {car::Argument::OptionalString},
                        quote! {car::ParameterType::String},
                    ),
                    "i64" => (
                        quote! {car::Argument::Int},
                        quote! {car::ParameterType::Int},
                    ),
                    "Option < i64 >" => (
                        quote! {car::Argument::OptionalInt},
                        quote! {car::ParameterType::Int},
                    ),
                    "f64" => (
                        quote! {car::Argument::Number},
                        quote! {car::ParameterType::Number},
                    ),
                    "Option < f64 >" => (
                        quote! {car::Argument::OptionalNumber},
                        quote! {car::ParameterType::Number},
                    ),
                    "bool" => (
                        quote! {car::Argument::Bool},
                        quote! {car::ParameterType::Bool},
                    ),
                    "Option < bool >" => (
                        quote! {car::Argument::OptionalBool},
                        quote! {car::ParameterType::Bool},
                    ),
                    _ => panic!("invalid parameter type"),
                }
            }
            _ => panic!("invalid parameter type"),
        };

        let ParameterMacroArgs {
            name: arg_name,
            description,
            choice_string,
            choice_int,
            choice_number,
            min_value_int,
            max_value_int,
            min_value_number,
            max_value_number,
            min_length,
            max_length,
        } = parameter_macro_args;

        let choices_string: Vec<TokenStream2> = choice_string
            .into_iter()
            .map(|x| {
                let name = x.name;
                let value = x.value;
                quote! {car::ParameterChoice::<String>::new(#name, #value)}
            })
            .collect();
        let choices_int: Vec<TokenStream2> = choice_int
            .into_iter()
            .map(|x| {
                let name = x.name;
                let value = x.value;
                quote! {car::ParameterChoice::<i64>::new(#name, #value)}
            })
            .collect();
        let choices_number: Vec<TokenStream2> = choice_number
            .into_iter()
            .map(|x| {
                let name = x.name;
                let value = x.value;
                quote! {car::ParameterChoice::<i64>::new(#name, #value)}
            })
            .collect();

        let min_value_int = quote_option(&min_value_int);
        let max_value_int = quote_option(&max_value_int);
        let min_value_number = quote_option(&min_value_number);
        let max_value_number = quote_option(&max_value_number);
        let min_length = quote_option(&min_length);
        let max_length = quote_option(&max_length);

        parameters.push(quote! {
            car::Parameter::builder()
                .name(#arg_name)
                .description(#description)
                .kind(#parameter_type)
                .required(#required)
                #(.choice_string(#choices_string))*
                #(.choice_int(#choices_int))*
                #(.choice_number(#choices_number))*
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
        arg_conversions.push(quote! {
            let mut #var_name = match args.remove(#arg_name).unwrap() {
                #arg_type(x) => x,
                _ => panic!("argument type mismatched")
            };
        });
    }

    let builder = Ident::new(&format!("car_builder_{}", &fn_name), fn_name.span());
    let wrap = Ident::new(&format!("car_wrap_{}", &fn_name), fn_name.span());
    let function = Ident::new(&format!("car_fn_{}", &fn_name), fn_name.span());

    let command_name = attr_args.name;
    let command_description = attr_args.description;

    (quote! {
        #visibility fn #builder() -> car::CommandBuilder {
            car::Command::builder()
                .name(#command_name)
                .description(#command_description)
                #(.parameter(#parameters))*
                .function(#wrap as car::CommandFunction)
        }
        fn #wrap(ctx: car::Context, mut args: std::collections::HashMap<String, car::Argument>) -> car::BoxedFuture<#output> {
            #(#arg_conversions)*
            #function(ctx, #(#fn_parameter_names),*)
        }
        fn #function(#(#fn_parameters),*) -> car::BoxedFuture<#output> {
            Box::pin(async move {
                #(#body)*
            })
        }
    }).into()
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
        &format!("car_group_commands_{}", &name_string),
        struct_name.span(),
    );
    let setup = Ident::new(&format!("car_setup_{}", &name_string), struct_name.span());

    let command_builders: Vec<Ident> = attr_args
        .commands
        .idents
        .into_iter()
        .map(|x| Ident::new(&format!("car_builder_{}", x.to_string()), x.span()))
        .collect();
    let category = attr_args.category;

    (quote! {
        fn #build_commands() -> Vec<car::Command> {
            let mut commands = Vec::new();
            #(commands.push(
                #command_builders()
                    .category(#category)
                    .build()
            );)*
            commands
        }
        fn #setup() {

        }
        #visibility static #name: car::Group = car::Group {
            build_commands: #build_commands,
            setup: #setup
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
