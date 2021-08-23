use std::str::FromStr;

// Declare a Method type to handle the different HTTP request methods as variants.

#[derive(Debug)]
pub enum Method {
    CONNECT,
    DELETE,
    HEAD,
    GET,
    OPTIONS,
    PATCH,
    POST,
    PUT,
    TRACE,
}

// Implementing FromStr on the Method enums allows us to convert strings from the requests
// to the corresponding variant.
impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CONNECT" => Ok(Self::CONNECT),
            "DELETE" => Ok(Self::DELETE),
            "HEAD" => Ok(Self::HEAD),
            "GET" => Ok(Self::GET),
            "OPTIONS" => Ok(Self::OPTIONS),
            "PATCH" => Ok(Self::PATCH),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "TRACE" => Ok(Self::TRACE),
            _ => Err(MethodError),
        }
    }
}

pub struct MethodError;
