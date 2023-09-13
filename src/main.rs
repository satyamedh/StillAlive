use std::{thread, time::Duration};
use clearscreen;

fn main() {
    println!("Hello, world! This will go in 2 ");
    thread::sleep(Duration::from_millis(4000));
    clearscreen::clear().expect("Somethin went wrong");
    println!("HI! YOU SAW NOTHIN");
}
