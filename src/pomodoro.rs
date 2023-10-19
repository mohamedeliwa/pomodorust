// use console;
use indicatif;
use notify_rust::Notification;
use std::{thread, time::Duration};

pub enum Period {
    Session,
    Pause,
}
pub struct Pomodoro {
    pub session: u64,
    pub pause: u64,
    pub next: Period,
}

impl Pomodoro {
    /**
     * runs a specific period of minutes
     * prints passed time indicators to stdout
     */
    pub fn run(&mut self) {
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
    pub fn notify(&self, msg: &str) {
        Notification::new()
            .summary("Pomodoro")
            .body(msg)
            .sound_name("alarm-clock-elapsed")
            .show()
            .expect("showing notification error!");
    }
}
