pub fn is_byte_pair_crlf(first_byte: &u8, second_byte: &u8) -> bool {
    *first_byte == 13 && *second_byte == 10
}

pub fn is_size_part(crlf_count: &usize) -> bool {
    *crlf_count == 0
}
