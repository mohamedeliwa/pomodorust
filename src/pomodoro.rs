use std::{sync::mpsc::Receiver, thread, time::Duration};

use indicatif::{ProgressBar, ProgressStyle};
use notify_rust::Notification;

#[derive(Debug)]
enum Interval {
    Session,
    Pause,
}

pub enum Actions {
    Puase,
    Resume,
}

#[derive(Debug)]
pub struct Pomodoro {
    session_length: u64,
    pause_length: u64,
    next_interval: Interval,
    bar: Option<ProgressBar>,
    rx: Receiver<Actions>,
}

impl Pomodoro {
    pub fn new(session_length: u64, pause_length: u64, rx: Receiver<Actions>) -> Pomodoro {
        Pomodoro {
            session_length,
            pause_length,
            next_interval: Interval::Session,
            bar: None,
            rx,
        }
    }

    fn create_bar(&mut self) -> () {
        let (name, length) = match self.next_interval {
            Interval::Session => {
                self.next_interval = Interval::Pause;
                ("Session", self.session_length * 60)
            }
            Interval::Pause => {
                self.next_interval = Interval::Session;
                ("Pause", self.pause_length * 60)
            }
        };
        let bar = ProgressBar::new(length).with_prefix(format!("{name}: "));
        // bar.set_style(ProgressStyle::with_template("{prefix} [{elapsed_precise}]").unwrap());
        self.bar = Some(bar);
    }

    fn remove_bar(&mut self) -> () {
        let bar = self
            .bar
            .as_ref()
            .expect("couldn't get ref for self.bar, before finishing it");
        bar.finish();
        self.bar = None;
    }

    fn notify(&self) -> () {
        let ended_interval = match self.next_interval {
            Interval::Session => "Pause",
            Interval::Pause => "Session",
        };
        let msg = format!("{ended_interval} has ended!");
        Notification::new()
            .summary("Pomodoro")
            .body(&msg)
            .sound_name("alarm-clock-elapsed")
            .show()
            .expect("showing notification error!");
    }

    pub fn run(&mut self) -> () {
        if self.bar.is_none() {
            self.create_bar();
        }

        let bar = self.bar.as_ref().expect("couldn't get ref for self.bar");
        let initial_position = bar.position();
        let length = bar.length().expect("bar doesn't have length!!!");

        println!(
            "position = {}, length = {length}, finished = {}",
            bar.position(),
            bar.is_finished(),
        );

        for _ in initial_position..length {
            match self.rx.try_recv() {
                Ok(received) => match received {
                    Actions::Puase => {
                        self.pause();
                    }
                    Actions::Resume => {}
                },
                Err(_) => {}
            };

            thread::sleep(Duration::from_secs(1));
            bar.inc(1);
        }
        self.notify();
        self.remove_bar();
    }

    fn pause(&self) -> () {
        match self.rx.recv() {
            Ok(received) => match received {
                Actions::Resume => return,
                Actions::Puase => {}
            },
            Err(_) => {}
        }
    }
}
