#![feature(old_path)]
#![feature(io)]

use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

fn main() {

    // get the arguments
    let arguments: Vec<String> = env::args().collect();

    // csv file path
    let path_string: String = arguments[1].clone();
    let csv_path: Path = Path::new(path_string);

    // let's open an csv file
    let mut csv_file = match File::open(&csv_path) {
        Err(why)     => panic!("error openning csv file: {}", why.description()),
        Ok(csv_file) => csv_file,
    };

    // reader
    let mut csv_reader = BufReader::new(csv_file);
    let mut line_buffer: String = String::new();

    loop {
        match csv_reader.read_line(&mut line_buffer) {
            Err(why) => panic!("error reading from file: {}", why.description()),
            Ok(lines_read) =>   { 
                                  println!("file has been read, lines read = {:?}", lines_read);
                                }
        };
        println!("{}", line_buffer);
    };
}
