extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Similar)]
pub fn similar_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    let tokens = impl_similar(&ast);
    return tokens;
    //panic!(tokens.to_string());
}

fn impl_similar(ast: &syn::DeriveInput) -> TokenStream {
    let mut fields = vec![];
    match &ast.data {
        syn::Data::Struct(data_struct) => {
            match &data_struct.fields {
                syn::Fields::Named(fields_named) => {
                    for field in &fields_named.named {
                        let ident = field.ident.as_ref().unwrap();
                        fields.push(ident);
                    }

                },
                syn::Fields::Unnamed(_) => {
                    panic!("Not yet implemented for Unnamed fields.")
                },
                syn::Fields::Unit => {
                    panic!("Not yet implemented for Unit.")
                },
            }
        },
        syn::Data::Enum(_) => {
            panic!("Not yet implemented for Enum.")
        },
        syn::Data::Union(_) => {
            panic!("Not yet implemented for Union.")
        },
    };

    let name = &ast.ident;
    let gen = quote! {
        impl Similar for #name {
            fn is_similar(self, other : #name, eps : f64) -> bool {
                return #(self.#fields.is_similar(other.#fields, eps))&*
            }
        }
    };
    gen.into()
}