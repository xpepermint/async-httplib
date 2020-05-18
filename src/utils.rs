pub fn has_sequence(bytes: &[u8], needle: &[u8]) -> bool {
    let mut found = 0;
    let nsize = needle.len();
    for byte in bytes.into_iter() {
        if *byte == needle[found] {
            found += 1;
        } else {
            found = 0;
        }
        if found == nsize {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn checks_has_sequence() {
        assert!(has_sequence(&[0x0D, 0x0A, 0x0D, 0x0A], &[0x0D, 0x0A, 0x0D, 0x0A]));
        assert!(has_sequence(&[1, 4, 6, 10, 21, 5, 150], &[10, 21, 5]));
        assert!(!has_sequence(&[1, 4, 6, 10, 21, 5, 150], &[10, 5]));
    }
}
