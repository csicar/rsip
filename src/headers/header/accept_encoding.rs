use macros::{StringTyped, UntypedHeader};

#[derive(UntypedHeader, StringTyped, Debug, PartialEq, Eq, Clone)]
pub struct AcceptEncoding(String);
