use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    Attribute, Error, Ident, Meta, Path, Result, Token, parse::Parser, parse2,
    punctuated::Punctuated, spanned::Spanned,
};

#[derive(Clone)]
pub struct FromAttr {
    pub types: Vec<Path>,
}

impl FromAttr {
    pub fn new(attr: &Attribute) -> Option<Result<Self>> {
        if attr.path().is_ident("from") {
            Some(if let Meta::List(meta) = &attr.meta {
                Self::from_args(meta.tokens.clone())
            } else {
                Err(Error::new(attr.span(), "Unsupported attribute format"))
            })
        } else {
            None
        }
    }

    fn from_args(tokens: TokenStream) -> Result<Self> {
        let parser = Punctuated::<Path, Token![,]>::parse_terminated;
        Ok(Self {
            types: parser.parse2(tokens)?.into_iter().collect(),
        })
    }
}

#[derive(Clone, Default)]
pub struct MuxAttr {
    pub default: Option<TokenStream>,
    pub entries: HashMap<Ident, TokenStream>,
}

impl MuxAttr {
    pub fn new(attr: &Attribute) -> Option<Result<Self>> {
        if attr.path().is_ident("mux") {
            Some(if let Meta::List(meta) = &attr.meta {
                Self::from_args(meta.tokens.clone())
            } else {
                Err(Error::new(attr.span(), "Unsupported attribute format"))
            })
        } else {
            None
        }
    }

    fn from_args(tokens: TokenStream) -> Result<Self> {
        let parser = Punctuated::<Meta, Token![,]>::parse_terminated;
        let entries = parser.parse2(tokens)?;

        let mut this = Self::default();
        for (i, entry) in entries.into_iter().enumerate() {
            match entry {
                Meta::NameValue(pair) => {
                    let key = pair.path.require_ident()?.clone();
                    let value = pair.value.to_token_stream();
                    if this.entries.insert(key, value).is_some() {
                        return Err(Error::new(pair.span(), "Duplicate key"));
                    }
                }
                other => {
                    if i != 0 {
                        return Err(Error::new(other.span(), "Default entry must be the first"));
                    }
                    let value = other.to_token_stream();
                    assert!(this.default.replace(value).is_none());
                }
            }
        }
        Ok(this)
    }
}

#[derive(Clone)]
pub enum MuxNamesAttr {
    Single(Ident),
    Multiple(HashMap<Ident, Ident>),
}

impl MuxNamesAttr {
    pub fn new(attr: &Attribute) -> Option<Result<Self>> {
        if attr.path().is_ident("mux_names") {
            Some(if let Meta::List(meta) = &attr.meta {
                MuxAttr::from_args(meta.tokens.clone()).and_then(|mux| {
                    if let Some(tokens) = mux.default {
                        if mux.entries.is_empty() {
                            Ok(Self::Single(parse2::<Ident>(tokens)?))
                        } else {
                            Err(Error::new(
                                tokens.span(),
                                "Expected either single name or multiple names with keys",
                            ))
                        }
                    } else {
                        Ok(Self::Multiple(
                            mux.entries
                                .into_iter()
                                .map(|(key, value)| Ok((key, parse2::<Ident>(value)?)))
                                .collect::<Result<_>>()?,
                        ))
                    }
                })
            } else {
                Err(Error::new(attr.span(), "Unsupported attribute format"))
            })
        } else {
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (Option<&Ident>, &Ident)> {
        let iter: Box<dyn Iterator<Item = (Option<&Ident>, &Ident)>> = match self {
            Self::Single(name) => Box::new([(None, name)].into_iter()),
            Self::Multiple(entries) => Box::new(entries.iter().map(|(k, v)| (Some(k), v))),
        };
        iter
    }
}
