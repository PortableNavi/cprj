extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};



#[proc_macro_derive(FromFile)]
pub fn derive(input: TokenStream) -> TokenStream
{
   let DeriveInput {ident, ..} = parse_macro_input!(input);
   
   quote! 
   {
       impl FromFile for #ident {}
   }.into()
}
