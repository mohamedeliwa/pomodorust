use std::{
    env::args,
    io::{self, Write},
    println, thread,
    time::Duration,
};

#[derive(Debug)]
struct Pomodoro {
    session: u64,
    pause: u64,
}

fn main() {
    let (session, pause) = args_formatter(args().skip(1).collect());
    let pomodoro = Pomodoro { session, pause };

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
* extracts session and pause values from user args
*/
fn args_formatter(args: Vec<String>) -> (u64, u64) {
    if args.len() < 2 {
        return (1, 1);
    }
    (
        u64::from_str_radix(&args[0], 10).unwrap_or(1),
        u64::from_str_radix(&args[1], 10).unwrap_or(1),
    )
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
