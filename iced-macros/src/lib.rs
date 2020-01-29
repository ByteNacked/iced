extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{parse_macro_input, Result, Field, Fields, ItemStruct, Ident, FieldsNamed, ExprLit, Lit, LitInt};
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
//

#[proc_macro]
pub fn generate_storage_ty(input: TokenStream) -> TokenStream {
    let i = parse_macro_input!(input as ItemStruct);

    //eprint!("{:#?}", &i);

    let ty_name_str = i.ident.to_string();

    let ty_name = Ident::new(&ty_name_str, Span::call_site());
    let un_ty_name = Ident::new(&format!("Recast{}", &ty_name_str), Span::call_site());

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

    let setter_names : Vec<_> = (&field_name).into_iter().map(|name| {
        Ident::new(&format!("set_{}", name.to_string()), name.span())
    }).collect();

    let getter_names : Vec<_> = (&field_name).into_iter().map(|name| {
        Ident::new(&format!("get_{}", name.to_string()), name.span())
    }).collect();

    let tail_names : Vec<_> = (&field_name).into_iter().map(|name| {
        Ident::new(&format!("pos_{}", name.to_string()), name.span())
    }).collect();
    
    let uids : Vec<_> = (0 .. field_name.len()).into_iter().map(|num| {
        ExprLit {
            attrs : vec![],
            lit : Lit::Int(LitInt::new(&num.to_string() , Span::call_site())),
        }
    }).collect();

    let out = quote!(
        //use $crate::Record;
        //use ::iced::Storage;

        const VALUE_MAX_SZ : usize = 0x40;

        union #un_ty_name {
            #( #field_name : #field_ty, )*
            buf : [u8;VALUE_MAX_SZ],
        }

        pub struct #ty_name<S> {
            #( #tail_names : usize, )*
            storage : S
        }

        impl<S : Storage> #ty_name<S> {
            
            pub fn new(storage : S) -> Self {
                use core::default::Default;
                Self {
                    storage,
                    #( #tail_names : 0,)*
                }
            }

            #( 
                pub fn #getter_names(&self) ->  #field_ty {
                    let mut buf = [0;0x40];
                    self.storage.read(#uids, &mut buf);
                    #field_ty::default()
                }
            )*
            #( 
                pub fn #setter_names(&mut self, #field_name : #field_ty) {
                    let mut buf = [0;0x40];
                    self.storage.write(#uids, &buf);
                }
            )*
        }
    );

    TokenStream::from(out)
}


