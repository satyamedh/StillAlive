use std::{thread, time::Duration};
use clearscreen;
use colored::Colorize;

fn main() {
    clearscreen::clear().expect("Somethin went wrong");
    println!("{}", "--------------------------------------------------------".yellow());
}
