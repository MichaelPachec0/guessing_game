extern crate core;

use std:: {
    io,
    cmp::Ordering
};
use std::num::ParseIntError;
use rand::prelude::*;

fn main() {
    // Grab a random number
    // TODO: set limits
    // let mut number:i64;
    // loop{
    //     number = initialize();
    //     if !game_logic(number){
    //         break;
    //     }
    // }
    let mut number:i64 = initialize();
    while game_logic(number) {
        number = initialize()
    }
}
fn game_logic(number: i64) -> bool {
    let mut guesses :i32 = 0;
    let mut state: bool;
    let mut guess:i64;
    loop{

        // equivalent of a formatted string in python, insert guess into the string

        // switch equivalent with comparison syntactic sugar, this is actually pretty nice
        guess = guess_input();
        if guess.eq(&i64::MAX) {
            return false;
        }
        println!("You guessed: {guess}");
        state = match_number(number, guess);
        guesses += 1;
        if !state || !guess_again() {
            break;
        }

    }
    println!("You guessed the number in {guesses} tries");
    play_again()
}
fn match_number(number: i64, guess: i64) -> bool {
    match guess.cmp(&number) {
        Ordering::Less=> {
            println!("You guessed lower than the number"); },
        Ordering::Greater=> { println!("You guessed higher than the number") },
        Ordering::Equal=> {
            println!("You guessed the number!");
            return false;
        }
    }
    true
}


fn play_again() -> bool {
    let mut answer:String;
    println!("Do you wish to play again?");
    loop {
        answer = format_input();
        if !answer.is_empty() &&
            answer.len().eq(&(1)) {
            return match answer.as_str() {
                "y" => true,
                "n" => false,
                _ => continue
            }
        }
    }
}

fn format_input() -> String {
    let mut ret:String = String::new();
    ret.clear();
    io::stdin()
        .read_line(&mut ret)
        .expect("Failed to read line");
    // String::from(ret.trim())
    ret.trim().to_string()
}

// leave all the mutable types in here, so that we can work with immutable data
fn guess_input() -> i64{
    // ret will be a type of result
    // let mut ret:Result<i64,ParseIntError>;

    println!("Guess the number!");
    println!("Please input your guess or q to quit the game");

    loop {
        // from the io crate use the stdin function
        // pipe into the read_line, which uses the address of the variable we created
        // guess.
        // If we fail, then output error message, and then loop again
        let guess:String = format_input();
        // skip parsing if we fail to read line
        if guess.is_empty(){
            continue
        }
        // try to parse as a i64
        let ret:Result<i64, ParseIntError> = guess.parse::<i64>();
        return match ret {
            Ok(int) => int,
            Err(_e) => {
                if guess.eq("q"){
                    return i64::MAX;
                }
                println!("ERR: {_e}, guess: {guess}");
                continue;
            },
        }
        // More idiomatic rust
        // if let Ok(int_ret) = ret {
        //     return int_ret;
        // } else if guess.as_str() == "q"{
        //     return i64::MAX;
        // } else if let Err(err) = ret {
        //     let _pr = err.to_string();
        //     println!("ERR: {_pr}")
        // } else {
        //     println!("I dont Know why I'm here")
        // }

        // Apparently this is not idiomatic for rust
        // if ret.is_ok() {
        //     // we checked that it is a number but that it also fits as a i64,
        //     // return the number
        //     return ret.unwrap();
        // } else if guess.as_str() == "q" {
        //     return i64::MAX;
        // } else {
        //     println!("NaN, please try again.")
        // }
    }
}

fn guess_again() -> bool {
    let mut answer:String;
    println!("Do you wish to try again?[y/n]");
    loop{
        answer = format_input();
        if !answer.is_empty() && answer.len() == 1{
            // fancy way of returning a match
            return match answer.as_str() {
                "y" => true,
                "n" => false,
                _ => continue
            }
        }
    }
}

// function that takes nothing, but returns a 64 bit int
fn initialize() -> i64 {
    let mut rng_gen:ThreadRng = thread_rng();
    let answer:i64 = rng_gen.gen();
    println!("{answer}");
    answer
}
