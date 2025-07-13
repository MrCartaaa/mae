// NOTE: commented out sections are for a WIP related to the get_context middleware
use mae::request_context::RequestContext;
use mae::session::Session; //, UserData, UserRoles};

use sqlx::PgPool;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};

fn connect() -> PgPool {
    PgPoolOptions::new().connect_lazy_with(
        PgConnectOptions::new()
            .host("127.0.0.1")
            .port(2345)
            .password("password")
            .username("postgres")
            .ssl_mode(PgSslMode::Prefer)
            .database("test_mae"),
    )
}

pub async fn get_context<T: Clone>(custom_context: T) -> Result<RequestContext<T>, sqlx::Error> {
    let conn = connect();

    let ctx = RequestContext::<T> {
        db_pool: conn,
        session: Session {
            user_id: 1,
            // session_id: String::from("25397b4c-5e59-47e7-a271-24c70242cfeb"),
            // user_data: UserData {
            //     id: 1,
            //     status: String::from("active"),
            //     roles: UserRoles {
            //         role: String::from("sys_client"),
            //         has_limits: false,
            //         limit_details: json! ({"sys_client": []}),
            //         tags: vec![],
            //     },
            // },
        },
        custom: custom_context,
    };
    Ok(ctx)
}
