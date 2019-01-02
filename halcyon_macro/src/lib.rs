extern crate proc_macro;

use proc_macro::{Literal, TokenStream, TokenTree};
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::{parse_macro_input, Expr};

/// Add one to an expression.
#[proc_macro_hack]
pub fn html(input: TokenStream) -> TokenStream {
    println!("{}", input.to_string());
    TokenTree::Literal(Literal::u32_suffixed(42)).into()
}
