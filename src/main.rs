use std::{thread, time::Duration};
use clearscreen;
use colored::{ColoredString, Colorize};
use std::env;
use std::fs;

fn main() {
    let top_box_line: ColoredString = String::from("------------------------------------------------------------").yellow();

    clearscreen::clear().expect("Somethin went wrong");
    println!("{}", top_box_line);


    let contents = fs::read_to_string("src/left1_pg1.txt")
        .expect("NU SMTH BROKE!");

    let text = contents.split("\n");

    for line in text {
        let mut to_be_printed: String = String::new();
        to_be_printed = format!("| {: <56} |", line);
        println!("{}", to_be_printed.yellow());
    }
}
