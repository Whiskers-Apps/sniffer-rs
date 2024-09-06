use std::{cmp::{max, min}, iter::zip};

/// Returns the similarity of both strings. From 0 to 1
pub fn get_jaro_winkler_distance(original: impl Into<String>, search: impl Into<String>) -> f64 {
    let original = original.into();
    let original_chars: Vec<char> = original.chars().collect();
    let search = search.into();
    let search_chars: Vec<char> = search.chars().collect();

    let original_size = original.len() as isize;
    let search_size = search.len() as isize;

    if original_size == 0 && search_size == 0 {
        return 1.0;
    }

    let match_distance: isize = (max(original_size, search_size) / 2) - 1;

    let mut original_matches: Vec<bool> = original_chars.iter().map(|_| false).collect();
    let mut search_matches: Vec<bool> = search_chars.iter().map(|_| false).collect();

    let mut matches = 0;
    let mut transpositions = 0;


    for i in 0..original_size as usize{
        let start = max(0, i as isize - match_distance) as usize;
        let end = min(i as isize + match_distance + 1, search_size) as usize;

        for j in start..end{
            if !search_matches[j] && original_chars[i] == search_chars[j]{
                original_matches[i] = true;
                search_matches[j] = true;
                matches += 1;
                break;
            }
        }
    }

    if matches == 0 {
        return 0.0;
    }

    let mut transpositions_count = 0;

    for i in 0..original_size as usize{
        if original_matches[i]{
            while !search_matches[transpositions_count]{
                transpositions_count += 1;
            }

            if original_chars[i]!= search_chars[transpositions_count]{
                transpositions += 1;
            }

            transpositions_count += 1;
        }
    }

    transpositions /= 2;

    let jaro = ((matches as f64 / original_size as f64) + (matches as f64 / search_size as f64 ) + ((matches as f64  - transpositions as f64 ) / matches as f64 )) / 3.0;


    let mut count = 0;

    for (a, b) in zip(original_chars, search_chars.iter().copied()) {
        if a == b {
            count += 1;
        } else {
            break;
        }
        if count == 4 {
            break;
        }
    }

    let prefix_length = count.min(4);

    let jaro_winkler = jaro + (0.1 * prefix_length as f64 * (1.0 - jaro));

    jaro_winkler
}
