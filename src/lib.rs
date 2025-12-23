// Spoon async runtime

/// A future that is immediately ready with a value.
struct Ready<T> {
    value: Option<T>,
}
