//! CRUD derive macro
//!
//! This macro generates CRUD methods for a given struct.

use std::str::FromStr;

use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, Attribute, Data, DeriveInput, Field,
    Fields, Ident, Lit, Meta, NestedMeta,
};

const TAG: &str = "crud";
const ID: &str = "id";
const INDEX: &str = "index";

const ASC: &str = "asc";
const DESC: &str = "desc";
const UNIQUE: &str = "unique";
const TEXT: &str = "text";

/// macro for CRUD derive
#[proc_macro_derive(CRUD, attributes(crud))]
pub fn derive_crud(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let stream = impl_crud(&input);

    // Debug use:
    // println!("{}", &stream);

    proc_macro::TokenStream::from(stream)
}

/// Index direction
#[derive(Debug, Clone)]
enum Dir {
    Asc,
    Desc,
}

/// Index options (MongoDB)
#[derive(Debug, Clone)]
struct IndexOptions {
    name: String,
    dir: Dir,
    unique: bool,
    text: bool,
}

impl IndexOptions {
    fn new_from_str(name: String, s: &str) -> Self {
        // parse from string
        let mut options = s.parse::<IndexOptions>().unwrap();
        options.name = name;
        options
    }
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

impl FromStr for IndexOptions {
    type Err = ();

    // ignore any string who does not match the `IndexOptions` fields' format
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut options = IndexOptions::default();
        s.split(',').for_each(|i| match i {
            ASC => options.dir = Dir::Asc,
            DESC => options.dir = Dir::Desc,
            UNIQUE => options.unique = true,
            TEXT => options.text = true,
            _ => {}
        });
        Ok(options)
    }
}

/// `crud_derive::Dir` -> `crud::Dir`
impl ToTokens for Dir {
    // since `crud_derive::Dir` is not a public API (cannot be exported in a proc-macro crate),
    // we need to convert it to a public API (defined in `crud` crate).
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(match self {
            Dir::Asc => quote! { crud::Dir::Asc },
            Dir::Desc => quote! { crud::Dir::Desc },
        })
    }
}

/// `crud_derive::IndexOptions` -> `crud::IndexOptions`
impl ToTokens for IndexOptions {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let dir = &self.dir;
        let unique = &self.unique;
        let text = &self.text;
        tokens.extend(quote! {
            crud::IndexOptions::new(#name, #dir, #unique, #text)
        })
    }
}

/// new type for `Vec<crud_derive::IndexOptions>`
/// we need it because `syn::ToTokens` cannot be implemented for `Vec<_>`
struct IndexOptionsCollection(Vec<IndexOptions>);

/// `crud_derive::IndexOptionsCollection` -> `Vec<crud::IndexOptions>`
impl ToTokens for IndexOptionsCollection {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let d = &self.0;
        tokens.extend(quote! {
            vec![#(#d),*]
        })
    }
}

type NamedFields = Punctuated<Field, Comma>;

/// turn ast into `Punctuated<Field, Comma>`, and filter out any type that is not a Rust struct
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
///
/// same as:
///
/// ```rust,ignore
/// fn get_attr_oid(named_fields: &NamedFields) -> Option<Ident> {
///     for field in named_fields.iter() {
///         for attr in field.attrs.iter() {
///             if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
///                 if meta_list.path.is_ident(TAG) {
///                     for nested_meta in meta_list.nested.iter() {
///                         if let NestedMeta::Meta(Meta::Path(path)) = nested_meta {
///                             if path.is_ident(ID) {
///                                 return Some(field.ident.as_ref().unwrap().clone());
///                             }
///                         }
///                     }
///                 }
///             }
///         }
///     }
///     None
/// }
/// ```
fn get_attr_oid(named_fields: &NamedFields) -> Option<Ident> {
    let nested_meta_find_map = |field: &Field, nested_meta: &NestedMeta| match nested_meta {
        NestedMeta::Meta(Meta::Path(path)) if path.is_ident(ID) => {
            Some(field.ident.as_ref().unwrap().clone())
        }
        _ => None,
    };

    let attrs_find_map = |field: &Field, attr: &Attribute| match attr.parse_meta() {
        Ok(Meta::List(meta_list)) if meta_list.path.is_ident(TAG) => meta_list
            .nested
            .iter()
            .find_map(|nested_meta| nested_meta_find_map(field, nested_meta)),
        _ => None,
    };

    let field_find_map = |field: &Field| {
        field
            .attrs
            .iter()
            .find_map(|attr| attrs_find_map(field, attr))
    };

    named_fields.iter().find_map(field_find_map)
}

/// find out a field whose attribute is `index`
///  
/// ```rust,ignore
/// struct TestCrud {
///     id: Option<ID>,
///     #[crud(index = "unique,asc")]
///     name: String,
///     #[crud(index = "unique,desc,text")]
///     tag: String,
/// }
/// ```
///
/// same as:
///
/// ```rust,ignore
/// fn index_format(named_fields: &NamedFields) -> Vec<IndexOptions> {
///     let mut result = Vec::<IndexOptions>::new();
///     for field in named_fields.iter() {
///         for attr in field.attrs.iter() {
///             if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
///                 if meta_list.path.is_ident(TAG) {
///                     for nm in meta_list.nested.iter() {
///                         if let NestedMeta::Meta(Meta::NameValue(mnv)) = nm {
///                             if mnv.path.is_ident(INDEX) {
///                                 if let Lit::Str(ref s) = mnv.lit {
///                                     result.push(IndexOptions::new_from_str(
///                                         field.ident.as_ref().unwrap().to_string(),
///                                         &s.value(),
///                                     ));
///                                 }
///                             }
///                         }
///                     }
///                 }
///             }
///         }
///     }
///     result
/// }
/// ```
fn index_format(named_fields: &NamedFields) -> Vec<IndexOptions> {
    let nested_meta_filter_map = |field: &Field, nested_meta: &NestedMeta| match nested_meta {
        NestedMeta::Meta(Meta::NameValue(mnv)) if mnv.path.is_ident(INDEX) => match mnv.lit {
            Lit::Str(ref s) => Some(IndexOptions::new_from_str(
                field.ident.as_ref().unwrap().to_string(),
                &s.value(),
            )),
            _ => None,
        },
        _ => None,
    };

    let attrs_fmap = |field: &Field, attr: &Attribute| match attr.parse_meta() {
        Ok(Meta::List(meta_list)) if meta_list.path.is_ident(TAG) => meta_list
            .nested
            .iter()
            .filter_map(|nested_meta| nested_meta_filter_map(field, nested_meta))
            .collect::<Vec<IndexOptions>>(),
        _ => vec![],
    };

    let field_fmap = |field: &Field| {
        field
            .attrs
            .iter()
            .flat_map(|attr| attrs_fmap(field, attr))
            .collect::<Vec<IndexOptions>>()
    };

    named_fields.iter().flat_map(field_fmap).collect()
}

/// main process of handling derive stream
fn impl_crud(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = input.ident.clone();
    let named_fields = named_fields(input);
    let index_info = index_format(&named_fields);
    let ioc = IndexOptionsCollection(index_info);

    // get ID either from field `id` or field whose attribute is `oid`
    let id = match (get_field_id(&named_fields), get_attr_oid(&named_fields)) {
        (Some(id), _) => id,
        (None, Some(oid)) => oid,
        _ => panic!("No `id` field nor `oid` attribute were found!"),
    };

    let expanded = quote! {
        // impl `BaseCRUD`
        impl BaseCRUD for #name {
            fn get_id(&self) -> ::std::option::Option<bson::oid::ObjectId> {
                self.#id
            }

            fn remove_id(&mut self) {
                self.#id = None;
            }

            fn mutate_id(&mut self, oid: bson::oid::ObjectId) -> anyhow::Result<()> {
                self.#id = Some(oid);
                Ok(())
            }

            fn show_indexes(&self) -> ::std::vec::Vec<crud::IndexOptions> {
                #ioc
            }
        }

        // impl `MongoCRUD`
        #[async_trait::async_trait]
        impl MongoCRUD<#name> for crud::MongoClient {}
    };

    expanded
}
