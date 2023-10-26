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
    Exit,
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

    /**
     * creates a new progress and adds it to the struct
     */
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
        let bar = ProgressBar::new(length).with_prefix(name);
        bar.set_style(
            ProgressStyle::with_template(
                "\n{prefix:.yellow}: {percent:.cyan/blue}% is completed.\n{wide_bar}",
            )
            .unwrap(),
        );
        self.bar = Some(bar);
    }

    /**
     * removes a progress bar from the struct
     * leaving None in place
     * marking it as finished
     */
    fn remove_bar(&mut self) -> () {
        let bar = self
            .bar
            .as_ref()
            .expect("couldn't get ref for self.bar, before finishing it");
        bar.finish();
        self.bar = None;
    }

    /**
     * shows desktop notification when an interval is finished
     */
    fn notify(&self) -> () {
        let ended_interval = match self.next_interval {
            Interval::Session => "Pause",
            Interval::Pause => "Session",
        };
        let msg = format!("{ended_interval} has ended!");
        let mut notification = Notification::new();
        notification.summary("Pomodoro").body(&msg);

        // adding support for notifications' sound in non windows systems 
        #[cfg(not(target_os = "windows"))]
        {
            notification.sound_name("alarm-clock-elapsed");
        }
   
        // adding support to notifications sound in windows
        #[cfg(target_os = "windows")]
        {
            notification.sound_name("Default");
        }
        notification.show().expect("showing notification error!");
    }

    /**
     * runs the logic to progress the progress bar
     */
    pub fn run(&mut self) -> () {
        if self.bar.is_none() {
            self.create_bar();
        }

        let bar = self.bar.as_ref().expect("couldn't get ref for self.bar");
        let initial_position = bar.position();
        let length = bar.length().expect("bar doesn't have length!!!");

        // println!(
        //     "position = {}, length = {length}, finished = {}",
        //     bar.position(),
        //     bar.is_finished(),
        // );

        for _ in initial_position..length {
            match self.rx.try_recv() {
                Ok(received) => match received {
                    // calling pause and waiting till user sends another action
                    Actions::Puase => {
                        let next_action = self.pause();
                        match next_action {
                            // exiting if user sends exit action while the app is paused
                            Actions::Exit => {
                                self.remove_bar();
                                return;
                            }
                            _ => {}
                        }
                    }
                    // exiting
                    Actions::Exit => {
                        self.remove_bar();
                        return;
                    }
                    _ => {}
                },
                Err(_) => {}
            };

            thread::sleep(Duration::from_secs(1));
            bar.inc(1);
        }
        self.notify();
        self.remove_bar();
    }

    /**
     * it pauses the thread execution till it receieves another user action
     * then returns the received action to the caller
     */
    fn pause(&self) -> Actions {
        match self.rx.recv() {
            Ok(received) => return received,
            Err(_) => Actions::Exit,
        }
    }
}
