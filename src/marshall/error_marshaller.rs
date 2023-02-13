// use std::io::Error;
//
// use crate::marshall::data_type::RedisDataType;

// pub fn marshall_error(error: &Error) -> String {
//     let first_byte = RedisDataType::Error.first_byte();
//
//     format!("{}{}{}", first_byte, error.to_string(), "\r\n")
// }
