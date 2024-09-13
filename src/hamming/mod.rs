/// Returns the amount of different positional characters (Hamming distance).
///
/// More at https://en.wikipedia.org/wiki/Hamming_distance
pub fn get_hamming_distance(original: impl Into<String>, search: impl Into<String>) -> usize {

    let original = original.into();
    let original_chars: Vec<char> = original.chars().collect();
    let search = search.into();
    let search_chars: Vec<char> = search.chars().collect();

    if original_chars.len() != search_chars.len() {
        panic!("Strings must have the same length");
    }

    let mut distance = 0;

    for (index, char) in original_chars.iter().enumerate() {
        if char != &search_chars[index] {
            distance += 1;
        }
    }

    distance
}
