
use crate::{
    hamming::get_hamming_distance, inner::get_inner_match, jaro_winkler::get_jaro_winkler_distance,
    levenshtein::get_levenshtein_distance, sniffer::Sniffer,
};

#[test]
fn levenshtein() {
    let expected = 3;
    let actual = get_levenshtein_distance("Banana", "banini");
    assert_eq!(actual, expected);
}

#[test]
fn hamming() {
    let expected = 2;
    let actual = get_hamming_distance("bulbasaur", "bulbysaul");
    assert_eq!(actual, expected);
}

#[test]
fn jaro_winkler() {
    let expected = 0.9666666666666667;
    let actual = get_jaro_winkler_distance("banana", "banan");
    assert_eq!(actual, expected);
}

#[test]
fn inner() {
    let expected = true;
    let actual = get_inner_match("Sprigatito", "agt");
    assert_eq!(actual, expected);
}

#[test]
fn sniffer() {
    let sniffer = Sniffer::new();
    let expected = true;
    let actual = sniffer.matches("Banana", "banana");

    assert_eq!(actual, expected);
}

#[test]
fn sniffer_result(){
    let sniffer = Sniffer::new();
    let result = sniffer.get_sniffer_result("Luxray", "lux");

    assert_eq!(result.levenshtein, 3);
    assert_eq!(result.jaro_winkler, 0.8833333333333334);
    assert_eq!(result.inner, true);
}