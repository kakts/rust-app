use std::io;
use std::cmp::Ordering;
use std::num;

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}


fn main() {
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // 入力文字列を数値にパースして、Guess型のインスタンスにする
        let guess: Guess = match guess.trim().parse() {
            Ok(num) => {
                Guess::new(num)
            },
            Err(_) => {
                println!("Continue \n");
                continue;
            }
        };

        if guess.value < 10 {
            break;
        }
    }
}
