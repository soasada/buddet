extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Entity)]
pub fn entity_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_entity(&ast)
}

fn impl_entity(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = &ast.data;

    let struct_data = match data {
        syn::Data::Struct(d) => d,
        _ => panic!("This derive can only be used on structs"),
    };

    let gen = quote! {
      use mongodb::{bson::{Document, doc}};
      impl Entity for #name {
        fn collection() -> &'static str {
          stringify!(#name).to_lowercase().as_str()
        }
        fn convert_to_doc(&self) -> Document {
          return
        }
        fn convert_to_entity(document: Document) -> Self::ToEntity {

        }
      }
    };

    gen.into()
}
