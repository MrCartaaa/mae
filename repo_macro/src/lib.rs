extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::Data::Struct;
use syn::Fields::Named;
use syn::FieldsNamed;
use syn::parse_macro_input;
use syn::{Data, DataStruct, DeriveInput, Field, Fields};

enum Method {
    Create,
    Read,
    Update,
}

fn has_attribute(field: &Field, attr_name: &str) -> bool {
    field
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident(attr_name))
}

fn get_ident(
    field: &Field,
    method: Method,
    i: usize,
) -> (proc_macro2::TokenStream, String, String) {
    let name = &field.ident;

    let field_name = field
        .ident
        .clone()
        .as_ref()
        .map(|id| format!("{}", id.to_string()))
        .unwrap();
    match method {
        Method::Create => {
            if has_attribute(field, "id") {
                return (quote! {}, String::from(""), String::from(""));
            }
            if has_attribute(field, "gen_date") {
                return (quote! {}, String::from("now()"), field_name);
            }
            if has_attribute(field, "from_context") {
                return (quote! {ctx.session.user.id}, format!("${}", i), field_name);
            }
            if has_attribute(field, "sys_client")
                || has_attribute(field, "option")
                || has_attribute(field, "status")
            {
                return (quote! {data.#name}, format!("${}", i), field_name);
            }
            return (quote! {data.#name}, format!("${}", i), field_name);
        }
        _ => todo!(),
    }
}

#[proc_macro_derive(Repo, attributes(default, sys_client, option, from_context, gen_date))]
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

    let mut idents = vec![];
    let mut sql_reprs = vec![];
    let mut field_names = vec![];

    fields.iter().enumerate().for_each(|(i, f)| {
        let (ident, sql_repr, field_name) = get_ident(f, Method::Create, i);
        if ident.is_empty() == false {
            idents.push(ident);
        }
        if sql_repr.is_empty() == false {
            sql_reprs.push(sql_repr);
        }
        if field_name.is_empty() == false {
            field_names.push(field_name);
        }
    });

    let sql_reprs_str = sql_reprs.into_iter().collect::<Vec<_>>().join(",");
    let field_names_string: String = field_names.into_iter().collect::<Vec<_>>().join(",");
    let struct_name = &ast.ident;
    let struct_name_str = &ast.ident.to_string().to_lowercase();

    let create_fn_data_type = format_ident!("Create{}", ast.ident);
    let update_fn_data_type = format_ident!("Update{}", ast.ident);
    let select_fn_data_type = format_ident!("Select{}", ast.ident);

    //TODO: have to add Ctx to these functions -- there is a bug in the body of create() (commented
    //out). Once this is fixed, we then have to wire up sqlx <> sqlite for testing in the mae lib.

    quote! {

            impl #struct_name {
                async fn create(ctx: RequestContext<Database>, data: #create_fn_data_type) -> Result<(), anyhow::Error> {
            let sql =
                format!(
             "INSERT INTO {} ({}) VALUES ({}) RETURNING *;",
             #struct_name_str,
             #field_names_string,
             #sql_reprs_str);


            let result: #struct_name = sqlx::query_as (
                &sql)#(.bind(#idents))*
            .fetch_one(ctx.db_pool).await?;

                Ok(())
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
        quote! {pub #name: #ty}
    });

    // rebuild repo struct
    let repo = quote! {

        #[derive(Repo, sqlx::FromRow)]
        pub struct #repo_ident {
            #[id] pub id: u64,
            #[sys_client] pub sys_client: u64,
            #[status] pub status: DomainStatus,
            #(#params,)*
            #[option] pub comment: Option<String>,
            #[sqlx(json)]
            #[option] pub tags: Vec<String>,
            #[sqlx(json)]
            #[option] pub sys_detail: Map<String, Value>,
            #[from_context] pub created_by: u64,
            #[from_context] pub updated_by: u64,
            #[gen_date] pub created_at: DateTime<Utc>,
            #[gen_date] pub updated_at: DateTime<Utc>,
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
            pub sys_client: u64,
            pub status: DomainStatus,
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
