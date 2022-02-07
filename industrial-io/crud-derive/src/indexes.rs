//！ Indexes
//！
//！ Single

use std::str::FromStr;

use quote::{quote, ToTokens};

pub const ASC: &str = "asc";
pub const DESC: &str = "desc";
pub const UNIQUE: &str = "unique";
pub const TEXT: &str = "text";

/// Index direction
#[derive(Debug, Clone)]
pub enum Dir {
    Asc,
    Desc,
}

/// Index options (MongoDB)
#[derive(Debug, Clone)]
pub struct SingleIndex {
    name: String,
    dir: Dir,
    unique: bool,
    text: bool,
}

impl SingleIndex {
    pub fn new_from_str(name: String, s: &str) -> Self {
        // parse from string
        let mut options = s.parse::<SingleIndex>().unwrap();
        options.name = name;
        options
    }
}

/// new type for `Vec<crud_derive::IndexOptions>`
/// we need it because `syn::ToTokens` cannot be implemented for `Vec<_>`
#[derive(Debug, Clone)]
pub struct SingleIndexOptions(pub Vec<SingleIndex>);

impl Default for SingleIndex {
    fn default() -> Self {
        SingleIndex {
            name: String::new(),
            dir: Dir::Asc,
            unique: false,
            text: false,
        }
    }
}

impl FromStr for SingleIndex {
    type Err = ();

    // ignore any string who does not match the `IndexOptions` fields' format
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut options = SingleIndex::default();
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
impl ToTokens for SingleIndex {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let dir = &self.dir;
        let unique = &self.unique;
        let text = &self.text;
        tokens.extend(quote! {
            crud::SingleIndex::new(#name, #dir, #unique, #text)
        })
    }
}

/// `crud_derive::IndexOptionsCollection` -> `Vec<crud::IndexOptions>`
impl ToTokens for SingleIndexOptions {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let d = &self.0;
        tokens.extend(quote! {
            crud::IndexOptions::Single(crud::SingleIndexOptions(vec![#(#d),*]))
        })
    }
}

#[derive(Debug, Clone)]
pub struct CompoundIndexOptions {
    pub names: Vec<String>,
    pub dir: Dir,
    pub unique: bool,
    pub text: bool,
}

impl CompoundIndexOptions {
    pub fn new(names: &[&str], dir: Dir, unique: bool, text: bool) -> Self {
        CompoundIndexOptions {
            names: names.iter().map(|v| v.to_string()).collect(),
            dir,
            unique,
            text,
        }
    }
}
