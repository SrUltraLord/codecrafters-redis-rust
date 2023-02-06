// See RESP protocol description
// https://redis.io/docs/reference/protocol-spec
pub(crate) enum DataType {
    String,
    Error,
    // Integer,
    // BulkString,
    // Array,
}

impl DataType {
    pub fn first_byte(&self) -> &str {
        match self {
            Self::String => "+",
            Self::Error => "-",
            // Self::Integer => ":",
            // Self::BulkString => "$",
            // Self::Array => "*",
        }
    }
}
