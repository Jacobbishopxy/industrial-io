//! CRUD derive macro
//!
//! This macro generates CRUD methods for a given struct.

use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};

const ID: &str = "id";

#[proc_macro_derive(CRUD, attributes(oid))]
pub fn derive_crud(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`
    let name = input.ident;

    // let generics =

    let expanded = quote! {
        //
    };

    proc_macro::TokenStream::from(expanded)
}

fn get_field_id(data: &Data) -> Option<&Ident> {
    match data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                for field in fields.named.iter() {
                    if *field.ident.as_ref().unwrap() == ID {
                        return Some(field.ident.as_ref().unwrap());
                    }
                }
                None
            }
            _ => None,
        },
        _ => None,
    }
}
