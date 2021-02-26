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

    let collection_name = name.to_string().to_lowercase();

    let gen = quote! {
      impl Entity for #name {
        type ToEntity = #name;
        fn collection() -> &'static str {
          #collection_name
        }
        fn convert_to_doc(&self) -> Document {
          doc! {
            "_id": "hola"
          }
        }
        fn convert_to_entity(document: Document) -> Self::ToEntity {
          User {
            _id: document.get("_id").unwrap().to_string(),
            firstname: document.get("firstname").unwrap().to_string(),
            lastname: document.get("lastname").unwrap().to_string(),
            email: document.get("email").unwrap().to_string(),
            password: document.get("password").unwrap().to_string(),
          }
        }
      }
    };

    gen.into()
}
