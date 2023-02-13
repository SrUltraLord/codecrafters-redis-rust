pub trait Marshaller<T> {
    fn init() -> Self;
    fn marshall(&self, value: T) -> String;
}
