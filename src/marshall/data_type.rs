// See RESP protocol description
// https://redis.io/docs/reference/protocol-spec
//

pub const CRLF: &str = "\r\n";

pub(crate) enum RedisDataType {
    String,
    BulkString,
    Error,
    Array,
    Integer,
}

#[allow(dead_code)]
impl RedisDataType {
    pub fn first_byte(&self) -> char {
        match self {
            Self::String => '+',
            Self::BulkString => '$',
            Self::Error => '-',
            Self::Array => '*',
            Self::Integer => ':',
        }
    }

    pub fn from_first_byte(first_byte: char) -> Self {
        match first_byte {
            '+' => Self::String,
            '$' => Self::BulkString,
            '-' => Self::Error,
            '*' => Self::Array,
            ':' => Self::Integer,
            _ => Self::Error,
        }
    }
}
