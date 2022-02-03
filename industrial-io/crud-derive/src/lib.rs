//! CRUD derive macro
//!
//! This macro generates CRUD methods for a given struct.

use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, Attribute, Data, DeriveInput, Field,
    Fields, Ident, Lit, Meta,
};

const ID: &str = "id";
const OID: &str = "oid";
const INDEX: &str = "index";

#[proc_macro_derive(CRUD, attributes(crud, oid))]
pub fn derive_crud(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let stream = impl_crud(&input);

    // Debug use:
    // println!("{}", &stream);

    proc_macro::TokenStream::from(stream)
}

/// find out the field whose name is `id`
fn get_field_id(ast: &DeriveInput) -> Option<Ident> {
    match ast.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                for field in fields.named.iter() {
                    if field.ident.as_ref().unwrap() == ID {
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

/// find out the field whose attribute is `oid`
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

// TODO: `index(asc/desc,unique,text)`
/// find out the field whose attribute is `index`
fn get_attr_index(ast: &DeriveInput) -> Option<Ident> {
    match ast.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                for field in fields.named.iter() {
                    let field_name = field.ident.as_ref().unwrap().to_string();

                    todo!()
                }
                None
            }
            _ => None,
        },
        _ => None,
    }
}

fn impl_crud(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = input.ident.clone();
    println!("{:?}", name.to_string());
    let named_fields = named_fields(input);
    field_format(&named_fields);

    let id = match (get_field_id(input), get_attr_oid(input)) {
        (Some(id), _) => id,
        (None, Some(oid)) => oid,
        _ => panic!("No `id` field nor `oid` attribute were found!"),
    };

    let expanded = quote! {
        impl IDMutator for #name {
            fn id(&self) -> Option<bson::oid::ObjectId> {
                self.#id
            }

            fn remove_id(&mut self) {
                self.#id = None;
            }

            fn mutate_id(&mut self, oid: bson::oid::ObjectId) -> anyhow::Result<()> {
                self.#id = Some(oid);
                Ok(())
            }
        }

        impl From<&#name> for bson::Document {
            fn from(v: &#name) -> Self {
                bson::to_document(v).unwrap()
            }
        }

        #[async_trait::async_trait]
        impl MongoCRUD<#name> for crud::MongoClient {}
    };

    expanded
}

fn attr_error<T: quote::ToTokens>(tokens: T) -> syn::Error {
    syn::Error::new_spanned(tokens, "expected `debug(bound = \"...\")`")
}

type NamedFields = Punctuated<Field, Comma>;

fn named_fields(ast: &DeriveInput) -> NamedFields {
    match &ast.data {
        Data::Struct(s) => {
            if let Fields::Named(ref named_fields) = s.fields {
                named_fields.named.clone()
            } else {
                unimplemented!("derive(Builder) only supports named fields")
            }
        }
        other => unimplemented!(
            "CRUD only supports Struct and is not implemented for {:?}",
            other
        ),
    }
}

fn field_format(named_fields: &NamedFields) {
    for field in named_fields.iter() {
        // let ident = field.ident.as_ref().unwrap();
        // let ty = &field.ty;
        for attr in field.attrs.iter() {
            match attr.parse_meta() {
                Ok(Meta::List(meta_list)) => {
                    let ml = meta_list.nested.iter().collect::<Vec<_>>();
                    println!(
                        "ml: {:?} -> {:?}",
                        field.ident.as_ref().unwrap().to_string(),
                        ml
                    );
                }
                Ok(Meta::Path(path)) => {
                    println!(
                        "path: {:?} -> {:?}",
                        field.ident.as_ref().unwrap().to_string(),
                        path
                    );
                }
                _ => {}
            }
        }
    }
}
