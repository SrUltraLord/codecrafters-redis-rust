use super::data_type::{RedisDataType, CRLF};
use super::marshall_utils::{is_byte_pair_crlf, is_size_part};

const CRLF_SEPARATORS_FOR_BULK_STR: usize = 2;

pub fn marshall(value: String) -> String {
    let first_byte = RedisDataType::BulkString.first_byte();
    let string_length = value.len();

    format!("{}{}{}{}{}", first_byte, string_length, CRLF, value, CRLF)
}

// TODO: Handle specified length error.
pub fn unmarshall_from_buffer(
    buffer: &[u8],
    start_idx: Option<&usize>,
) -> Result<(String, usize), String> {
    let mut size_buffer = Vec::<u8>::new();
    let mut result_buffer = Vec::<u8>::new();
    let mut crlf_count: usize = 0;
    let mut last_visited_idx = 0;

    let start_idx = start_idx.copied().unwrap_or(0);
    let mut i = start_idx;
    while i <= buffer.len() - 1 {
        if is_byte_pair_crlf(&buffer[i], &buffer[i + 1]) {
            i += 2; // Skip CRLF
            crlf_count += 1;

            if crlf_count == CRLF_SEPARATORS_FOR_BULK_STR {
                last_visited_idx = i;
                break;
            }

            continue;
        }

        if is_size_part(&crlf_count) {
            if i != start_idx {
                size_buffer.push(buffer[i]);
            }

            i += 1;
            continue;
        }

        result_buffer.push(buffer[i]);
        i += 1;
    }

    let result = String::from_utf8(result_buffer).unwrap_or(String::default());

    Ok((result, last_visited_idx))
}
