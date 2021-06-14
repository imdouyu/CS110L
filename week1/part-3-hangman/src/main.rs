// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);
    println!("Welcome to CS110L Hangman!");
    // println!("{}", secret_word);
    // println!("{:?}", secret_word_chars);

    let mut letters = String::new();
    let mut dash_letter: Vec<char> = String::new().chars().collect();
    let mut num_guesses_left = NUM_INCORRECT_GUESSES;

    let word_length = secret_word.len();
    let mut i = 0;
    while i < word_length {
        dash_letter.push('-');
        i = i + 1;
    }

    let mut j = 0;
    while j < word_length && num_guesses_left > 0 {
        println!("The word so far is {:?}", dash_letter);
        println!("You have guessed the following letters:{}", letters);
        println!("You have {} guesses left", num_guesses_left);
        print!("Please guess a letter: ");
        // Make sure the prompt from the previous line gets displayed:
        io::stdout().flush().expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");
        let ch = guess.chars().next().unwrap();
        letters.push(ch);
        if secret_word.contains(ch) {
            for x in 0..secret_word_chars.len() {
                if secret_word_chars[x] == ch {
                    dash_letter[x] = ch;
                    // break;
                }
            }
            j = j + 1;
            println!("");
        } else {
            println!("Sorry, that letter is not in the word\n");
            num_guesses_left = num_guesses_left - 1;
        }
    }

    if num_guesses_left > 0 {
        println!(
            "Congratulations you guessed the secret word: {}!
",
            secret_word
        );
    } else {
        println!(
            "Sorry, you ran out of guesses!
"
        );
    }

    // Your code here! :)
}
