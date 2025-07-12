use mae::request_context::{RequestContext, Session, User};

use sqlx::PgPool;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};

fn connect() -> PgPool {
    PgPoolOptions::new().connect_lazy_with(
        PgConnectOptions::new()
            .host("127.0.0.1")
            .port(5432)
            .password("password")
            .username("postgres")
            .ssl_mode(PgSslMode::Prefer)
            .database("test_mae"),
    )
}

pub async fn get_context() -> Result<RequestContext, sqlx::Error> {
    let conn = connect();

    let ctx = RequestContext {
        db_pool: conn,
        session: Session {
            user: User { id: 1 },
        },
    };
    migrate(&ctx).await?;
    Ok(ctx)
}

async fn migrate(ctx: &RequestContext) -> Result<(), sqlx::Error> {
    sqlx::migrate!().run(&ctx.db_pool).await?;
    Ok(())
}
