extern crate proc_macro;
extern crate proc_macro2;
extern crate proc_macro_hack;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Token};

struct JoinAll {
    expressions: Vec<Expr>,
}

impl Parse for JoinAll {
    fn parse(input: ParseStream) -> syn::Result<JoinAll> {
        let mut expressions = Vec::new();
        let mut last_expr = TokenStream2::new();

        while !input.is_empty() {
            let next: TokenTree = input.parse()?;
            last_expr.extend(Some(next));
            if input.is_empty() {
                expressions.push(syn::parse2(last_expr)?);
                last_expr = TokenStream2::new();
            } else if input.peek(Token![,]) {
                if let Ok(parsed) = syn::parse2(last_expr.clone()) {
                    expressions.push(parsed);
                    last_expr = TokenStream2::new();
                    input.parse::<Token![,]>()?;
                }
            }
        }

        Ok(JoinAll { expressions })
    }
}

#[proc_macro_hack]
pub fn join_all(input: TokenStream) -> TokenStream {
    let JoinAll { expressions } = syn::parse_macro_input!(input as JoinAll);

    TokenStream::from(quote! { ::futures::join!(#(#expressions),*) })
}
