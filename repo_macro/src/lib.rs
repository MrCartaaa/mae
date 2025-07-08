extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::Data::Struct;
use syn::Fields::Named;
use syn::parse::Parser;
use syn::parse_macro_input;
use syn::{DeriveInput, Field};

#[proc_macro_attribute]
pub fn repo(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as DeriveInput);

    match &mut ast.data {
        Struct(struct_data) => match &mut struct_data.fields {
            Named(fields) => {
                fields
                    .named
                    .push(Field::parse_named.parse2(quote! {pub id: u64}).unwrap());
                fields.named.push(
                    Field::parse_named
                        .parse2(quote! {pub sys_client: u64})
                        .unwrap(),
                );
                // TODO: This should be an enum
                fields.named.push(
                    Field::parse_named
                        .parse2(quote! {pub status: DomainStatus})
                        .unwrap(),
                );
                fields.named.push(
                    Field::parse_named
                        .parse2(quote! {pub tags: Vec<String>})
                        .unwrap(),
                );
                fields.named.push(
                    Field::parse_named
                        .parse2(quote! {pub sys_detail: Map<String, Value>})
                        .unwrap(),
                );
                fields.named.push(
                    Field::parse_named
                        .parse2(quote! {pub created_by: u64})
                        .unwrap(),
                );
                fields.named.push(
                    Field::parse_named
                        .parse2(quote! {pub updated_by: u64})
                        .unwrap(),
                );
                fields.named.push(
                    Field::parse_named
                        .parse2(quote! {pub updated_at: DateTime<Utc>})
                        .unwrap(),
                );
                fields.named.push(
                    Field::parse_named
                        .parse2(quote! {pub created_at: DateTime<Utc>})
                        .unwrap(),
                );
                fields.named.push(
                    Field::parse_named
                        .parse2(quote! {pub comment: Option<String>})
                        .unwrap(),
                );
            }
            _ => {}
        },
        _ => panic!("domain has to be used with Structs."),
    };

    quote! {
    #ast
        }
    .into()
}

// #[proc_macro_derive(Repo)]
