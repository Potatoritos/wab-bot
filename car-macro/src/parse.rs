use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{
    braced, parenthesized, parse::Parse, parse::ParseStream, Attribute, Block, Ident, ReturnType,
    Stmt, Token, Type, Visibility,
};

pub struct FunctionParameter {
    pub mutable: bool,
    pub name: Ident,
    pub kind: Type,
}
impl ToTokens for FunctionParameter {
    fn to_tokens(&self, stream: &mut TokenStream2) {
        let Self {
            mutable,
            name,
            kind,
        } = self;

        if *mutable {
            stream.extend(quote! { mut #name: #kind });
        } else {
            stream.extend(quote! { #name: #kind });
        }
    }
}
pub struct FunctionParse {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub name: Ident,
    pub fn_parameters: Vec<FunctionParameter>,
    pub output: Type,
    pub body: Vec<Stmt>,
}

impl Parse for FunctionParse {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attributes = input.call(Attribute::parse_outer)?;
        let visibility: Visibility = input.parse()?;
        input.parse::<Token![async]>()?;
        input.parse::<Token![fn]>()?;
        let name: Ident = input.parse()?;

        let content;
        parenthesized!(content in input);
        let mut fn_parameters = Vec::new();
        while !content.is_empty() {
            let mut mutable = false;
            if content.peek(Token![mut]) {
                content.parse::<Token![mut]>()?;
                mutable = true;
            }

            let name: Ident = content.parse()?;
            content.parse::<Token![:]>()?;
            let kind: Type = content.parse()?;
            fn_parameters.push(FunctionParameter {
                mutable,
                name,
                kind,
            });
            if !content.is_empty() {
                content.parse::<Token![,]>()?;
            }
        }

        let output = match input.parse::<ReturnType>()? {
            ReturnType::Type(_, ty) => (*ty).clone(),
            ReturnType::Default => syn::Type::Verbatim(quote!{()}),
        };

        let body;
        braced!(body in input);
        let body = body.call(Block::parse_within)?;

        Ok(Self {
            attributes,
            visibility,
            name,
            fn_parameters,
            output,
            body,
        })
    }
}

pub struct StructParse {
    pub visibility: Visibility,
    pub name: Ident
}
impl Parse for StructParse {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let visibility: Visibility = input.parse()?;
        input.parse::<Token![struct]>()?;
        let name: Ident = input.parse()?;
        input.parse::<Token![;]>()?;
        Ok(Self {
            visibility,
            name
        })
    }
}