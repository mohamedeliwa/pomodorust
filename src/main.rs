use clap::Parser;
// use console;
use dialoguer;
use indicatif;
use notify_rust::{Hint, Notification};
use std::{thread, time::Duration};

enum Period {
    Session,
    Pause,
}
struct Pomodoro {
    session: u64,
    pause: u64,
    next: Period,
}

impl Pomodoro {
    /**
     * runs a specific period of minutes
     * prints passed time indicators to stdout
     */
    fn run(&mut self) {
        let seconds = match self.next {
            Period::Session => self.session * 60,
            Period::Pause => self.pause * 60,
        };

        let thread_join_handle = thread::spawn(move || {
            let bar = indicatif::ProgressBar::new(seconds);
            bar.set_style(indicatif::ProgressStyle::with_template("[{elapsed_precise}]").unwrap());
            for _ in 0..seconds {
                bar.inc(1);
                thread::sleep(Duration::from_secs(1));
            }
            bar.finish();
        });
        self.next = match self.next {
            Period::Session => Period::Pause,
            Period::Pause => Period::Session,
        };
        let _res = thread_join_handle.join();
    }
    /***
     * notifies the user that a period is elapsed
     */
    fn notify(&self, msg: &str) {
        Notification::new()
            .summary("Pomodoro")
            .body(msg)
            .hint(Hint::SoundName(String::from("alarm-clock-elapsed")))
            .show()
            .expect("showing notification error!");
    }
}

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

    let mut pomodoro = Pomodoro {
        session: pomodoro.session,
        pause: pomodoro.pause,
        next: Period::Session,
    };

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
