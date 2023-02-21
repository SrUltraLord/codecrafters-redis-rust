use super::bulk_string_marshaller;
use super::data_type::{RedisDataType, CRLF};
use super::marshall_utils::{is_byte_pair_crlf, is_size_part};

#[allow(dead_code)]
pub fn marshall(value: Vec<String>) -> String {
    let length = value.len();
    let first_byte = RedisDataType::Array.first_byte();

    let mut marshalled_strings = String::default();

    for string in value {
        let marshalled_string = bulk_string_marshaller::marshall(string);

        marshalled_strings.push_str(&marshalled_string);
    }

    format!("{}{}{}{}", first_byte, length, CRLF, marshalled_strings)
}

// TODO: handle numbers
pub fn unmarshall_from_buffer(buffer: &[u8]) -> Vec<String> {
    let mut total_items_buffer = Vec::<u8>::new();
    let mut items = Vec::<String>::new();
    let mut crlf_count = 0;

    let mut i: usize = 0;
    while i <= buffer.len() - 1 {
        if is_byte_pair_crlf(&buffer[i], &buffer[i + 1]) {
            i += 2; // Skip CRLF
            crlf_count += 1;
            continue;
        }

        if is_size_part(&crlf_count) {
            // Skip first byte.
            if i != 0 {
                total_items_buffer.push(buffer[i]);
            }

            i += 1;
            continue;
        }

        // Handle elements
        let (item, last_visited_idx) =
            bulk_string_marshaller::unmarshall_from_buffer(&buffer, Some(&i)).unwrap();

        items.push(item);

        if last_visited_idx != 0 {
            i = last_visited_idx;
            continue;
        }

        i += 1;
    }

    items
}
