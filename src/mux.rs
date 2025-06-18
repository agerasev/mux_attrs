use std::collections::HashSet;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Attribute, DeriveInput, Error, Ident, Meta, Result, parse2, visit_mut::VisitMut};

use crate::attrs::{MuxAttr, MuxNamesAttr};

pub fn derive(input: TokenStream) -> Result<TokenStream> {
    let derive_input: DeriveInput = parse2(input)?;

    let attrs = derive_input
        .attrs
        .iter()
        .filter_map(MuxNamesAttr::new)
        .collect::<Result<Vec<_>>>()?;
    let [names] = attrs.try_into().map_err(|_| {
        Error::new(
            Span::call_site(),
            "There must be exactly one `mux_names` attribute",
        )
    })?;

    let mut items = quote! {};
    let all_keys = match &names {
        MuxNamesAttr::Single(_) => HashSet::new(),
        MuxNamesAttr::Multiple(entries) => entries.keys().collect(),
    };
    for (key, name) in names.iter() {
        let mut demuxer = Demuxer {
            key,
            all_keys: &all_keys,
            errors: Vec::new(),
        };

        let mut new_input = derive_input.clone();

        new_input.ident = name.clone();

        demuxer.visit_derive_input_mut(&mut new_input);
        if let Some(err) = demuxer.errors.drain(..).next() {
            return Err(err);
        }

        items = quote! {
            #items
            #new_input
        };
    }

    Ok(items)
}

struct Demuxer<'a> {
    key: Option<&'a Ident>,
    all_keys: &'a HashSet<&'a Ident>,
    errors: Vec<Error>,
}

impl VisitMut for Demuxer<'_> {
    fn visit_attributes_mut(&mut self, attrs: &mut Vec<Attribute>) {
        let mut new_attrs = Vec::new();
        for attr in attrs.iter() {
            match self.demux_attr(attr) {
                Ok(Some(attr)) => new_attrs.push(attr),
                Ok(None) => (),
                Err(err) => self.errors.push(err),
            }
        }
        *attrs = new_attrs;
    }
}

impl Demuxer<'_> {
    fn demux_attr(&mut self, attr: &Attribute) -> Result<Option<Attribute>> {
        let mut mux = match MuxAttr::new(attr) {
            Some(result) => result?,
            None => {
                return Ok(
                    match attr
                        .path()
                        .get_ident()
                        .map(|ident| ident.to_string())
                        .as_deref()
                    {
                        Some("mux_names") => None,
                        _ => Some(attr.clone()),
                    },
                );
            }
        };

        for key in mux.entries.keys() {
            if !self.all_keys.contains(key) {
                return Err(Error::new(key.span(), "Name not found in `max_names`"));
            }
        }

        let mut content = None;
        if let Some(key) = self.key {
            if let Some(value) = mux.entries.remove(key) {
                content = Some(value);
            }
        }
        if content.is_none() {
            if let Some(value) = mux.default {
                content = Some(value);
            }
        }

        Ok(if let Some(content) = content {
            Some(Attribute {
                meta: parse2::<Meta>(content)?,
                ..attr.clone()
            })
        } else {
            None
        })
    }
}
