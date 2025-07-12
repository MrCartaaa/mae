extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::Data::Struct;
use syn::Fields::Named;
use syn::FieldsNamed;
use syn::parse_macro_input;
use syn::{Data, DataStruct, DeriveInput, Field, Fields};

// SQL Method
enum Method {
    Insert,
    Select,
    Update,
}

// check if a field has a specific attribute
fn has_attribute(field: &Field, attr_name: &str) -> bool {
    field
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident(attr_name))
}

// get the TokenStream, SQL syntactical represnetation, sql field_name, and the returning sql
// field_name.
// if these return as null / empty, then they will not be populated in their respective section.
fn get_sql_parts(
    field: &Field,
    method: Method,
    i: usize,
) -> (proc_macro2::TokenStream, String, String, String) {
    let ident = &field.ident;

    let field_name = field
        .ident
        .clone()
        .as_ref()
        .map(|id| format!("{}", id.to_string()))
        .unwrap();
    match method {
        Method::Insert => {
            if has_attribute(field, "id") {
                return (quote! {}, String::from(""), String::from(""), field_name);
            }
            if has_attribute(field, "gen_date") {
                return (
                    quote! {},
                    String::from("now()"),
                    field_name.clone(),
                    field_name.clone(),
                );
            }
            if has_attribute(field, "from_context") {
                return (
                    quote! {ctx.session.user.id},
                    format!("${}", i),
                    field_name.clone(),
                    field_name.clone(),
                );
            }
            return (
                quote! {data.#ident},
                format!("${}", i),
                field_name.clone(),
                field_name.clone(),
            );
        }
        Method::Update => todo!(),
        Method::Select => todo!(),
    }
}

// Macro to impl Repo:
// Methods:
//  Insert(ctx, Insert[repo_name]) -> Result<impl Repo, sqlx::Error>;
//  Select(ctx, Select[repo_name]) -> Result<impl Repo, sqlx::Error>;
//  Update(ctx, Update[repo_name]) -> Result<impl Repo, sqlx::Error>;
#[proc_macro_derive(Repo, attributes(id, from_context, gen_date,))]
pub fn derive_repo(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);

    // Making sure it the derive macro is called on a sctruct;
    let fields = match &ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    // get the sql parts

    let mut idents = vec![];
    let mut sql_reprs = vec![];
    let mut field_names = vec![];
    let mut returning = vec![];

    fields.iter().enumerate().for_each(|(i, f)| {
        let (ident, sql_repr, field_name, returning_field) = get_sql_parts(f, Method::Insert, i);
        if ident.is_empty() == false {
            idents.push(ident);
        }
        if sql_repr.is_empty() == false {
            sql_reprs.push(sql_repr);
        }
        if field_name.is_empty() == false {
            field_names.push(field_name);
        }
        if returning_field.is_empty() == false {
            returning.push(returning_field);
        }
    });

    // convert sql parts into strings

    let sql_reprs_str = sql_reprs.into_iter().collect::<Vec<_>>().join(", ");
    let field_names_string: String = field_names.into_iter().collect::<Vec<_>>().join(", ");
    let returning_string: String = returning.into_iter().collect::<Vec<_>>().join(", ");

    // get the struct details
    let struct_name = &ast.ident;
    let struct_name_str = &ast.ident.to_string().to_lowercase();

    // create the SQL Method params

    let create_fn_data_type = format_ident!("Insert{}", ast.ident);
    let update_fn_data_type = format_ident!("Update{}", ast.ident);
    let select_fn_data_type = format_ident!("Select{}", ast.ident);

    quote! {

            impl #struct_name {
                pub async fn insert(ctx: &RequestContext, data: #create_fn_data_type) -> Result<#struct_name, anyhow::Error> {

                    let sql = format!(
                    "INSERT INTO {} ({}) VALUES ({}) RETURNING {};",
                    #struct_name_str,
                    #field_names_string,
                    #sql_reprs_str,
                    #returning_string);

                    let result: #struct_name = sqlx::query_as (
                        &sql)#(.bind(#idents))*
                    .fetch_one(&ctx.db_pool).await?;

                    Ok(result)
                }

                fn update(data: #update_fn_data_type) -> Result<#struct_name, anyhow::Error> {
                todo!()
                }

                fn select(data: #select_fn_data_type) -> Result<#struct_name, anyhow::Error> {
                todo!()
                }
            }
        }
    .into()
}

// procedural macro to populate require structs for working with a PgRepo
#[proc_macro_attribute]
pub fn repo(_: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let repo_ident = &ast.ident;

    // confirm the macro is being called on a Struct Type and extract the fields.
    let fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("Only works for structs"),
    };

    // rebuild the struct fields
    let params = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! {pub #name: #ty}
    });

    // rebuild repo struct with the existing fields and default fields for the repo
    // NOTE: here, we are deriving the Repo with the proc_macro_derive fn from above
    let repo = quote! {

        #[derive(Repo, sqlx::FromRow, Serialize, Deserialize)]
        pub struct #repo_ident {
            #[id] pub id: i32,
            pub sys_client: i32,
            pub status: DomainStatus,
            #(#params,)*
            pub comment: Option<String>,
            #[sqlx(json)]
            pub tags: Value,
            #[sqlx(json)]
            pub sys_detail: Value,
            #[from_context] pub created_by: i32,
            #[from_context] pub updated_by: i32,
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
    let create_repo_ident = format_ident!("Insert{}", &ast.ident);
    let create_repo = quote! {
        pub struct #create_repo_ident {
            #(pub #params,)*
            pub sys_client: i32,
            pub status: DomainStatus,
            pub comment: Option<String>,
            pub tags: Value,
            pub sys_detail: Value,
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
            #(pub #opt_params,)*
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
    // REPO SELECT DATA
    let select_repo_ident = format_ident!("Select{}", &ast.ident);
    let select_repo = quote! {
        pub struct #select_repo_ident {
            #(pub #opt_params,)*
            pub comment: Option<String>,
            pub tags: Option<Vec<String>>,
            pub sys_detail: Option<Map<String, Value>>,
        }
    };

    // Return the existing Repo with default fields and the structs that support SQL Methods

    quote! {
        #repo

        #create_repo
        #select_repo
        #update_repo
    }
    .into()
}
