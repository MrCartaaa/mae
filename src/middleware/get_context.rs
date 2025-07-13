use crate::error_response::e500;
use crate::request_context::RequestContext;
use crate::session::{Session, TypedSession};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::InternalError;
use actix_web::middleware::Next;
use actix_web::{FromRequest, HttpMessage, HttpResponse, web};
use sqlx::PgPool;
use std::sync::Arc;

// WARNING: This function currently doesn't work... although it compiles.
// route 500's with a 'missing expected request extension data' message
pub async fn get_context<T: 'static>(
    mut req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let session = {
        let (http_req, payload) = req.parts_mut();
        TypedSession::from_request(http_req, payload).await
    }?;

    match session.get_session().map_err(e500)? {
        Some(session_data) => {
            let db_pool_arc = &*Arc::clone(
                &req.app_data::<web::Data<PgPool>>()
                    .unwrap()
                    .clone()
                    .into_inner()
                    .clone(),
            );
            let custom = req
                .app_data::<web::Data<T>>()
                .unwrap()
                .clone()
                .into_inner()
                .clone();
            let session = Session::from(session_data);
            req.extensions_mut().insert(RequestContext {
                db_pool: db_pool_arc.to_owned(),
                custom,
                session,
            });
            next.call(req).await
        }
        None => {
            let resp = HttpResponse::Unauthorized().finish();
            let e = anyhow::anyhow!("Unauthorized.");
            Err(InternalError::from_response(e, resp).into())
        }
    }
}
