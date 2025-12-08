use std::collections::HashMap;

/// Check if the start of a string matches any key in a HashMap (prioritizing longer keys), and replace the prefix if a match is found
pub fn replace_prefix_longest(s: &str, map: &HashMap<String, String>) -> String {
    // Convert HashMap key-value pairs to a Vec and sort by key length in descending order (longest first)
    let mut sorted_pairs: Vec<_> = map.iter().collect();
    sorted_pairs.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

    // Iterate over sorted key-value pairs to find prefix match
    for (prefix, replacement) in sorted_pairs {
        if s.starts_with(prefix.as_str()) {
            // Replace the matched prefix and append the remaining substring
            return format!("{}{}", replacement, &s[prefix.len()..]);
        }
    }
    // Return original string if no prefix match found
    s.to_string()
}