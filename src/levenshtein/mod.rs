///Returns the amount of characters needed to change for the strings to match.
pub fn get_levenshtein_distance(original: impl Into<String>, search: impl Into<String>) -> usize {
    let original = original.into();
    let original_chars: Vec<char> = original.chars().collect();

    let search = search.into();
    let search_chars: Vec<char> = search.chars().collect();

    let original_size = original.len();
    let search_size = search.len();

    let mut distance: Vec<Vec<usize>> = vec![vec![0; search_size + 1]; original_size + 1];

    for i in 0..(original_size + 1) {
        distance[i][0] = i;
    }

    for i in 0..(search_size + 1) {
        distance[0][i] = i;
    }

    for i in 1..(original_size + 1) {
        for j in 1..(search_size + 1) {
            let cost = if original_chars[i - 1] == search_chars[j - 1] {
                0
            } else {
                1
            };

            distance[i][j] = (distance[i - 1][j] + 1)
                .min(distance[i][j - 1] + 1)
                .min(distance[i - 1][j - 1] + cost);
        }
    }

    return distance[original_size][search_size];
}
