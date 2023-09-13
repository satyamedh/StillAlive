use std::{thread, time::Duration};
use clearscreen;
use colored::{ColoredString, Colorize};

fn main() {
    let top_box_line: ColoredString = String::from("------------------------------------------------------------").yellow();

    clearscreen::clear().expect("Somethin went wrong");
    println!("{}", top_box_line);
    let mut to_be_printed: String = String::new();
    to_be_printed = format!("| {: <56} |", "Should be yellow n padded");
    println!("{}", to_be_printed.yellow());
}
