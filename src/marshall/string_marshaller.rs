use crate::marshall::data_type::DataType;
use crate::marshall::marshaller::Marshaller;

pub struct StringMarshaller {
    value: String,
    first_byte: String,
}

impl Marshaller for StringMarshaller {
    fn marshall(&self) -> String {
        format!("{}{}{}", self.first_byte, self.value, "\r\n")
    }
}

impl StringMarshaller {
    pub fn new(value: &str) -> Self {
        Self {
            value: String::from(value),
            first_byte: String::from(DataType::String.first_byte()),
        }
    }
}
