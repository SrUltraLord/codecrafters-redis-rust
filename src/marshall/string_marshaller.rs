use crate::marshall::data_type::{RedisDataType, CRLF};
use crate::marshall::marshaller::Marshaller;

pub struct StringMarshaller {
    first_byte: String,
}

impl Marshaller<&str> for StringMarshaller {
    fn init() -> Self {
        Self {
            first_byte: String::from(RedisDataType::String.first_byte()),
        }
    }

    fn marshall(&self, value: &str) -> String {
        format!("{}{}{}", self.first_byte, value, CRLF)
    }
}
