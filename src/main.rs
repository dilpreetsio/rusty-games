struct TicTacToe {
    board: Vec<Vec<char>>,
    current_player: char,
}

impl TicTacToe {
    fn new() -> TicTacToe {
        TicTacToe {
            board: vec![vec![' '; 3]; 3],
            current_player: 'O',
        }
    }

    fn print_board(&mut self) {
        println!("Current player: {}", self.current_player);
        for row in &self.board {
            for &cell in row {
                print!("|");
                print!("{}", cell);
            }
            print!("| \n");
            println!("-------");
        }
    }

    fn horizontal_win(&mut self, row: usize) -> bool {
        self.board[row][0] != ' '
            && self.board[row][0] == self.board[row][1]
            && self.board[row][1] == self.board[row][2]
    }

    fn vertical_win(&mut self, col: usize) -> bool {
        self.board[0][col] != ' '
            && self.board[0][col] == self.board[1][col]
            && self.board[1][col] == self.board[2][col]
    }

    fn right_diagonal_win(&mut self) -> bool {
        self.board[0][0] != ' '
            && self.board[0][0] == self.board[1][1]
            && self.board[1][1] == self.board[2][2]
    }

    fn left_diagonal_win(&mut self) -> bool {
        self.board[0][2] != ' '
            && self.board[0][2] == self.board[1][1]
            && self.board[1][1] == self.board[2][0]
    }

    fn check_win(&mut self) -> bool {
        self.right_diagonal_win()
            || self.left_diagonal_win()
            || self.horizontal_win(0)
            || self.horizontal_win(1)
            || self.horizontal_win(2)
            || self.vertical_win(0)
            || self.vertical_win(1)
            || self.vertical_win(2)
    }

    fn make_moke(&mut self, row: usize, col: usize) {
        self.board[row][col] = self.current_player;
    }

    pub fn play(&mut self) {
        self.print_board();
        // let mut game_loop = true;
        let mut turns = 0;
        loop {
            if turns == 9 {
                println!("Game Draw");
                break;
            }
            let mut choice = String::new();

            println!("Player {}, enter your move (1-9):", self.current_player);

            std::io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");

            let input: i32 = choice.trim().to_string().parse().unwrap();
            println!("You chose outside match {}", input);
            match input {
                1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 => {
                    let row = (input - 1) / 3 as i32;
                    let col = (input - 1) % 3;

                    if self.board[row as usize][col as usize] != ' ' {
                        self.print_board();
                        println!("Invalid move. Please try again.");
                        continue;
                    }

                    self.make_moke(row as usize, col as usize);
                    turns += 1;
                    self.print_board();

                    if self.check_win() {
                        println!("Yay! You won, Player {}!", self.current_player);
                        break;
                    }
                    // register the move and print board
                    //
                    self.current_player = if self.current_player == 'X' { 'O' } else { 'X' };
                }
                _ => {
                    println!("Invalid input. Please try again.");
                    continue;
                }
            }
        }
    }
}

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
            let mut game = TicTacToe::new();
            game.play()
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
}
