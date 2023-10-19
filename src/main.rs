use clap::Parser;
use dialoguer;

mod pomodoro;
use pomodoro::Pomodoro;

#[derive(Parser, Debug)]
struct Args {
    /// How many minutes in a session
    #[arg(short, long, default_value_t = 1)]
    session: u64,
    /// How many minutes in a break
    #[arg(short, long, default_value_t = 1)]
    pause: u64,
}

fn main() {
    let pomodoro = Args::parse();

    let mut pomodoro = Pomodoro::new(pomodoro.session, pomodoro.pause);

    let confirmation = dialoguer::Confirm::new()
        .with_prompt("Start a session?")
        .interact()
        .unwrap();
    if !confirmation {
        return;
    }
    pomodoro.run();
    pomodoro.notify("Session has ended!");
    let confirmation = dialoguer::Confirm::new()
        .with_prompt("Start a break?")
        .interact()
        .unwrap();
    if !confirmation {
        return;
    }
    pomodoro.run();
    pomodoro.notify("Break has ended!");
}
