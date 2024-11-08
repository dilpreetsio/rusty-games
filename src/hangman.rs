use crate::common;

struct Hangman {
    word: String,
    wrong_moves: i32,
    current_word: String,
    guessed_letters: Vec<char>,
}

impl Hangman {
    fn new(guess_word: &str) -> Hangman {
        Hangman {
            word: guess_word.to_string(),
            current_word: "_".repeat(guess_word.len()),
            wrong_moves: 0,
            guessed_letters: Vec::new(),
        }
    }

    fn guess_word(&mut self, guess: &char) {
        let mut correct_guess = false;
        for (index, character) in self.word.chars().enumerate() {
            if character == *guess {
                self.current_word
                    .replace_range(index..index + 1, &guess.to_string());

                correct_guess = true;
            }
        }

        if !correct_guess {
            self.wrong_moves += 1;
        }
    }

    fn check_win(&mut self) -> bool {
        self.wrong_moves < 7 && self.current_word.to_lowercase() == self.word.to_lowercase()
    }

    fn play(&mut self) {
        loop {
            common::clear_screen();

            let mut guess = String::new();
            println!("Current word: {}", self.current_word);
            println!(
                "Guess a letter, you have {} guesses left",
                7 - self.wrong_moves
            );
            std::io::stdin().read_line(&mut guess).unwrap();
            guess = guess.trim().to_string();

            if guess.len() != 1
                || !guess.chars().all(|c| c.is_alphabetic())
                || self
                    .guessed_letters
                    .contains(&guess.char_indices().nth(0).unwrap().1)
            {
                println!("Enter a valid letter");
                continue;
            }

            let input_character = guess.chars().nth(0).unwrap();
            self.guess_word(&input_character);
            self.guessed_letters.push(guess.chars().nth(0).unwrap());

            // end game check
            if self.wrong_moves >= 7 {
                println!("You lose! The word was {}", self.word);
                break;
            } else if self.check_win() {
                println!("You win! The word was {}", self.word);
                break;
            }
        }
    }
}

pub fn play() {
    let mut hangman = Hangman::new("Rustician");
    hangman.play();
}
