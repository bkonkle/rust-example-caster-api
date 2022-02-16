/// A convenience wrapper for ordering
#[allow(missing_docs)]
pub enum Ordering<T> {
    Asc(T),
    Desc(T),
}
