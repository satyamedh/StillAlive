use std::{io, thread, time::Duration};
use clearscreen;
use colored::{ColoredString, Colorize};
use std::env;
use std::fmt::format;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Write};
use soloud::*;
use termion::{color, style};


fn main() {

    thread::spawn(|| {
        audio();
    });


    let files_left = [
        "left1_pg1.txt",
        "left1_pg2.txt",
        "left1_pg3.txt",
        "left1_pg4.txt",
    ];

    for file in files_left {

        clear();

        let contents = fs::read_to_string(format!("src/{}", file))
            .expect("NU SMTH BROKE!");

        let text = contents.split("\n");

        for line in text {
            let mut to_be_printed: String = String::new();
            to_be_printed = format!("{: <56}", line);
            let to_be_printed: ColoredString = to_be_printed.yellow();
            print!("{}", "| ".yellow());
            for letter in to_be_printed.chars(){
                print!("{}", letter.to_string().yellow());
                io::stdout().flush().unwrap();
                if letter != ' ' { thread::sleep(Duration::from_millis(50)); }
            }
            println!("{}", " |".yellow());
        }
        // Sleep 1s
        thread::sleep(Duration::from_secs(1));

    }

}


fn audio(){
    let sl = Soloud::default().unwrap();
    let mut wav = Wav::default();
    wav.load_mem(include_bytes!("Portal_still_alive.mp3")).unwrap();
    sl.play(&wav);
    while sl.voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn clear(){
    clearscreen::clear().expect("Loop issues...?");

    println!("{}", "------------------------------------------------------------".yellow());

    let mut i = 0;
    while i <= 30 {
        println!("{}", "|                                                          |".yellow());
        i = i + 1;
    }

    println!("{}", "------------------------------------------------------------".yellow());

    println!("{}", termion::cursor::Goto(2, 1));

}