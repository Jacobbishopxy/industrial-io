//! CRUD derive macro
//!
//! This macro generates CRUD methods for a given struct.

use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};

const ID: &str = "id";
const OID: &str = "oid";

#[proc_macro_derive(CRUD, attributes(oid))]
pub fn derive_crud(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let stream = impl_crud(&input);

    proc_macro::TokenStream::from(stream)
}

fn get_field_id(ast: &DeriveInput) -> Option<Ident> {
    match ast.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                for field in fields.named.iter() {
                    if *field.ident.as_ref().unwrap() == ID {
                        return Some(field.ident.as_ref().unwrap().clone());
                    }
                }
                None
            }
            _ => None,
        },
        _ => None,
    }
}

fn get_attr_oid(ast: &DeriveInput) -> Option<Ident> {
    ast.attrs.iter().find_map(|attr| {
        if attr.path.segments.len() == 1 && attr.path.segments[0].ident == OID {
            match attr.tokens.clone().into_iter().next() {
                Some(proc_macro2::TokenTree::Ident(ident)) => Some(ident),
                _ => None,
            }
        } else {
            None
        }
    })
}

fn impl_crud(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = input.ident.clone();
    let id = match (get_field_id(input), get_attr_oid(input)) {
        (Some(id), _) => id,
        (None, Some(oid)) => oid,
        _ => panic!("No ID or OID field found"),
    };

    let expanded = quote! {
        impl IDMutator for #name {
            fn mutate_id(&mut self, oid: bson::oid::ObjectId) -> TGResult<()> {
                self.#id = Some(oid);
                Ok(())
            }
        }
    };

    expanded
}
