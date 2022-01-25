use std::{
    char,
    collections::{HashMap, HashSet},
};

use clap::Parser;
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
    let new_words: Vec<(char, char)> = new_words.iter().map(|v| (v[0], v[1])).collect();

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

fn score_word(word: &str, letter_frequency: &HashMap<char, usize>) -> usize {
    // Get the unique characters
    word.chars()
        .sorted()
        .dedup()
        // Get the score from each
        .map(|letter_to_find| {
            letter_frequency
                .get(&letter_to_find)
                .expect("Found an alien letter")
        })
        .sum()
}

fn mimic_user_input(correct_word: &str, current_guess: &str) -> Vec<(char, char)> {
    // Assuming both are strings of length 5
    correct_word
        .chars()
        .zip(current_guess.chars())
        .map(|(correct_letter, guessed_letter)| {
            if guessed_letter == correct_letter {
                (guessed_letter, 'g')
            } else if correct_word.contains(guessed_letter) {
                (guessed_letter, 'y')
            } else {
                (guessed_letter, 'b')
            }
        })
        .collect()
}

/// If we already have a word in mind, count how many guesses are required to get it
fn guess_word(word_to_guess: &str, input_words: &[&str]) -> u8 {
    let alphabet: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut word_options: Vec<HashSet<char>> = vec![
        alphabet.clone(),
        alphabet.clone(),
        alphabet.clone(),
        alphabet.clone(),
        alphabet,
    ];

    let mut valid_words: Vec<&str> = input_words.iter().copied().collect();

    let mut current_guess = valid_words[0];

    let mut n_guesses: u8 = 0;
    // Loop until we find the word
    loop {
        n_guesses += 1;

        if current_guess == word_to_guess {
            break;
        }

        if n_guesses > 20 {
            println!("Could not {word_to_guess} after 20 guesses");
            break;
        }

        // Generate what would be the user's input.
        let user_input = mimic_user_input(word_to_guess, current_guess);

        // Update which letters can be used where
        let r = update_available_letters(&user_input, &word_options);
        word_options = r.0;
        let yellow_letters = r.1;

        // Update the list of available words
        valid_words.retain(|s| word_is_valid(s, &word_options, &yellow_letters));

        // Exit if no more words are available
        if valid_words.is_empty() {
            break;
        }

        // Update current best guess.
        current_guess = valid_words[0];
    }

    n_guesses
}

/// The idea here is to attempt a different method. It will provide the "best" word as a
/// first option, and then the next best word that contains none of the same letters
/// as in the first. E.g. "later" and then "sonic". It then continues on from there as
/// done before
fn guess_word_method_2(word_to_guess: &str, input_words: &[&str]) -> u8 {
    // Make a mutable copy of `valid_words`
    let mut valid_words: Vec<&str> = input_words.iter().copied().collect();

    let alphabet: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut word_options: Vec<HashSet<char>> = vec![
        alphabet.clone(),
        alphabet.clone(),
        alphabet.clone(),
        alphabet.clone(),
        alphabet,
    ];

    // Get the best starting word, and the next best word after that
    let best_starting_word = valid_words[0];
    let second_word = input_words
        .iter()
        .find(|&&s| !s.chars().any(|c| best_starting_word.contains(c)))
        .expect("Could not find a second best word");

    // Return 1 or 2 if either best or second is the word_to_guess
    if best_starting_word == word_to_guess {
        return 1;
    } else if *second_word == word_to_guess {
        return 2;
    }

    // Mimic the user's input
    let user_input = mimic_user_input(word_to_guess, best_starting_word);

    // Update which letters can be used where
    let r = update_available_letters(&user_input, &word_options);
    word_options = r.0;
    let yellow_letters = r.1;

    // Update the list of available words
    valid_words.retain(|s| word_is_valid(s, &word_options, &yellow_letters));

    // If there's just one item left in valid_words, then we know it'll take two guesses
    if valid_words.len() == 1 {
        return 2;
    }

    // Get the user's input
    let user_input = mimic_user_input(word_to_guess, second_word);

    // Update which letters can be used where
    let r = update_available_letters(&user_input, &word_options);
    word_options = r.0;
    let yellow_letters = r.1;

    // Update the list of available words
    valid_words.retain(|s| word_is_valid(s, &word_options, &yellow_letters));

    if valid_words.len() == 1 {
        return 3;
    }

    // Create the current guess
    let mut current_guess = valid_words[0];

    // The guess count. They have to put in two guesses, to get here, and then a new guess
    let mut n_guesses: u8 = 2;

    // Loop for the remaining 4 turns
    for _ in 1..=4 {
        // Update number of guesses
        n_guesses += 1;

        // Exit if correct guess
        if current_guess == word_to_guess {
            return n_guesses;
        }

        // Get the user's input
        let user_input = mimic_user_input(word_to_guess, current_guess);

        // Update which letters can be used where
        let r = update_available_letters(&user_input, &word_options);
        word_options = r.0;
        let yellow_letters = r.1;

        // Update the list of available words
        valid_words.retain(|s| word_is_valid(s, &word_options, &yellow_letters));

        // Exit if no more words are available
        if valid_words.is_empty() {
            break;
        }

        // Update current best guess.
        current_guess = valid_words[0];
    }

    n_guesses
}

/// This program will help you solve wordle more quickly.
/// Run with no arguments to have it help you. You can optionally run in "hard mode", or
/// run the efficiency tests, and tell you how many guesses were required for each word,
/// in each mode.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Run the efficiency tests?
    #[clap(short, long)]
    run_tests: bool,

    /// Use the hard mode method
    #[clap(long)]
    hard_mode: bool,
}

fn main() {
    let start_time = std::time::Instant::now();

    let input_str =
        std::fs::read_to_string("allowed_words.txt").expect("Could not read dictionary");

    let words: Vec<&str> = input_str.lines().collect();

    // Get the frequency of letters in the target list
    let letter_frequency = words.iter().flat_map(|s| s.chars()).counts();

    // Score the words, and sort them by their score, highest to smallest.
    let valid_words: Vec<&str> = words
        .iter()
        .map(|word| (word, score_word(word, &letter_frequency)))
        .sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
        .map(|(word, _)| *word)
        .collect();

    let mut mut_valid_words = valid_words.clone();

    let args = Args::parse();
    if args.run_tests {
        // Start a timer for the tests
        let test_time = std::time::Instant::now();

        // Print out headers
        println!("word,n_hard_mode_guesses");

        // Count how long it takes to guess each word
        valid_words
            .iter()
            .map(|word| (*word, guess_word(word, &valid_words)))
            .sorted_by(|(_, count1), (_, count2)| Ord::cmp(count1, count2))
            .for_each(|(word, count)| println!("{word},{count}"));

        // Print out headers
        println!("\n\n\nword,n_easy_mode_guesses");

        // Count how long it takes to guess each word
        valid_words
            .iter()
            .map(|word| (*word, guess_word_method_2(word, &valid_words)))
            .sorted_by(|(_, count1), (_, count2)| Ord::cmp(count1, count2))
            .for_each(|(word, count)| println!("{word},{count}"));

        // Print out how long it took to guess for all words
        println!(
            "Testing {} words took {} ms",
            valid_words.len(),
            test_time.elapsed().as_millis()
        );

        // Print out how long it took to run everything
        println!(
            "Running the program took {} ms",
            start_time.elapsed().as_millis()
        );

        // Exit
        return;
    }

    let alphabet: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut word_options: Vec<HashSet<char>> = vec![
        alphabet.clone(),
        alphabet.clone(),
        alphabet.clone(),
        alphabet.clone(),
        alphabet,
    ];

    println!("Startup took {} ms", start_time.elapsed().as_millis());

    if args.hard_mode {
        // Now the main loop
        for _ in 1..=6 {
            // Ask the user for input
            println!("\nPlease enter your input letters and their color");
            println!("The words suggested are in order of most helpful to least");

            // Get the user's input
            let user_input = get_word_input();

            // Update which letters can be used where
            let r = update_available_letters(&user_input, &word_options);
            word_options = r.0;
            let yellow_letters = r.1;

            // Update the list of available words
            mut_valid_words.retain(|s| word_is_valid(s, &word_options, &yellow_letters));

            // Tell the user about them
            println!("\n\n{:?}", &valid_words);

            // Exit if `valid_words.len() <= 1`
            if valid_words.len() <= 1 {
                break;
            }
        }

        // Exit
        return;
    }

    // Get the best starting word, and the next best word after that
    let best_starting_word = valid_words.first().expect("No valid words");
    let second_word = valid_words
        .iter()
        .find(|&&s| !s.chars().any(|c| best_starting_word.contains(c)))
        .expect("Could not find a second best word");

    // Tell the user to input the best_starting_word, and enter the data from the site
    println!(
        "Please type '{}' into the site, and then enter the data from the site",
        best_starting_word
    );

    // Mimic the user's input
    let user_input = get_word_input();

    // Update which letters can be used where
    let r = update_available_letters(&user_input, &word_options);
    word_options = r.0;
    let yellow_letters = r.1;

    // Update the list of available words
    mut_valid_words.retain(|s| word_is_valid(s, &word_options, &yellow_letters));

    // If there's only one word left in valid_words, then print it and exit
    if valid_words.len() == 1 {
        println!("{}", valid_words[0]);
        return;
    }

    // Tell the user to input the second_word, and enter the data from the site
    println!(
        "\nPlease type '{}' into the site, and then enter the data from the site",
        second_word
    );

    // Get the user's input
    let mut user_input = get_word_input();

    // Update which letters can be used where
    let r = update_available_letters(&user_input, &word_options);
    word_options = r.0;
    let yellow_letters = r.1;

    // Update the list of available words
    mut_valid_words.retain(|s| word_is_valid(s, &word_options, &yellow_letters));

    // If there's only one word left in valid_words, then print it and exit
    if mut_valid_words.len() == 1 {
        println!("{}", mut_valid_words[0]);
        return;
    }

    // Loop for the remaining 4 turns
    for round in 1..=4 {
        if round != 1 {
            // Ask the user for input
            println!("\nPlease enter your input letters and their color");
            println!("The words suggested are in order of most helpful to least");

            // Get the user's input
            user_input = get_word_input();
        }

        // Update which letters can be used where
        let r = update_available_letters(&user_input, &word_options);
        word_options = r.0;
        let yellow_letters = r.1;

        // Update the list of available words
        mut_valid_words.retain(|s| word_is_valid(s, &word_options, &yellow_letters));

        // Tell the user about them
        println!("\n\n{:?}", &mut_valid_words);

        // Exit if no more words are available
        if mut_valid_words.is_empty() {
            break;
        }
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

#[test]
fn test_score_word_1() {
    let letter_frequency: HashMap<char, usize> = HashMap::from([
        ('e', 1233),
        ('a', 979),
        ('r', 899),
        ('o', 754),
        ('t', 729),
        ('l', 719),
        ('i', 671),
        ('s', 669),
        ('n', 575),
        ('c', 477),
        ('u', 467),
        ('y', 425),
        ('d', 393),
        ('h', 389),
        ('p', 367),
        ('m', 316),
        ('g', 311),
        ('b', 281),
        ('f', 230),
        ('k', 210),
        ('w', 195),
        ('v', 153),
        ('z', 40),
        ('x', 37),
        ('q', 29),
        ('j', 27),
    ]);
    let word = "hello";
    let got = score_word(word, &letter_frequency);
    let expected: usize = 1233 + 389 + 719 + 754;
    assert_eq!(expected, got);
}

#[test]
fn test_guess_word_1() {
    // This is a word we should guess in one
    let word_to_guess = "later";

    let input_str =
        std::fs::read_to_string("allowed_words.txt").expect("Could not read dictionary");

    let words: Vec<&str> = input_str.lines().collect();

    // Get the frequency of letters in the target list
    let letter_frequency = words.iter().flat_map(|s| s.chars()).counts();

    // Score the words, and sort them by their score, highest to smallest.
    let valid_words: Vec<&str> = words
        .iter()
        .map(|word| (word, score_word(word, &letter_frequency)))
        .sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
        .map(|(word, _)| *word)
        .collect();

    let got = guess_word(word_to_guess, &valid_words);
    assert_eq!(1, got);
}

#[test]
fn test_guess_word_2() {
    // This is a word we should guess in 2
    let word_to_guess = "irate";

    let input_str =
        std::fs::read_to_string("allowed_words.txt").expect("Could not read dictionary");

    let words: Vec<&str> = input_str.lines().collect();

    // Get the frequency of letters in the target list
    let letter_frequency = words.iter().flat_map(|s| s.chars()).counts();

    // Score the words, and sort them by their score, highest to smallest.
    let valid_words: Vec<&str> = words
        .iter()
        .map(|word| (word, score_word(word, &letter_frequency)))
        .sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
        .map(|(word, _)| *word)
        .collect();

    let got = guess_word(word_to_guess, &valid_words);
    assert_eq!(2, got);
}

#[test]
fn test_guess_word_3() {
    // This is a word we should guess in 3
    let word_to_guess = "crimp";

    let input_str =
        std::fs::read_to_string("allowed_words.txt").expect("Could not read dictionary");

    let words: Vec<&str> = input_str.lines().collect();

    // Get the frequency of letters in the target list
    let letter_frequency = words.iter().flat_map(|s| s.chars()).counts();

    // Score the words, and sort them by their score, highest to smallest.
    let valid_words: Vec<&str> = words
        .iter()
        .map(|word| (word, score_word(word, &letter_frequency)))
        .sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
        .map(|(word, _)| *word)
        .collect();

    let got = guess_word(word_to_guess, &valid_words);
    assert_eq!(3, got);
}
