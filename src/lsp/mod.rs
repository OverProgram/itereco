use std::fmt::{Display, Formatter};

mod header;
mod response;
mod request;

const JSON_RPC_STRING: &'static str = "2.0";

#[derive(Debug)]
pub struct Packet {
    header: header::Header,
}

impl Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\r\n", self.header)
    }
}
