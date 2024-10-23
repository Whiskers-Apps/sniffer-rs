/// Returns true if the search string is contained in the original.
pub fn get_contain_match(original: impl Into<String>, search: impl Into<String>) -> bool {
    let original = original.into().replace(" ", "");
    let search = search.into().replace(" ", "");

    original.contains(&search)
}
