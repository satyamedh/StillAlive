use std::{io, thread, time::Duration};
use std::cmp::max;
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
        "left1_pg1",
        "left1_pg2",
        "left1_pg3",
        "left1_pg4",
    ];

    for file in files_left {

        clear();

        let contents = fs::read_to_string(format!("src/{}.txt", file))
            .expect("NU SMTH BROKE!");

        let text = contents.split("\n");


        let timingsfile = fs::read_to_string(format!("src/{}.timings", file))
            .expect("NU SMTH BROKE!");

        let timings = timingsfile.split("\n");
        let timings = timings.collect::<Vec<&str>>();
        let mut linecount = 0;
        for line in text {

            let timingdata = timings[linecount].split(",").collect::<Vec<&str>>();
            let sec_per_line: f32 = timingdata[0].parse().unwrap();
            let sec_later: f32 = timingdata[1].parse().unwrap();

            let mut to_be_printed: String = String::new();
            to_be_printed = format!("{: <56}", line);

            let num_chars = max(1, to_be_printed.trim().chars().count()) as f32;
            let time_to_sleep = sec_per_line / (num_chars);

            print!("{}", "| ".yellow());
            for char in to_be_printed.chars(){
                print!("{}", (String::from(char)).yellow());
                io::stdout().flush().expect("Uhh FLUSH DIDNT DO GOOD");

                if char != ' ' { thread::sleep(Duration::from_secs_f32(time_to_sleep)); }
            }
            println!("{}", " |".yellow());
            thread::sleep(Duration::from_secs(sec_later as u64));

            linecount = linecount + 1;
        }
        // Sleep 1s
        thread::sleep(Duration::from_secs(1));

    }

}


fn audio(){
    thread::sleep(Duration::from_secs(7));
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