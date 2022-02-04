//! CRUD derive macro
//!
//! This macro generates CRUD methods for a given struct.

use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, Data, DeriveInput, Field, Fields,
    Ident, Meta, NestedMeta,
};

const ID: &str = "id";
const OID: &str = "oid";
const INDEX: &str = "index";
const ASC: &str = "asc";
const DESC: &str = "desc";
const UNIQUE: &str = "unique";
const TEXT: &str = "text";
const INDEX_OPTIONS: &[&str] = &[ASC, DESC, UNIQUE, TEXT];

#[proc_macro_derive(CRUD, attributes(index, oid))]
pub fn derive_crud(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let stream = impl_crud(&input);

    // Debug use:
    // println!("{}", &stream);

    proc_macro::TokenStream::from(stream)
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

/// find out whether a field name is `id`
fn get_field_id(named_fields: &NamedFields) -> Option<Ident> {
    for field in named_fields.iter() {
        if field.ident.as_ref().unwrap() == ID {
            return Some(field.ident.as_ref().unwrap().clone());
        }
    }
    None
}

/// find out a field whose attribute is `oid`
fn get_attr_oid(named_fields: &NamedFields) -> Option<Ident> {
    for field in named_fields.iter() {
        for attr in field.attrs.iter() {
            if let Ok(Meta::Path(path)) = attr.parse_meta() {
                if path.segments.len() == 1 && path.is_ident(OID) {
                    return Some(field.ident.as_ref().unwrap().clone());
                }
            }
        }
    }
    None
}

enum Dir {
    Asc,
    Desc,
}

struct IndexOption {
    name: String,
    dir: Dir,
    unique: bool,
    text: bool,
}

// TODO: read from fields' attributes and form an `IndexOption`
fn index_format(named_fields: &NamedFields) {
    for field in named_fields.iter() {
        // let ident = field.ident.as_ref().unwrap();
        // let ty = &field.ty;
        for attr in field.attrs.iter() {
            if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                if meta_list.path.segments.len() == 1 && meta_list.path.is_ident(INDEX) {
                    let ml = meta_list
                        .nested
                        .iter()
                        .filter_map(|nm| {
                            if let NestedMeta::Meta(Meta::Path(path)) = nm {
                                if path.segments.len() == 1 {
                                    let ident = path.get_ident().unwrap().to_string();
                                    if INDEX_OPTIONS.contains(&ident.as_str()) {
                                        return Some(ident);
                                    }
                                }
                            }
                            None
                        })
                        .collect::<Vec<_>>();
                    println!(
                        "ml: {:?} -> {:?}",
                        field.ident.as_ref().unwrap().to_string(),
                        ml
                    );
                }
            }
        }
    }
}

fn impl_crud(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = input.ident.clone();
    let named_fields = named_fields(input);
    index_format(&named_fields);

    // get ID either from field `id` or field whose attribute is `oid`
    let id = match (get_field_id(&named_fields), get_attr_oid(&named_fields)) {
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
