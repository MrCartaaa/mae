use mae::repo::prelude::*;

#[repo]
#[allow(dead_code)]
pub struct RepoExample {
    pub value: u64,
}

#[test]
fn should_make_domain_struct() {
    let _domain = RepoExample {
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
    // TODO: derive_macro needs to be created, then Domain::new() and that should be used to
    // return a result -> assert!(domain.is_ok());
    assert!(true);
}
