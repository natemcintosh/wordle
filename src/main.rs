use std::collections::HashSet;

use itertools::Itertools;

fn update_available_letters(
    new_words: &[(char, char)],
    word_options: &[HashSet<char>],
) -> (Vec<HashSet<char>>, Vec<(usize, char)>) {
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

    // Create the result array as a copy of the input
    let result = word_options.to_vec();

    // Remove the black letters from all positions
    let mut available_letters: Vec<HashSet<char>> = result
        .iter()
        .map(|letters| {
            letters
                .difference(&black_letters)
                .copied()
                .collect::<HashSet<_>>()
        })
        .collect();

    // Remove the yellow letters from their specified positions
    for (idx, letter) in &yellow_letters {
        let _ = available_letters[*idx].remove(letter);
    }

    // Remove everything but the green letters at their indices
    for (idx, letter) in green_letters {
        available_letters[idx] = HashSet::from([letter]);
    }

    (available_letters, yellow_letters)
}

fn get_word_input() -> Vec<(char, char)> {
    // Get input from the user
    let _droppable = std::io::Write::flush(&mut std::io::stdout());
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");

    // We expect five sets of two characters, seperated by spaces
    let new_words: Vec<Vec<char>> = input
        .split_ascii_whitespace()
        .map(|s| s.chars().take(2).collect::<Vec<char>>())
        .collect();

    // If not 5 pairs, restart process
    if new_words.len() != 5 {
        println!("Please enter exactly five pairs");
        return get_word_input();
    }

    // Convert the inner vec to a tuple
    let new_words: Vec<(char, char)> = new_words
        .iter()
        .inspect(|v| assert!(v.len() == 2))
        .map(|v| (v[0], v[1]))
        .collect();

    new_words
}

fn word_is_valid(
    s: &str,
    word_options: &[HashSet<char>],
    yellow_letters: &[(usize, char)],
) -> bool {
    // First check to see if all the yellow letters appear, and that they are not in places they shouldn't be
    if !yellow_letters.iter().all(|(bad_idx, letter)| {
        s.chars()
            .enumerate()
            .filter(|(idx, _)| idx != bad_idx)
            .any(|(_, letter_to_check)| letter_to_check == *letter)
    }) {
        return false;
    }

    // Assumes that `s` has length 5
    for (letter, good_letters) in s.chars().zip(word_options.iter()) {
        if !good_letters.contains(&letter) {
            return false;
        }
    }
    true
}

fn main() {
    let input_str = std::fs::read_to_string("american_english_dictionary.txt")
        .expect("Could not read dictionary");

    let mut valid_words: Vec<String> = input_str
        .lines()
        // Filter out anything with an apostrophe
        .filter(|&s| !s.ends_with("'s"))
        // Filter out anything that is not 5 letters
        .filter(|&s| s.len() == 5)
        // Convert all to lowercase
        .map(str::to_lowercase)
        // Remove duplicates by sorting and deduping
        .sorted()
        .dedup()
        .collect();

    let alphabet: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut word_options: Vec<HashSet<char>> = vec![
        alphabet.clone(),
        alphabet.clone(),
        alphabet.clone(),
        alphabet.clone(),
        alphabet,
    ];

    // Now the main loop
    for _ in 1..=6 {
        // Ask the user for input
        println!("\nPlease enter your input letters and their color");

        // Get the user's input
        let user_input = get_word_input();

        // Update which letters can be used where
        let r = update_available_letters(&user_input, &word_options);
        word_options = r.0;
        let yellow_letters = r.1;

        // Update the list of available words
        valid_words.retain(|s| word_is_valid(s, &word_options, &yellow_letters));

        // Tell the user about them
        println!("\n\n{:?}", &valid_words);
    }
}

#[test]
fn test_update_1() {
    let alphabet: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let word_options: Vec<HashSet<char>> = vec![
        alphabet.clone(),
        alphabet.clone(),
        alphabet.clone(),
        alphabet.clone(),
        alphabet,
    ];

    let input_letters = vec![('a', 'b'), ('b', 'g'), ('c', 'y'), ('d', 'b'), ('e', 'g')];
    let expected: Vec<HashSet<char>> = vec![
        "bcefghijklmnopqrstuvwxyz".chars().collect(),
        HashSet::from(['b']),
        "befghijklmnopqrstuvwxyz".chars().collect(),
        "bcefghijklmnopqrstuvwxyz".chars().collect(),
        HashSet::from(['e']),
    ];

    let (got, _) = update_available_letters(&input_letters, &word_options);
    for (idx, (expectedi, goti)) in expected.iter().zip(got.iter()).enumerate() {
        assert_eq!(expectedi, goti, "index {} did not match", idx);
    }
}
