use sqlx::PgPool;

pub struct RequestContext {
    pub db_pool: PgPool,
    pub session: Session,
}

pub struct Session {
    pub user: User,
}

pub struct User {
    pub id: i64,
}
