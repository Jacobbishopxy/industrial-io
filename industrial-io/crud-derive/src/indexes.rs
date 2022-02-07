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

#[derive(Debug, Clone)]
pub struct CommonOption {
    pub dir: Dir,
    pub unique: bool,
    pub text: bool,
}

impl Default for CommonOption {
    fn default() -> Self {
        CommonOption {
            dir: Dir::Asc,
            unique: false,
            text: false,
        }
    }
}

impl FromStr for CommonOption {
    type Err = ();

    // ignore any string who does not match the `IndexOptions` fields' format
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut options = CommonOption::default();
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

/// Index options (MongoDB)
#[derive(Debug, Clone, Default)]
pub struct SingleIndex {
    pub name: String,
    pub common_option: CommonOption,
}

impl SingleIndex {
    pub fn new_from_str(name: String, s: &str) -> Self {
        // parse from string
        let common_option = s.parse::<CommonOption>().unwrap();
        SingleIndex {
            name,
            common_option,
        }
    }
}

/// new type for `Vec<crud_derive::IndexOptions>`
/// we need it because `syn::ToTokens` cannot be implemented for `Vec<_>`
#[derive(Debug, Clone)]
pub struct SingleIndexOptions(pub Vec<SingleIndex>);

/// `crud_derive::IndexOptions` -> `crud::IndexOptions`
impl ToTokens for SingleIndex {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let dir = &self.common_option.dir;
        let unique = &self.common_option.unique;
        let text = &self.common_option.text;
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

/// Compound index options
#[derive(Debug, Clone, Default)]
pub struct CompoundIndexOptions {
    pub names: Vec<String>,
    pub common_option: CommonOption,
}

impl ToTokens for CompoundIndexOptions {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let names = &self.names;
        let dir = &self.common_option.dir;
        let unique = &self.common_option.unique;
        let text = &self.common_option.text;
        tokens.extend(quote! {
            crud::IndexOptions::Compound(crud::CompoundIndexOptions {
                names: vec![#(#names.to_string()),*],
                dir: #dir,
                unique: #unique,
                text: #text,
            })
        })
    }
}

#[derive(Debug, Clone)]
pub enum IndexOptions {
    Single(SingleIndexOptions),
    Compound(CompoundIndexOptions),
    None,
}

impl ToTokens for IndexOptions {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            IndexOptions::Single(v) => v.to_tokens(tokens),
            IndexOptions::Compound(v) => v.to_tokens(tokens),
            IndexOptions::None => tokens.extend(quote! { crud::IndexOptions::None }),
        }
    }
}
