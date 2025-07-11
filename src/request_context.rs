pub struct RequestContext<T> {
    db_pool: T,
    session: Session,
}

pub struct Session {
    user: User,
}

pub struct User {
    id: i64,
}
