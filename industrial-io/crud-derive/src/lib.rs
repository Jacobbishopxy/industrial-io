//! CRUD derive macro
//!
//! This macro generates CRUD methods for a given struct.

use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, Data, DeriveInput, Field, Fields,
    Ident, Lit, Meta, NestedMeta,
};

const TAG: &str = "crud";
const ID: &str = "id";
const INDEX: &str = "index";

enum Dir {
    Asc,
    Desc,
}

struct IndexOptions {
    name: String,
    dir: Dir,
    unique: bool,
    text: bool,
}

impl Default for IndexOptions {
    fn default() -> Self {
        IndexOptions {
            name: String::new(),
            dir: Dir::Asc,
            unique: false,
            text: false,
        }
    }
}

impl IndexOptions {
    fn new(name: &str) -> Self {
        IndexOptions {
            name: name.to_string(),
            ..Default::default()
        }
    }

    // TODO:
}

#[proc_macro_derive(CRUD, attributes(crud))]
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

/// find out a field whose attribute is `id`
///
/// ```rust,ignore
/// struct TestCrud {
///     #[crud(id)]
///     idx: Option<ID>,
/// }
/// ```
fn get_attr_oid(named_fields: &NamedFields) -> Option<Ident> {
    for field in named_fields.iter() {
        for attr in field.attrs.iter() {
            if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                if meta_list.path.is_ident(TAG) {
                    for nested_meta in meta_list.nested.iter() {
                        if let NestedMeta::Meta(Meta::Path(path)) = nested_meta {
                            if path.is_ident(ID) {
                                return Some(field.ident.as_ref().unwrap().clone());
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

// TODO: read from fields' attributes and form an `IndexOption`

/// find out a field whose attribute is `index`
///  
/// ```rust,ignore
/// struct TestCrud {
///     id: Option<ID>,
///     #[crud(index = "unique,asc,text")]
///     name: String,
/// }
/// ```
fn index_format(named_fields: &NamedFields) -> Vec<IndexOptions> {
    let mut result = Vec::<IndexOptions>::new();
    for field in named_fields.iter() {
        for attr in field.attrs.iter() {
            if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                if meta_list.path.is_ident(TAG) {
                    for nm in meta_list.nested.iter() {
                        if let NestedMeta::Meta(Meta::NameValue(mnv)) = nm {
                            if mnv.path.is_ident(INDEX) {
                                if let Lit::Str(ref s) = mnv.lit {
                                    // TODO: push to `io`
                                    println!(
                                        ">>> {:?} {:?}",
                                        field.ident.as_ref().unwrap().to_string(),
                                        s.value()
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    result
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
