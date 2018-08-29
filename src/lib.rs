extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro]
pub fn test(input: TokenStream) -> TokenStream {
    let chars = syn::parse::<syn::LitStr>(input).unwrap();
    let mut table: [bool; 256] = [false; 256];
    for c in chars.value().chars() {
        table[c as usize] = true;
    }
    let expanded = quote! {
        [#(#table),*]
    };
    expanded.into()
}
