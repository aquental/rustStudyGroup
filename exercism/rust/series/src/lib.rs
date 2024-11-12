pub fn series(digits: &str, len: usize) -> Vec<String> {
    (len..digits.len() + 1)
        .map(|i| digits[i - len..i].to_owned())
        .collect()
}
