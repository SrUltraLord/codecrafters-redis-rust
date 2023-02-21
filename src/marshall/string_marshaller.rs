use crate::marshall::data_type::{RedisDataType, CRLF};

pub fn marshall(value: String) -> String {
    let first_byte = RedisDataType::String.first_byte();
    format!("{}{}{}", first_byte, value, CRLF)
}
