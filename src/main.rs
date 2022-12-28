use std::fs::File;
use std::io::{stdin, BufReader, BufRead, Error, ErrorKind, Result};
use rand::seq::SliceRandom;
use once_cell::sync::Lazy;

static WORDS: Lazy<Vec<String>> = Lazy::new(|| {Game::get_words().expect("Word compilation failed")});
struct Game {
    data: [[u8; 5]; 6],
    current_row: usize,
    secret_word: String
}

impl Game {

    fn clear() {
        print! ("\x1B[2J\x1B[1;1H");
    }

    fn get_words() -> Result<Vec<String>> {
        let f = match File::open("5let_words.txt") {
            Ok(file) => file,
            Err(e) => panic!("Error opening file {}", e)
        };
        let lines = BufReader::new(f)
            .lines()
            .collect::<Result<Vec<String>>>()?;

        Ok(lines)
    }

    fn get_random_5let_word() -> String {
        return WORDS.choose(&mut rand::thread_rng())
            .expect("Word choice failed")
            .to_string();
    }

    fn out(&self) {

        for row in self.data {
            for (i, c) in row.iter().enumerate() {
                if self.secret_word.as_bytes()[i] == *c {
                    print!("\x1b[32m{}\x1b[0m ", *c as char);
                } 
                else if self.secret_word.contains(*c as char){
                    print!("\x1b[33m{}\x1b[0m ", *c as char);
                }
                else {
                    print!("{} ", *c as char);
                }
            }
            println!();
        }
    }

    fn set_row(&mut self, guess: &String) -> Result<()> {
        
        if guess.len() != 5 {
            return Err(Error::new(ErrorKind::InvalidInput, format!("'{}' is of an invalid length {}", guess, guess.len())));
        }

        if !WORDS.contains(guess) {
            return Err(Error::new(ErrorKind::InvalidInput, format!("'{}' is not in the word list", guess)));
        }

        for (i, c) in guess.as_bytes().iter().enumerate() {
            self.data[self.current_row][i] = *c;
        }
        return Ok(());
    }
}


fn main() {

    Game::clear();

    let mut game = Game {
        data: [['_' as u8; 5]; 6],
        current_row: 0,
        secret_word: Game::get_random_5let_word()
    };


    while game.current_row < 6 {
        //print!("{}\n", game.secret_word);
        game.out();
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("String read failed");

        let buffer = match buffer.strip_suffix("\n") {
            Some(s) => s.to_string(),
            None => buffer
        };

        match game.set_row(&buffer) {
            Ok(()) => {

                if buffer == game.secret_word {
                    println!("You won");
                    return;
                }

                game.current_row+=1;
                Game::clear();

            }
            Err(e) => {
                Game::clear();
                println!("{e}");
            }
        }
    }
    println!("You lost, the word was {}", game.secret_word);




}