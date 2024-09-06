use crate::hamming::get_hamming_distance;
use crate::inner::get_inner_match;
use crate::jaro_winkler::get_jaro_winkler_distance;
use crate::levenshtein::get_levenshtein_distance;

#[derive(Debug, Clone)]
/** An object containing the results of all the algorithms*/
pub struct SnifferResult {
    pub levenshtein: usize,
    pub hamming: isize,
    pub jaro_winkler: f64,
    pub inner: bool,
}

#[derive(Debug, Clone)]
pub struct Sniffer {
    /// The amount of characters that can be different
    levenshtein_distance: usize,
    /// Use levenshtein match
    do_levenshtein_match: bool,
    /// The amount of positional characters that can be different
    hamming_distance: usize,
    /// Use hamming match
    do_hamming_match: bool,
    /// The difference between the search and the original. From 0.0 to 1.0
    jaro_winkler_distance: f64,
    /// Use jaro winkler match
    do_jaro_winkler_match: bool,
    /// Use inner match
    do_inner_match: bool,
    /// Do case-sensitive search
    case_sensitive: bool,
}

impl Sniffer {
    pub fn new() -> Self {
        Self {
            levenshtein_distance: 2,
            do_levenshtein_match: true,
            hamming_distance: 2,
            do_hamming_match: true,
            jaro_winkler_distance: 0.8,
            do_jaro_winkler_match: true,
            do_inner_match: true,
            case_sensitive: false,
        }
    }

    /// Returns true if any of the algorithms has a match between the strings
    pub fn matches(self, original: impl Into<String>, search: impl Into<String>) -> bool {
        let first_word = if self.case_sensitive {
            original.into()
        } else {
            original.into().to_lowercase()
        };

        let second_word = if self.case_sensitive {
            search.into()
        } else {
            search.into().to_lowercase()
        };

        let levenshtein_match = if self.do_levenshtein_match {
            get_levenshtein_distance(&first_word, &second_word) <= self.levenshtein_distance
        } else {
            false
        };

        let hamming_match = if self.do_hamming_match && first_word.len() == second_word.len() {
            get_hamming_distance(&first_word, &second_word) <= self.hamming_distance
        } else {
            false
        };

        let jaro_winkler_match = if self.do_jaro_winkler_match {
            get_jaro_winkler_distance(&first_word, &second_word) >= self.jaro_winkler_distance
        } else {
            false
        };

        let inner_match = if self.do_inner_match {
            get_inner_match(&first_word, &second_word)
        } else {
            false
        };

        levenshtein_match || hamming_match || jaro_winkler_match || inner_match
    }

    /// Get a sniffer result object containing the results of all the matches.
    /// Hamming match returns -1 if the strings are a different size
    pub fn get_sniffer_result(
        self,
        original: impl Into<String>,
        search: impl Into<String>,
    ) -> SnifferResult {
        let first_word = if self.case_sensitive {
            original.into()
        } else {
            original.into().to_lowercase()
        };

        let second_word = if self.case_sensitive {
            search.into()
        } else {
            search.into().to_lowercase()
        };

        return SnifferResult {
            levenshtein: get_levenshtein_distance(&first_word, &second_word),
            hamming: if first_word.len() == second_word.len() {
                get_hamming_distance(&first_word, &second_word) as isize
            } else {
                -1
            },
            jaro_winkler: get_jaro_winkler_distance(&first_word, &second_word),
            inner: get_inner_match(&first_word, &second_word),
        };
    }

    pub fn set_levenshtein_distance(&mut self, levenshtein_distance: usize) {
        self.levenshtein_distance = levenshtein_distance;
    }

    pub fn set_do_levenshtein_match(&mut self, do_levenshtein_match: bool) {
        self.do_levenshtein_match = do_levenshtein_match;
    }

    pub fn set_hamming_distance(&mut self, hamming_distance: usize) {
        self.hamming_distance = hamming_distance;
    }

    pub fn set_do_hamming_match(&mut self, do_hamming_match: bool) {
        self.do_hamming_match = do_hamming_match;
    }

    pub fn set_jaro_winkler_distance(&mut self, jaro_winkler_distance: f64) {
        self.jaro_winkler_distance = jaro_winkler_distance;
    }

    pub fn set_do_jaro_winkler_match(&mut self, do_jaro_winkler_match: bool) {
        self.do_jaro_winkler_match = do_jaro_winkler_match;
    }

    pub fn set_do_inner_match(&mut self, do_inner_match: bool) {
        self.do_inner_match = do_inner_match;
    }

    pub fn set_case_sensitive(&mut self, case_sensitive: bool) {
        self.case_sensitive = case_sensitive;
    }
}
