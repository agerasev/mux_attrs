use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Error, Fields, Ident, Path, Result, parse2, spanned::Spanned};

use crate::attrs::FromAttr;

pub fn derive(input: TokenStream) -> Result<TokenStream> {
    let input: DeriveInput = parse2(input)?;

    let attrs = input
        .attrs
        .iter()
        .filter_map(FromAttr::new)
        .collect::<Result<Vec<_>>>()?;
    let [attr] = attrs.try_into().map_err(|_| {
        Error::new(
            Span::call_site(),
            "There must be exactly one `from` attribute",
        )
    })?;

    let dst = input.ident.clone();

    let mut accum = quote! {};
    for src in attr.types {
        if src.get_ident().map(Ident::to_string) != Some(dst.to_string()) {
            let impl_ = derive_single(input.clone(), src)?;
            accum = quote! {
                #accum
                #impl_
            }
        }
    }
    Ok(accum)
}

fn derive_single(input: DeriveInput, src: Path) -> Result<TokenStream> {
    let dst = input.ident;

    let struct_init = match input.data {
        Data::Struct(data) => {
            let fields = list_fields(data.fields);
            quote! {
                let #src #fields = value;
                Self #fields
            }
        }
        Data::Enum(data) => {
            let mut accum = quote! {};
            for variant in data.variants {
                let ident = variant.ident;
                let fields = list_fields(variant.fields);
                accum = quote! {
                    #accum
                    #src::#ident #fields => Self::#ident #fields,
                };
            }
            quote! {
                match value {
                    #accum
                }
            }
        }
        Data::Union(_) => {
            return Err(Error::new(Span::call_site(), "Unions are not supported"));
        }
    };

    Ok(quote! {
        impl From<#src> for #dst {
            fn from(value: #src) -> Self {
                #struct_init
            }
        }
    })
}

fn list_fields(fields: Fields) -> TokenStream {
    match fields {
        Fields::Named(fields) => {
            let mut accum = quote! {};
            for field in fields.named {
                let ident = field.ident;
                accum = quote! { #accum #ident, };
            }
            quote! { { #accum } }
        }
        Fields::Unnamed(fields) => {
            let mut accum = quote! {};
            for (i, field) in fields.unnamed.into_iter().enumerate() {
                let ident = Ident::new(&format!("t{i}"), field.span());
                accum = quote! { #accum #ident, };
            }
            quote! { ( #accum ) }
        }
        Fields::Unit => {
            quote! {}
        }
    }
}
