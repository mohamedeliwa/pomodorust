use clap::Parser;
use std::{
    io::{self, Write},
    println, thread,
    time::Duration,
};

#[derive(Parser, Debug)]
struct Pomodoro {
    /// How many minutes in a session
    #[arg(short, long, default_value_t = 1)]
    session: u64,
    /// How many minutes in a break
    #[arg(short, long, default_value_t = 1)]
    pause: u64,
}

fn main() {
    let pomodoro = Pomodoro::parse();

    println!("Start session ?\n");
    let answer = bool_answer_formatter();
    if !answer {
        return;
    };
    println!("Session has started!");
    runner(pomodoro.session);
    println!("Session ended!");
    println!("Start break ?\n");
    let answer = bool_answer_formatter();
    if !answer {
        return;
    };
    println!("\nBreak has started!");
    runner(pomodoro.pause);
    println!("Break ended!")
}

/**
* formats user's answer for yes or no questions
*/
fn bool_answer_formatter() -> bool {
    let mut answer = String::new();
    let yes = String::from("y\n");
    io::stdin().read_line(&mut answer).unwrap();
    if answer == yes {
        true
    } else {
        false
    }
}

/**
* runs a specific period of minutes
* prints passed time indicators to stdout
*/
fn runner(minutes: u64) {
    let seconds = minutes * 60;
    let thread_join_handle = thread::spawn(move || {
        for i in 1..(seconds + 1) {
            thread::sleep(Duration::from_secs(1));
            if i % 60 == 0 {
                print!("{i}\n");
            } else if i % 10 == 0 {
                print!("{i}");
            } else if i % 2 == 0 {
                print!(".");
            }
            io::stdout().flush().unwrap();
        }
    });
    thread::sleep(Duration::from_secs(seconds));
    let _res = thread_join_handle.join();
}
