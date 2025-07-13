use crate::session::Session;
use actix_web::web::{Data, ReqData};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct RequestContext<T: Clone> {
    pub db_pool: PgPool,
    pub session: Session,
    pub custom: T,
}

impl<T: Clone> RequestContext<T> {
    pub fn new(db_pool_arc: Data<PgPool>, session: ReqData<Session>, custom_arc: Data<T>) -> Self {
        let db_pool = &*Arc::clone(&db_pool_arc.into_inner());
        let custom = &*Arc::clone(&custom_arc.into_inner());
        RequestContext {
            session: session.into_inner(),
            custom: custom.to_owned(),
            db_pool: db_pool.to_owned(),
        }
    }
}
