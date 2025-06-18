mod attrs;
mod from_;
mod mux;

use proc_macro::TokenStream;
use quote::ToTokens;

#[proc_macro_derive(From, attributes(from))]
pub fn derive_from(input: TokenStream) -> TokenStream {
    match from_::derive(input.into()) {
        Ok(expr) => expr.into_token_stream(),
        Err(err) => err.into_compile_error(),
    }
    .into()
}

#[proc_macro_derive(Mux, attributes(mux, mux_names))]
pub fn derive_mux(input: TokenStream) -> TokenStream {
    match mux::derive(input.into()) {
        Ok(expr) => expr.into_token_stream(),
        Err(err) => err.into_compile_error(),
    }
    .into()
}
