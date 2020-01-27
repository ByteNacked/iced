extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{parse_macro_input, Result, Field, Fields, ItemStruct, Ident, FieldsNamed};
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
pub fn generate_storage_ty(input: TokenStream) -> TokenStream {
    let i = parse_macro_input!(input as ItemStruct);

    //eprint!("{:#?}", &i);

    let ty_name_str = i.ident.to_string();
    let ty_name = Ident::new(&ty_name_str, Span::call_site());

    let fields = if let ItemStruct { fields : Fields::Named( FieldsNamed{ named, .. } ), .. } = &i {
        named
    } else {
        unimplemented!()
    };
    //eprint!("fields : {:#?}", &fields);

    let field_name : Vec<&_> = fields.into_iter().filter_map(|f| {
        f.ident.as_ref()
    }).collect();

    let field_ty : Vec<&_> = fields.into_iter().map(|f| {
        &f.ty
    }).collect();

    let out = quote!(
        use $crate::Record;
        use $crate::Storage;

        struct #ty_name<A> {
            #( #field_name : #field_ty),*
        }

        impl<A : Storage> #ty_name<A> {
            
        }
    );

    TokenStream::from(out)
}
