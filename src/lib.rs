pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut result = Vec::new();
    let n = digits.len();

    // Return empty vector if length is zero or exceeds input string length
    if len == 0 || len > n {
        return Vec::new();
    }

    // Iterate through the input string
    for i in 0..=(n - len) {
        // Extract substring of length `len` starting at index `i`
        let substring = &digits[i..(i + len)];
        result.push(substring.to_string());
    }

    result
}
