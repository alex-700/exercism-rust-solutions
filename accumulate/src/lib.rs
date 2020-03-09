pub fn map<T, R>(input: Vec<T>, f: impl FnMut(T) -> R) -> Vec<R> {
    input.into_iter().map(f).collect()
}
