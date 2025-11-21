pub trait DatabaseKey: Eq + Ord {}

impl DatabaseKey for String {}
impl DatabaseKey for i64 {}