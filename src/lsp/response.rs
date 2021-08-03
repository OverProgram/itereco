mod error;

#[derive(Debug)]
pub struct Response<T, E> {
    id: String,
    result: Result<T, error::ResponseError<E>>
}
