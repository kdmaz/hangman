mod word_list;
use rand::prelude::SliceRandom;
use std::{collections::HashSet, io};

fn main() {
    let valid_chars = get_valid_chars();
    let word_list = word_list::get_word_list();

    'program: loop {
        let random_word = *word_list.choose(&mut rand::thread_rng()).unwrap();
        let mut guessed_chars = HashSet::new();
        let mut guesses_remaining = 10;
        let mut display_word = get_display_word(&random_word, &guessed_chars);

        'game: while guesses_remaining > 0 {
            println!("{}", display_word);

            let char_guess = match get_char_guess(&valid_chars, &guessed_chars) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            };

            if !random_word.contains(&char_guess.to_string()) {
                guesses_remaining -= 1;
            }
            guessed_chars.insert(char_guess);

            display_word = get_display_word(&random_word, &guessed_chars);
            if !display_word.contains("_") {
                println!("You won! The word is {}", random_word);
                break 'game;
            }

            println!("Guesses remaining ({})", guesses_remaining);

            if guesses_remaining == 0 {
                println!("The word is {}", random_word);
            }
        }

        if !play_again() {
            break 'program;
        }
    }
}

fn get_valid_chars() -> HashSet<char> {
    let mut valid_chars = HashSet::new();
    for c in "abcdefghijklmnopqrstuvwxyz".chars() {
        valid_chars.insert(c);
    }
    valid_chars
}

fn get_display_word(random_word: &str, guessed_chars: &HashSet<char>) -> String {
    random_word
        .chars()
        .enumerate()
        .map(|(i, letter)| {
            let is_last_letter = random_word.len() - 1 == i;
            let width = if is_last_letter { 1 } else { 2 };
            let letter = if guessed_chars.contains(&letter) {
                letter
            } else {
                '_'
            };
            format!("{0:1$}", letter, width)
        })
        .collect()
}

fn get_char_guess(
    valid_chars: &HashSet<char>,
    guessed_chars: &HashSet<char>,
) -> Result<char, String> {
    println!("Guess a letter!");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).unwrap();
    guess = guess.trim().to_lowercase();

    if guess.len() > 1 {
        return Err(String::from("You can only guess one character!"));
    }

    let char_guess = match guess.chars().next() {
        Some(c) if valid_chars.contains(&c) => c,
        None => return Err(String::from("You can't guess an empty character!")),
        _ => return Err(format!("\"{}\" is not a valid guess!", guess)),
    };

    if guessed_chars.contains(&char_guess) {
        return Err(format!("You already guessed \"{}\"!", char_guess));
    }

    Ok(char_guess)
}

fn play_again() -> bool {
    println!("Play again? (y/n)");

    let mut response = String::new();
    io::stdin().read_line(&mut response).unwrap();
    response.trim().to_lowercase().contains("y")
}
