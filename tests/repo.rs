use mae::repo::prelude::*;
use mae::request_context::{RequestContext, Session, User};

use sqlx::Connection;
use sqlx::sqlite::SqliteConnection;

async fn connect() -> Result<SqliteConnection, sqlx::Error> {
    let conn = SqliteConnection::connect("sqlite::memory:").await?;
    Ok(conn)
    // let options = SqliteConnectOptions::new()
    //     .filename(filename)
    //     .create_if_missing(true);
    // SqlitePoolOptions::new()
    //     .max_connections(1)
    //     .connect_with(options)
    //     .await
}

async fn get_context() -> Result<RequestContext<SqliteConnection>, sqlx::Error> {
    let conn = connect().await?;

    Ok(RequestContext::<SqliteConnection> {
        db_pool: conn,
        session: Session {
            user: User { id: 1 },
        },
    })
}

#[repo]
pub struct RepoExample {
    pub value: u64,
    pub string_value: String,
}

#[test]
fn should_make_domain_struct() {
    let _my_repo = RepoExample {
        value: 1,
        string_value: String::from("hello_world"),
        comment: None,
        id: 1,
        sys_client: 1,
        status: DomainStatus::Active,
        tags: vec![],
        sys_detail: Map::new(),
        created_by: 1,
        updated_by: 1,
        updated_at: Utc::now(),
        created_at: Utc::now(),
    };
    assert!(true);
}

#[tokio::test]
async fn should_create_record() {
    let ctx = get_context().await.unwrap();

    let data = CreateRepoExample {
        sys_client: 1,
        status: DomainStatus::Active,
        value: 1,
        string_value: String::from("hello_world"),
        comment: None,
        tags: None,
        sys_detail: None,
    };
    let rec = RepoExample::create(ctx, data).await;

    assert!(rec.is_ok());
}
