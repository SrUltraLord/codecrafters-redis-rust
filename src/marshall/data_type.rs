// See RESP protocol description
// https://redis.io/docs/reference/protocol-spec
//
pub const CRLF: &str = "\r\n";

pub(crate) enum RedisDataType {
    String,
    // BulkString,
    // Error,
    // Integer,
    // Array,
}

impl RedisDataType {
    pub fn first_byte(&self) -> &str {
        match self {
            Self::String => "+",
            // Self::BulkString => "$",
            // Self::Error => "-",
            // Self::Integer => ":",
            // Self::Array => "*",
        }
    }
}
