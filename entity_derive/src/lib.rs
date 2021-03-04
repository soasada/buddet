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
    let attributes = struct_data.fields.iter()
        .filter(|f| f.ident.is_some())
        .map(|f| f.ident.as_ref().unwrap())
        .collect::<Vec<_>>();

    let vec = attributes.iter()
        .map(|f| quote!(stringify!(#f): self.#f.clone()))
        .collect::<Vec<_>>();

    let gen = quote! {
      impl Entity for #name {
        type ToEntity = #name;
        fn collection() -> &'static str {
          #collection_name
        }
        fn convert_to_doc(&self) -> Document {
          doc! {
            #(#vec),*
          }
        }
      }
    };

    gen.into()
}
