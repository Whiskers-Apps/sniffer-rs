
/// Returns true if all the search characters are inside of the original string.
pub fn get_inner_match(original: impl Into<String>, search: impl Into<String>) -> bool {
    let original = original.into();
    let search = search.into();

    let original_chars: Vec<char> = original.chars().collect();
    let search_chars: Vec<char> = search.chars().collect();

    let mut has_all_chars = true;

    for char in search_chars {
        if !original_chars.contains(&char) {
            has_all_chars = false;
            break;
        }
    }

    has_all_chars
}
