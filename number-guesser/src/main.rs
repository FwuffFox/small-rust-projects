use rand::Rng;
use std::{
    io::{stdin, stdout, Write},
    cmp::Ordering,
};
use colored::Colorize;

fn main() {
    loop {
        game_loop();
        print!("{}", "Play again? [y/n]: ".blue());
        flush_output();
        let mut answer = String::new();
        stdin().read_line(&mut answer).expect("Couldn't read answer");
        if !answer.starts_with('y') {
            break;
        }
    }
}

fn flush_output() {
    stdout().flush().expect("Couldn't flush");
}

fn game_loop() {
    let rand_num = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guess = String::new();

        print!("{}", "Enter your number: ".blue());
        flush_output();
        stdin().read_line(&mut guess).expect("Can't read input!");
        let guess: i32 = guess.trim().parse().expect("Not a number!");

        match guess.cmp(&rand_num) {
            Ordering::Less => println!("{}", "Your guess is too small".yellow()),
            Ordering::Greater => println!("{}", "Your guess is too big".yellow()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}