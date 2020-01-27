extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Result, Field, ItemStruct};
use syn::parse::{Parse, ParseStream};
use quote::quote;

//struct MyMacroInput {
//    f : Field,
//}
//
//impl Parse for MyMacroInput {
//    fn parse(input: ParseStream) -> Result<Self> {
//        let f = Field::parse_named(input)?;
//
//        Ok(MyMacroInput {
//            f
//        })
//    }
//}

#[proc_macro]
pub fn generate_storage(input: TokenStream) -> TokenStream {
    let i = parse_macro_input!(input as ItemStruct);
    eprint!("{:#?}", i);

    let out = quote!(
        #i
    );

    TokenStream::from(out)
}
