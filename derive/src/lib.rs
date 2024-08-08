extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(AbstractState)]
pub fn derive_abstract_state(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let gen_tokens = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let matches_impl = fields.named.iter().map(|f| {
                    let field_name = &f.ident;
                    quote! {
                        if !self.#field_name.matches(&other.#field_name) {
                            return false;
                        }
                    }
                });
                let update_impl = fields.named.iter().map(|f| {
                    let field_name = &f.ident;
                    quote! {
                        self.#field_name.update(&other.#field_name);
                    }
                });
                quote! {
                    impl AbstractState for #name {
                        fn matches(&self, other: &Self) -> bool {
                            #( #matches_impl )*
                            true
                        }
                        fn update(&mut self, other: &Self) {
                            #( #update_impl )*
                        }
                    }
                }
            }
            Fields::Unnamed(fields) => {
                let matches_impl = fields.unnamed.iter().enumerate().map(|(i, _)| {
                    let index = syn::Index::from(i);
                    quote! {
                        if !self.#index.matches(&other.#index) {
                            return false;
                        }
                    }
                });
                let update_impl = fields.unnamed.iter().enumerate().map(|(i, _)| {
                    let index = syn::Index::from(i);
                    quote! {
                        self.#index.update(&other.#index);
                    }
                });
                quote! {
                    impl AbstractState for #name {
                        fn matches(&self, other: &Self) -> bool {
                            #( #matches_impl )*
                            true
                        }
                        fn update(&mut self, other: &Self) {
                            #( #update_impl )*
                        }
                    }
                }
            }
            Fields::Unit => {
                quote! {
                    impl AbstractState for #name {
                        fn matches(&self, other: &Self) -> bool {
                            true
                        }
                        fn update(&mut self, _other: &Self) {}
                    }
                }
            }
        },
        _ => unimplemented!(),
    };

    gen_tokens.into()
}
