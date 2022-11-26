use console::Term;
use core::time;
use std::fmt::Debug;
use std::sync::{Arc, Mutex, MutexGuard};
use std::{
    io::Read,
    thread,
    time::{Duration, Instant},
};

struct TimingValue<'a> {
    name: &'a str,
    alert: &'a str,
    duration: Vec<Duration>,
    elapsed: Duration,
    start: Instant,
    done: u8,
}

impl TimingValue<'_> {
    fn new<'a>(name: &'a str, duration: Vec<u64>, alert: &'a str) -> TimingValue<'a> {
        let mut dvec: Vec<Duration> = Vec::new();
        for d in duration {
            dvec.push(Duration::from_secs(d));
        }
        TimingValue {
            name,
            alert,
            duration: dvec,
            elapsed: Duration::from_secs(0),
            start: Instant::now(),
            done: 0,
        }
    }
    fn start(&mut self) {
        self.start = Instant::now();
        self.done = self.duration.len() as u8;
    }
    fn tick(&mut self) {
        if self.done == 0 {
            return;
        }
        if self.start.elapsed() < self.duration[self.duration.len() - self.done as usize] {
            self.elapsed = self.start.elapsed();
        } else {
            self.elapsed = self.start.elapsed();
            self.done -= 1;
        }
    }
    fn draw(&self) -> String {
        if self.done == 0 {
            format!("{}", self.alert)
        } else {
            if self.duration.len() > 1 {
                format!(
                    "{}: {}\t({}/{})",
                    self.name,
                    (self.duration[self.duration.len() - 1] - self.elapsed).as_secs(),
                    self.done,
                    self.duration.len()
                )
            } else {
                format!(
                    "{}: {}",
                    self.name,
                    (self.duration[self.duration.len() - 1] - self.elapsed).as_secs()
                )
            }
        }
    }
}

fn main() {
    let char = Arc::new(Mutex::new('F'));
    let char2 = char.clone();
    let term = Term::stdout();
    let ts = match term.clear_screen() {
        Ok(_) => true,
        Err(_) => false,
    };
    let mut timers = [
        TimingValue::new("Roshan", vec![480, 660], "Rosh is up"),
        TimingValue::new("Glyph", vec![300], "No glyph!"),
        TimingValue::new("BB", vec![480], "No bb!"),
    ];
    thread::spawn(move || loop {
        let tr = Term::stdout();
        let c = tr.read_char().unwrap();
        let mut num = char2.lock().unwrap();
        *num = c;
        //thread::sleep(Duration::from_secs(1));
    });
    loop {
        if ts {
            term.clear_screen().unwrap();
        }
        let mut i = 1;
        for t in &mut timers {
            t.tick();
            term.write_line(&format!("{}) {}", i, t.draw())).unwrap();
            i += 1;
        }
        let mut c = char.lock().unwrap();
        let num = *c as usize - 48;
        if (1..9).contains(&num) {
            if num <= timers.len() {
                timers[num as usize - 1].start();
            }
        }
        *c = 'F';
        thread::sleep(Duration::from_secs(1));
    }
}
