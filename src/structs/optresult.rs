/// Optional Result
pub enum OptResult<T, E> {
    /// A value was computed.
    Some(T),
    /// No value was computed, but no error occoured.
    None,
    /// No value was computed, due to a error.
    Err(E),
}
