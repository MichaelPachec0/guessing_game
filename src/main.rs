extern crate core;

use std::{
    io,
    cmp::Ordering,
    num::ParseIntError,
    env,
};
use rand::prelude::*;

fn main() {
    // Grab a random number
    // let mut number:i64;
    // loop{
    //     number = initialize();
    //     if !game_logic(number){
    //         break;
    //     }
    // }
    // TODO: let the user set their limits
    let args: Vec<String> = env::Args::collect(env::args());
    if args.len() < 2 {
        println!("WE GOT ARGS!");
    }
    let mut number: i64 = initialize(None, None, None);
    while game_logic(&number) {
        number = initialize(None, None, None);
    }
}

fn game_logic(number: &i64) -> bool {
    let mut guesses: i32 = 0;
    let mut state: bool;
    let mut guess: i64;
    loop {

        // equivalent of a formatted string in python, insert guess into the string

        // switch equivalent with comparison syntactic sugar, this is actually pretty nice
        guess = guess_input();
        if guess.eq(&i64::MAX) {
            return false;
        }
        println!("You guessed: {guess}");
        state = match_number(number, guess);
        guesses += 1;
        if !state {
            break;
        }
    }
    println!("You guessed the number in {guesses} tries");
    play_again()
}

fn match_number(number: &i64, guess: i64) -> bool {
    match guess.cmp(number) {
        Ordering::Less => {
            println!("You guessed lower than the number");
        }
        Ordering::Greater => { println!("You guessed higher than the number") }
        Ordering::Equal => {
            println!("You guessed the number!");
            return false;
        }
    }
    true
}


fn play_again() -> bool {
    let mut answer: String;
    println!("Do you wish to play again?");
    loop {
        answer = format_input();
        if !answer.is_empty() &&
            answer.len().eq(&(1)) {
            return match answer.as_str() {
                "y" => true,
                "n" => false,
                _ => continue
            };
        }
    }
}

fn format_input() -> String {
    // from the io crate use the stdin function
    // pipe into the read_line, which uses the address of the variable we created
    // guess.

    let mut ret: String = String::new();
    io::stdin()
        .read_line(&mut ret)
        .expect("Failed to read line");
    // If we fail, then output error message, and then loop again

    // Either or work,
    // String::from(ret.trim())
    ret.trim().to_string()
}

// leave all the mutable types in here, so that we can work with immutable data
fn guess_input() -> i64 {
    // ret will be a type of result
    // let mut ret:Result<i64,ParseIntError>;

    println!("Guess the number!");
    println!("Please input your guess or q to quit the game");

    loop {

        let guess: String = format_input();
        // skip parsing if we fail to read line
        if guess.is_empty() {
            continue;
        }
        // try to parse as a i64
        let ret: Result<i64, ParseIntError> = guess.parse::<i64>();
        match ret {
            Ok(int) => {
                return int;
            },
            Err(_e) => {
                if guess.eq("q") {
                    return i64::MAX;
                }
                println!("ERR: {_e}, guess: {guess}");
                continue;
            }
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

// fn guess_again() -> bool {
//     let mut answer:String;
//     println!("Do you wish to try again?[y/n]");
//     loop{
//         answer = format_input();
//         if !answer.is_empty() && answer.len() == 1{
//             // fancy way of returning a match
//             return match answer.as_str() {
//                 "y" => true,
//                 "n" => false,
//                 _ => continue
//             }
//         }
//     }
// }


fn initialize(_min: Option<i64>, _max: Option<i64>, _debug: Option<bool>) -> i64 {
    // TODO: Let user set limits
    // clippy at its most opinionated doesnt like this, i prefer this since the next match would
    //  would follow the same logic.
    //     let max = match _max {
    //         Some(tmp) => tmp,
    //         None => 100,
    //     };
    let max: i64 = _max.unwrap_or(100);
    let min: i64 = match _min {
        Some(_tmp) if _tmp < max => _tmp,
        _ => 0,
    };

    let debug: bool = _debug.unwrap_or(false);
    let mut rng_gen: ThreadRng = thread_rng();
    let answer: i64 = rng_gen.gen_range(min..max);
    if debug {
        println!("{answer}");
    }
    answer
}
