// use console;
use indicatif::{ProgressBar, ProgressStyle};
use notify_rust::Notification;
use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc,
    },
    thread,
    time::Duration,
};

// enum Action {
//     Resume,
//     Puase,
// }
enum Period {
    Session,
    Pause,
}

pub struct Pomodoro {
    session: u64,
    pause: u64,
    next: Period,
    bar: Option<ProgressBar>,
    tx: Arc<Sender<Action>>,
    rx: Arc<Receiver<Action>>,
}

impl Pomodoro {
    /**
     * creates a new instance of the Pomodoro struct and initializes its state
     */
    pub fn new(session: u64, pause: u64) -> Pomodoro {
        let (tx, rx) = mpsc::channel::<Action>();
        Pomodoro {
            session,
            pause,
            next: Period::Session,
            bar: None,
            tx: Arc::new(tx),
            rx: Arc::new(rx),
        }
    }
    /**
     * runs a specific period of minutes
     * prints passed time indicators to stdout
     */
    pub fn run(&mut self) {
        if self.bar.is_none() {
            match self.next {
                Period::Session => {
                    let bar = ProgressBar::new(self.session * 60);
                    bar.set_style(ProgressStyle::with_template("[{elapsed_precise}]").unwrap());
                    self.bar = Some(bar)
                }
                Period::Pause => {
                    let bar = ProgressBar::new(self.pause * 60);
                    bar.set_style(ProgressStyle::with_template("[{elapsed_precise}]").unwrap());
                    self.bar = Some(bar)
                }
            }
        };
        let bar = self.bar.as_ref().expect("couldn't get self.bar ref");
        let position = bar.position();
        let length = bar.length().expect("failed to get bar length!");
        for _ in position..length {
            bar.inc(1);
            thread::sleep(Duration::from_secs(1));
        }

        self.notify();
        bar.finish();
        self.next = match self.next {
            Period::Session => {
                self.bar.take();
                Period::Pause
            }
            Period::Pause => {
                self.bar.take();
                Period::Session
            }
        };
    }
    /**
     * pauses the timer
     */
    // pub fn pause(&self) {
    // transmits a pause message
    // self.tx.as_ref().unwrap().send(Action::Puase).unwrap();
    // }
    /**
     * resumes the timer
     */
    // pub fn resume(&self) {
    // transmits a resume message
    // self.tx.as_ref().unwrap().send(Action::Resume).unwrap();
    // }
    /***
     * notifies the user that a period is elapsed
     */
    fn notify(&self) {
        let msg = match self.next {
            Period::Session => "Session has ended!",
            Period::Pause => "Break has ended!",
        };
        Notification::new()
            .summary("Pomodoro")
            .body(msg)
            .sound_name("alarm-clock-elapsed")
            .show()
            .expect("showing notification error!");
    }
}
