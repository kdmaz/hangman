use rand::Rng;
use std::{collections::HashSet, fs, io};

fn main() {
    let valid_chars = get_valid_chars();
    let words = get_random_words();

    'program: loop {
        let hidden_word = get_hidden_word(&words);
        let mut total_guesses = 10;
        let mut guesses: HashSet<String> = HashSet::new();
        let mut display_word = get_display_word(&hidden_word, &guesses);

        'game: while total_guesses > 0 {
            println!("{}", display_word);

            let guess = match get_user_guess(&valid_chars) {
                Ok(guess) => {
                    if guesses.contains(&guess) {
                        println!("You already guessed \"{}\"!", guess);
                        continue;
                    }
                    guess
                }
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            };

            guesses.insert(guess.clone());

            if !hidden_word.contains(&guess) {
                total_guesses -= 1;
            }

            display_word = get_display_word(&hidden_word, &guesses);
            if !display_word.contains("_") {
                println!("You won! The word is {}", hidden_word);
                break 'game;
            }

            println!("Guesses remaining ({})", total_guesses);

            if total_guesses == 0 {
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

fn get_random_words() -> Vec<String> {
    let filename = "random_words.txt";
    let random_words = fs::read_to_string(filename)
        .expect(format!("Could not read file \"{}\"!", filename).as_str());
    random_words
        .split("\r\n")
        .map(str::to_string)
        .collect::<Vec<_>>()
}

fn get_hidden_word(words: &Vec<String>) -> &String {
    let index = rand::thread_rng().gen_range(0..words.len());
    &words[index]
}

fn get_display_word(hidden_word: &String, guesses: &HashSet<String>) -> String {
    hidden_word
        .split("")
        .enumerate()
        .map(|(i, letter)| {
            let is_last_letter = hidden_word.len() == i;

            let get_letter_to_display = |letter| {
                if guesses.contains(letter) {
                    letter
                } else {
                    "_"
                }
            };

            if letter == "" {
                String::new()
            } else if is_last_letter {
                get_letter_to_display(letter).to_string()
            } else {
                format!("{} ", get_letter_to_display(letter))
            }
        })
        .collect()
}

fn get_user_guess(valid_chars: &HashSet<String>) -> Result<String, String> {
    println!("Guess a letter!");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).unwrap();
    guess = guess.trim().to_lowercase();

    if !valid_chars.contains(guess.as_str()) {
        return Err(format!("\"{}\" is not a valid character!", guess));
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
