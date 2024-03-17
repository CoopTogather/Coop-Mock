// Validator trait
pub trait Validator<T> {
    fn is_valid(value: T) -> bool;
}
