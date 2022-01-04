use std::collections::HashSet;

use itertools::Itertools;

fn update_available_letters(
    new_words: Vec<(char, char)>,
    word_options: &[HashSet<char>],
) -> Vec<HashSet<char>> {
    // Gather the black letters
    let black_letters: HashSet<char> = new_words
        .iter()
        // Get the item at index 1
        .filter(|&v| v.1 == 'b')
        .map(|v| v.0)
        .collect();

    // Gather the yellow letters and the position at which to remove it
    let yellow_letters: Vec<(usize, char)> = new_words
        .iter()
        .enumerate()
        // Get the item at index 1
        .filter(|(_, v)| v.1 == 'y')
        .map(|(idx, v)| (idx, v.0))
        .collect();

    // Gather the green letters and the position at which to remove it
    let green_letters: Vec<(usize, char)> = new_words
        .iter()
        .enumerate()
        // Get the item at index 1
        .filter(|(_, v)| v.1 == 'g')
        .map(|(idx, v)| (idx, v.0))
        .collect();

    // Create the result array
    let result = word_options.clone();

    // Remove the black letters from all positions
    let mut result: Vec<HashSet<char>> = result
        .iter()
        .map(|letters| {
            letters
                .difference(&black_letters)
                .copied()
                .collect::<HashSet<_>>()
        })
        .collect();

    // Remove the yellow letters from their specified positions
    yellow_letters.iter().for_each(|(idx, letter)| {
        let _ = result[*idx].remove(letter);
    });

    // Remove everything but the green letters at their indices
    green_letters.iter().for_each(|(idx, letter)| {
        result[*idx] = HashSet::from([*letter]);
    });

    result
}

fn get_word_input() -> Vec<(char, char)> {
    // Get input from the user
    let _ = std::io::Write::flush(&mut std::io::stdout());
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");

    // We expect five sets of two characters, seperated by spaces
    let new_words: Vec<Vec<char>> = input
        .split_ascii_whitespace()
        .map(|s| s.chars().take(2).collect::<Vec<char>>())
        .collect();
    assert!(new_words.len() == 5, "Did not get 5 pairs");

    // Convert the inner vec to a tuple
    let new_words: Vec<(char, char)> = new_words
        .iter()
        .inspect(|v| assert!(v.len() == 2))
        .map(|v| (v[0], v[1]))
        .collect();

    new_words
}

fn main() {
    let input_str = std::fs::read_to_string("american_english_dictionary.txt")
        .expect("Could not read dictionary");

    let valid_lowercase_words: Vec<String> = input_str
        .lines()
        // Filter out anything with an apostrophe
        .filter(|&s| !s.ends_with("'s"))
        // Filter out anything that is not 5 letters
        .filter(|&s| s.len() == 5)
        // Convert all to lowercase
        .map(|s| s.to_lowercase())
        // Remove duplicates by sorting and deduping
        .sorted()
        .dedup()
        .collect();
}

#[test]
fn test_update_1() {
    let alphabet: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let word_options: Vec<HashSet<char>> = vec![
        alphabet.clone(),
        alphabet.clone(),
        alphabet.clone(),
        alphabet.clone(),
        alphabet.clone(),
    ];

    let input_letters = vec![('a', 'b'), ('b', 'g'), ('c', 'y'), ('d', 'b'), ('e', 'g')];
    let expected: Vec<HashSet<char>> = vec![
        "bcefghijklmnopqrstuvwxyz".chars().collect(),
        HashSet::from(['b']),
        "befghijklmnopqrstuvwxyz".chars().collect(),
        "bcefghijklmnopqrstuvwxyz".chars().collect(),
        HashSet::from(['e']),
    ];

    let got = update_available_letters(input_letters, &word_options);
    for (idx, (expectedi, goti)) in expected.iter().zip(got.iter()).enumerate() {
        assert_eq!(expectedi, goti, "index {} did not match", idx);
    }
}
