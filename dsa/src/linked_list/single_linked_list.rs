struct SSLNode<T> {
    value: T,
    next: Option<Box<SSLNode<T>>>,
}
