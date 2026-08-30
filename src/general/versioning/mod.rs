pub trait Versioning<T> {
    fn latest(&self) -> Option<&T>;
    fn oldest(&self) -> Option<&T>;
}