use crate::build::get_context;
use mae::repo::prelude::*;

#[repo]
pub struct RepoExample {
    pub value: i32,
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
        tags: sqlx::types::JsonValue::Array(vec![]),
        sys_detail: sqlx::types::JsonValue::Object(Map::new()),
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

    let data = InsertRepoExample {
        sys_client: 1,
        status: DomainStatus::Active,
        value: 1,
        string_value: String::from("hello_world"),
        comment: None,
        tags: sqlx::types::JsonValue::Array(vec![]),
        sys_detail: sqlx::types::JsonValue::Object(Map::new()),
    };
    let rec = RepoExample::insert(&ctx, data).await;
    assert!(rec.is_ok());

    assert_eq!(rec.unwrap().string_value, "hello_world");
}
