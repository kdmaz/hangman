mod word_list;
use rand::prelude::SliceRandom;
use std::{collections::HashSet, io};

fn main() {
    let valid_chars = get_valid_chars();
    let word_list = word_list::get_word_list();

    'program: loop {
        let hidden_word = get_hidden_word(&word_list);
        let mut user_guessed_chars = HashSet::new();
        let mut display_word = get_display_word(&hidden_word, &user_guessed_chars);
        let mut guesses_remaining = 10;

        'game: while guesses_remaining > 0 {
            println!("{}", display_word);

            let user_guess = match get_user_guess(&valid_chars, &user_guessed_chars) {
                Ok(guess) => guess,
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            };

            if !hidden_word.contains(&user_guess.to_string()) {
                guesses_remaining -= 1;
            }
            user_guessed_chars.insert(user_guess);

            display_word = get_display_word(&hidden_word, &user_guessed_chars);
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

fn get_valid_chars() -> HashSet<char> {
    let mut valid_chars = HashSet::new();
    for c in "abcdefghijklmnopqrstuvwxyz".chars() {
        valid_chars.insert(c);
    }
    valid_chars
}

fn get_hidden_word<'a>(words: &'a [&str; 1000]) -> &'a str {
    &words.choose(&mut rand::thread_rng()).unwrap()
}

fn get_display_word(hidden_word: &str, user_guessed_chars: &HashSet<char>) -> String {
    hidden_word
        .chars()
        .enumerate()
        .map(|(i, letter)| {
            let is_last_letter = hidden_word.len() - 1 == i;
            let width = if is_last_letter { 1 } else { 2 };
            let letter = if user_guessed_chars.contains(&letter) {
                letter
            } else {
                '_'
            };
            format!("{0:1$}", letter, width)
        })
        .collect()
}

fn get_user_guess(
    valid_chars: &HashSet<char>,
    user_guesses: &HashSet<char>,
) -> Result<char, String> {
    println!("Guess a letter!");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).unwrap();
    guess = guess.trim().to_lowercase();

    if guess.len() > 1 {
        return Err(String::from("You can only guess one character!"));
    }

    let guess = match guess.chars().next() {
        Some(c) if valid_chars.contains(&c) => c,
        None => return Err(String::from("You can't guess an empty character!")),
        _ => return Err(format!("\"{}\" is not a valid guess!", guess)),
    };

    if user_guesses.contains(&guess) {
        return Err(format!("You already guessed \"{}\"!", guess));
    }

    Ok(guess)
}

fn play_again() -> bool {
    println!("Play again? (y/n)");

    let mut play_again = String::new();
    io::stdin().read_line(&mut play_again).unwrap();
    play_again.trim().to_lowercase().contains("y")
}
