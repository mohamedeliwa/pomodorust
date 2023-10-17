use clap::Parser;
// use console;
use dialoguer;
use indicatif;
use notify_rust::{Hint, Notification};
use std::{thread, time::Duration};

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

    let confirmation = dialoguer::Confirm::new()
        .with_prompt("Start a session?")
        .interact()
        .unwrap();
    if !confirmation {
        return;
    }
    runner(pomodoro.session);
    Notification::new()
        .summary("Pomodoro")
        .body("Session has ended!")
        .hint(Hint::SoundName(String::from("alarm-clock-elapsed")))
        .show()
        .expect("showing notification error!");
    let confirmation = dialoguer::Confirm::new()
        .with_prompt("Start a break?")
        .interact()
        .unwrap();
    if !confirmation {
        return;
    }
    runner(pomodoro.pause);
    Notification::new()
        .summary("Pomodoro")
        .body("Break has ended!")
        .hint(Hint::SoundName(String::from("alarm-clock-elapsed")))
        .show()
        .expect("showing notification error!");
}

/**
* runs a specific period of minutes
* prints passed time indicators to stdout
*/
fn runner(minutes: u64) {
    let seconds = minutes * 60;

    let thread_join_handle = thread::spawn(move || {
        let bar = indicatif::ProgressBar::new(seconds);
        bar.set_style(indicatif::ProgressStyle::with_template("[{elapsed_precise}]").unwrap());
        for _ in 0..seconds {
            bar.inc(1);
            thread::sleep(Duration::from_secs(1));
        }
        bar.finish();
    });
    let _res = thread_join_handle.join();
}
