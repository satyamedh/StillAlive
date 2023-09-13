use std::{thread, time::Duration};
use clearscreen;
use colored::{ColoredString, Colorize};
use std::env;
use std::fmt::format;
use std::fs;

fn main() {
    let box_vert_line: ColoredString = String::from("------------------------------------------------------------").yellow();

    let files_left = [
        "left1_pg1.txt",
        "left1_pg2.txt",
        "left1_pg3.txt",
        "left1_pg4.txt",
    ];

    for file in files_left {

        clearscreen::clear().expect("Loop issues...?");

        println!("{}", box_vert_line);
        let contents = fs::read_to_string(format!("src/{}", file))
            .expect("NU SMTH BROKE!");

        let text = contents.split("\n");

        for line in text {
            let mut to_be_printed: String = String::new();
            to_be_printed = format!("| {: <56} |", line);
            println!("{}", to_be_printed.yellow());
        }

        println!("{}", box_vert_line);

        // Sleep 2s
        thread::sleep(Duration::from_secs(2));

    }

}
