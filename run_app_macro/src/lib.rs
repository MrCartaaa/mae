extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn run_app(_: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);

    let fn_block = &input_fn.block.stmts[0];

    quote! {
    async fn run<Context: Clone + Send + 'static>(
        listener: TcpListener,
        db_pool: PgPool,
        base_url: String,
        hmac_secret: SecretString,
        redis_uri: SecretString,
        custom_context: Context,
    ) -> Result<Server, anyhow::Error> {

         let redis_store = app::redis_session(redis_uri).await?;
         let server = HttpServer::new(move || {
             ActixWebApp::new()
                 .wrap(TracingLogger::default())
                 .wrap(app::session_middleware(
                     hmac_secret.clone(),
                     redis_store.clone(),
                 ))
                 .app_data(web::Data::new(ApplicationBaseUrl(base_url.clone())))
                 .app_data(web::Data::new(HmacSecret(hmac_secret.clone())))
                 .app_data(web::Data::new(db_pool.clone()))
                 .app_data(web::Data::new(custom_context.clone()))
             .#fn_block
         })
         .listen(listener)?
         .run();
         Ok(server)
         }
         }
    .into()
}
