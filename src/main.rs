mod common;
mod hangman;
mod tic_tac_toe;

fn main() {
    println!("Welcome to Rust Games!");
    show_menu();
    let mut choice = String::new();
    std::io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim() {
        "1" => {
            println!("You chose Tic-Tac-Toe!");
            tic_tac_toe::play();
        }
        "2" => {
            println!("You chose Hangman!");
            hangman::play();
        }
        "Exit" => {
            println!("Thank you for playing!");
            std::process::exit(0);
        }
        _ => {
            println!("Invalid choice. Please try again.");
            show_menu();
        }
    }
}

fn show_menu() {
    println!("Choose a game:");
    println!("1. Tic-Tac-Toe");
    println!("2. Hangman");
}
