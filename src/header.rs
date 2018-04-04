pub trait Header {
    fn decoded(&self) -> Self;
    fn next_layer<T>(&self, &T) -> T;
}
