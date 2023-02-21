use crate::marshall::data_type::{RedisDataType, CRLF};

pub fn marshall(error_message: String) -> String {
    let first_byte = RedisDataType::Error.first_byte();
    format!("{}{}{}", first_byte, error_message, CRLF)
}
