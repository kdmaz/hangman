use rand::Rng;
use std::{collections::HashSet, fs, io};

fn main() {
    let valid_chars = get_valid_chars();
    let words = get_random_words();

    loop {
        let word = get_random_word(&words);
        let mut total_guesses = 10;
        let mut guesses: HashSet<String> = HashSet::new();

        while total_guesses > 0 {
            let guess = match get_letter_guess(&valid_chars) {
                Ok(g) => g,
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            };

            if guesses.contains(&guess) {
                println!("You already guessed \"{}\"!", guess);
                continue;
            }

            println!("Random word is \"{}\"", word);
            println!("Your guess is \"{}\"", guess);

            guesses.insert(guess);
            total_guesses -= 1;

            println!("Guesses remaining ({})", total_guesses);
        }

        // play again?
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

fn get_random_word(words: &Vec<String>) -> &String {
    let index = rand::thread_rng().gen_range(0..words.len());
    &words[index]
}

fn get_letter_guess(valid_chars: &HashSet<String>) -> Result<String, String> {
    println!("Guess a letter!");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).unwrap();
    guess = guess.trim().to_lowercase();

    if !valid_chars.contains(guess.as_str()) {
        return Err(format!("\"{}\" is not a valid character!", guess));
    }

    Ok(guess)
}
