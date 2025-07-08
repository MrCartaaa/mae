extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::Data::Struct;
use syn::Fields::Named;
use syn::FieldsNamed;
use syn::parse_macro_input;
use syn::{Data, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(Repo)]
pub fn derive_repo(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);

    // add impl CRUD functions to RepoStruct

    let fields = match &ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };
    let idents = fields.iter().map(|f| {
        let name = &f.ident;
        quote! {#name}
    });
    let values_count: String = fields
        .iter()
        .enumerate()
        .map(|(i, _)| format!("${}", i))
        .collect();
    let field_names: Vec<String> = fields
        .into_iter()
        .map(|f| f.ident.clone().as_ref().map(|id| id.to_string()).unwrap())
        .collect();
    let field_names_string: String = field_names.into_iter().collect();
    let struct_name = &ast.ident;

    let create_fn_data_type = format_ident!("Create{}", ast.ident);
    let update_fn_data_type = format_ident!("Update{}", ast.ident);
    let select_fn_data_type = format_ident!("Select{}", ast.ident);

    //TODO: have to add Ctx to these functions -- there is a bug in the body of create() (commented
    //out). Once this is fixed, we then have to wire up sqlx <> sqlite for testing in the mae lib.

    quote! {

            impl #struct_name {
                fn create(data: #create_fn_data_type) -> Result<(), anyhow::Error> {
                Ok(())
            // let sql =
            //     format!(
            //  "INSERT INTO {table} ({field_names}) VALUES (values_count) RETURNING {field_names};",
            //  table = #struct_name,
            //  field_names = #field_names_string,
            //  values_count = #values_count);
            //
            //
            // let result = sqlx.query_as!(
            //     #struct_name, sql, #(#idents),*
            // ).fetch_one(ctx.db_pool).await?;
            //
            // Ok(result)
            }
                fn update(data: #update_fn_data_type) -> Result<(), anyhow::Error> {
                Ok(())
                }

                fn select(data: #select_fn_data_type) -> Result<(), anyhow::Error> {
    Ok(())
                }
            }
        }
    .into()
}

#[proc_macro_attribute]
pub fn repo(_: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    // UTIL
    let repo_ident = &ast.ident;

    let fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("Only works for structs"),
    };
    let params = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! {#name: #ty}
    });

    // rebuild repo struct
    let repo = quote! {
        pub struct #repo_ident {
            pub id: u64,
            pub sys_client: u64,
            pub status: DomainStatus,
            #(#params,)*
            pub comment: Option<String>,
            pub tags: Vec<String>,
            pub sys_detail: Map<String, Value>,
            pub created_by: u64,
            pub updated_by: u64,
            pub created_at: DateTime<Utc>,
            pub updated_at: DateTime<Utc>,
        }
    };

    // create DATA structs for CRUD Operations
    let params = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! {#name: #ty}
    });

    // REPO SELECT DATA
    let create_repo_ident = format_ident!("Create{}", &ast.ident);
    let create_repo = quote! {
        pub struct #create_repo_ident {
            #(#params,)*
            pub comment: Option<String>,
            pub tags: Option<Vec<String>>,
            pub sys_detail: Option<Map<String, Value>>,
        }
    };
    let opt_params = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! {#name: Option<#ty>}
    });
    // REPO UPDATE DATA
    let update_repo_ident = format_ident!("Update{}", &ast.ident);
    let update_repo = quote! {
        pub struct #update_repo_ident {
            #(#opt_params,)*
            pub comment: Option<String>,
            pub tags: Option<Vec<String>>,
            pub sys_detail: Option<Map<String, Value>>,
        }
    };
    let opt_params = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! {#name: Option<#ty>}
    });
    // REPO SELECT DATA
    let select_repo_ident = format_ident!("Select{}", &ast.ident);
    let select_repo = quote! {
        pub struct #select_repo_ident {
            #(#opt_params,)*
            pub comment: Option<String>,
            pub tags: Option<Vec<String>>,
            pub sys_detail: Option<Map<String, Value>>,
        }
    };

    quote! {
        #repo

        #create_repo
        #select_repo
        #update_repo
    }
    .into()
}
