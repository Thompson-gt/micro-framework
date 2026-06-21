#[derive(Debug)]
/// enum of the responses the http server knows how to encode to send to the client
pub enum ValidResponse<T> {
    String(String),
    Vec(T),
}
