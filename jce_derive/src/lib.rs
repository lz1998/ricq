// unuseable yet

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{self, DataStruct};

#[proc_macro_derive(JcePut)]
pub fn jce_put_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let struct_name = input.ident.to_string();
    let struct_data = input.data;
    if let syn::Data::Struct(data_struct) = struct_data {
        gen_put_body(data_struct, struct_name)
    } else {
        panic!()
    }
}

fn gen_put_body(data: DataStruct, struct_name: String) -> TokenStream {
    let mut ts = TokenStream2::default();
    for f in data.fields {
        let item_name = f.ident.unwrap().to_string();
        // let item_tag = f.attrs[0].tokens.clone();
        ts = quote! {
            #ts
            self.#item_name.put(jce_mut);
        };
    }
    ts = quote! {
        impl JcePut for #struct_name {
            fn put(self, jce_mut: &mut JceMut, tag: u8) -> &mut JceMut {
                jce_mut.put_head(10, tag);
                #ts
                jce_mut.put_head(11, 0);
            }
        }
    };
    ts.into()
}
