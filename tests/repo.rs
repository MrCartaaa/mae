use anyhow::anyhow;
use mae::repo::prelude::*;

#[derive(Repo)]
#[repo]
pub struct RepoExample {
    pub value: u64,
}

#[test]
fn should_make_domain_struct() {
    let _my_repo = RepoExample {
        value: 1,
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

#[test]
fn should_create_record() {
    let data = CreateRepoExample {
        value: 1,
        comment: None,
        tags: None,
        sys_detail: None,
    };
    let rec = RepoExample::create(data);

    assert!(rec.is_ok());
}
