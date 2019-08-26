#[derive(Clone, Debug)]
pub struct Penalty<T> {
    pub r#match: T,
    pub mismatch: T,
    pub gap: T,
}
