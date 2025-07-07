mod test_domain;

#[macro_use]
extern crate domain;

fn main() {}

#[cfg(test)]
mod tests {
    use crate::test_domain::*;
    use chrono::Utc;
    use serde_json::Map;

    #[test]
    fn should_make_domain_struct() {
        let _domain = Domain {
            value: 1,
            comment: None,
            id: 1,
            sys_client: 1,
            status: String::from("active"),
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
}
