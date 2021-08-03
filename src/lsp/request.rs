#[derive(Debug)]
pub struct Request<P> {
    id: String,
    method: String,
    params: Option<P>
}
