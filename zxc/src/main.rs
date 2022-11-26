// cargo run --package zxc
use std::io::{Read, Write};
use std::{io, thread, time::Duration};
fn main() {
    for i in (0..=1000).rev().step_by(7) {
        println!("{} - 7 = {}", i, i - 7);
        thread::sleep(Duration::from_millis(20));
    }
    print!("Я ГУЛЬ");
    //pause();
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}
