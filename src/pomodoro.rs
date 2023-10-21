// use console;
use indicatif;
use notify_rust::Notification;
use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

enum Action {
    Resume,
    Puase,
}
enum Period {
    Session,
    Pause,
}

pub struct Pomodoro {
    session: u64,
    pause: u64,
    next: Period,
    tx: Option<Sender<Action>>,
    rx: Option<Receiver<Action>>,
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
            tx: Some(tx),
            rx: Some(rx),
        }
    }
    /**
     * runs a specific period of minutes
     * prints passed time indicators to stdout
     */
    pub fn run(&mut self) {
        let seconds = match self.next {
            Period::Session => self.session * 60,
            Period::Pause => self.pause * 60,
        };

        let rx = self.rx.as_ref().unwrap();
        // let thread_join_handle = thread::spawn(move || {
        let bar = indicatif::ProgressBar::new(seconds);
        bar.set_style(indicatif::ProgressStyle::with_template("[{elapsed_precise}]").unwrap());
        for _ in 0..seconds {
            bar.inc(1);
            thread::sleep(Duration::from_secs(1));
        }
        bar.finish();
        // });

        // let _res = thread_join_handle.join();
        self.notify();
        self.next = match self.next {
            Period::Session => Period::Pause,
            Period::Pause => Period::Session,
        };
    }
    /**
     * pauses the timer
     */
    pub fn pause(&self) {
        // transmits a pause message
        self.tx.as_ref().unwrap().send(Action::Puase).unwrap();
    }
    /**
     * resumes the timer
     */
    pub fn resume(&self) {
        // transmits a resume message
        self.tx.as_ref().unwrap().send(Action::Resume).unwrap();
    }
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
