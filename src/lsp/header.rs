use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::num::ParseIntError;
use std::error::Error;

const CONTENT_TYPE_NAME: &'static str = "Content-Type";
const CONTENT_LENGTH_NAME: &'static str = "Content-Length";

#[derive(Debug)]
pub struct Header {
    pub content_length: u32,
    pub content_type: Option<String>
}

impl Header {
    fn new(content_length: u32) -> Self {
        Header {
            content_length,
            content_type: Some("application/vscode-jsonrpc; charset=utf-8".to_string())
        }
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}\r\n", CONTENT_LENGTH_NAME, self.content_length)?;
        match self.content_type.as_ref() {
            Some(content_type) => write!(f, "{}: {}\r\n", CONTENT_TYPE_NAME, content_type),
            None => Ok(())
        }
    }
}

impl FromStr for Header {
    type Err = HeaderParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut content_length: Option<u32> = None;
        let mut content_type: Option<String> = None;

        for field in s.split("\r\n") {
            let field_parts: Vec<&str> = field.split(": ").collect();
            if field_parts.len() != 2 {
                return Err(HeaderParseError::SyntaxError);
            }

            match field_parts[0] {
                CONTENT_LENGTH_NAME => { content_length.replace(field_parts[1].parse().map_err(|err| HeaderParseError::LengthParseError(err))?); },
                CONTENT_TYPE_NAME => { content_type.replace(field_parts[1].to_string()); },
                field_name => return Err(HeaderParseError::UnknownField { field_name: field_name.to_string() })
            }
        }

        Ok(Header {
            content_type,
            content_length: content_length.ok_or(HeaderParseError::MissingField { field_name: "Content-Length".to_string() })?
        })
    }
}

#[derive(Debug)]
pub enum HeaderParseError {
    MissingField{ field_name: String },
    LengthParseError(ParseIntError),
    UnknownField{ field_name: String },
    SyntaxError
}

impl Display for HeaderParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingField { field_name } => write!(f, "missing field \"{}\" in header", field_name),
            Self::LengthParseError(err) => write!(f, "error parsing length: {}", err),
            Self::UnknownField { field_name } => write!(f, "unknown field \"{}\" in header", field_name),
            Self::SyntaxError => write!(f, "could not parse header")
        }
    }
}

impl Error for HeaderParseError {}
