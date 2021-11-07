use hangman::word_list;
use rand::prelude::SliceRandom;
use std::{collections::HashSet, io};

fn main() {
    let valid_chars = get_valid_chars();
    let words = word_list::get_word_list();

    'program: loop {
        let hidden_word = get_hidden_word(&words);
        let mut user_guesses = HashSet::new();
        let mut display_word = get_display_word(hidden_word, &user_guesses);
        let mut guesses_remaining = 10;

        'game: while guesses_remaining > 0 {
            println!("{}", display_word);

            let guess = match get_user_guess(&valid_chars, &user_guesses) {
                Ok(guess) => guess,
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            };

            if !hidden_word.contains(&guess) {
                guesses_remaining -= 1;
            }
            user_guesses.insert(guess);

            display_word = get_display_word(&hidden_word, &user_guesses);
            if !display_word.contains("_") {
                println!("You won! The word is {}", hidden_word);
                break 'game;
            }

            println!("Guesses remaining ({})", guesses_remaining);

            if guesses_remaining == 0 {
                println!("The word is {}", hidden_word);
            }
        }

        if !play_again() {
            break 'program;
        }
    }
}

fn get_valid_chars() -> HashSet<String> {
    let mut valid_chars = HashSet::new();
    for c in "abcdefghijklmnopqrstuvwxyz".split("") {
        valid_chars.insert(String::from(c));
    }
    valid_chars
}

fn get_hidden_word<'a>(words: &'a [&str; 1000]) -> &'a str {
    &words.choose(&mut rand::thread_rng()).unwrap()
}

fn get_display_word(hidden_word: &str, user_guesses: &HashSet<String>) -> String {
    hidden_word
        .split("")
        .filter(|&letter| !letter.is_empty())
        .enumerate()
        .map(|(i, letter)| {
            let is_last_letter = hidden_word.len() - 1 == i;
            let width = if is_last_letter { 1 } else { 2 };
            let letter = if user_guesses.contains(letter) {
                letter
            } else {
                "_"
            };
            format!("{0:1$}", letter, width)
        })
        .collect()
}

fn get_user_guess(
    valid_chars: &HashSet<String>,
    user_guesses: &HashSet<String>,
) -> Result<String, String> {
    println!("Guess a letter!");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).unwrap();
    guess = guess.trim().to_lowercase();

    if !valid_chars.contains(guess.as_str()) || guess == "" {
        return Err(format!("\"{}\" is not a valid guess!", guess));
    }

    if user_guesses.contains(&guess) {
        return Err(format!("You already guessed \"{}\"!", guess));
    }

    Ok(guess)
}

fn play_again() -> bool {
    println!("Play again? (y/n)");

    let mut yes_or_no = String::new();
    io::stdin().read_line(&mut yes_or_no).unwrap();
    yes_or_no = yes_or_no.trim().to_lowercase();
    [String::from("y"), String::from("yes")].contains(&yes_or_no)
}
