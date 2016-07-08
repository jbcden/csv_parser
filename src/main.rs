use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::io::BufRead;
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender};
use std::io::{Error};

fn main() {
    let count = Arc::new(Mutex::new(0));
    let (tx, rx) = mpsc::channel();

    let mut csv = std::fs::File::open("../FINAL.csv").unwrap();
    let mut buf_read = BufReader::new(&csv);
    let lines = buf_read.lines();
    let line_nums = lines.count();

    println!("NUM LINES: {}", line_nums);
    for line in lines {

        process_line(line, count, &tx);
    }

    println!("PANDA TEST");
    for _ in 0..line_nums {
        let res = match rx.recv() {
            Ok(n) => n,
            Err(_) => -2
        };
        println!("{}", res);
    }

    // Total: 617_806_098
    // println!("Total: {}", count);
}

fn process_line(line: Result<String, std::io::Error>, count: Arc<Mutex<i32>>, tx: &std::sync::mpsc::Sender<i32>) {
    let (count, tx) = (count.clone(), tx.clone());
    thread::spawn( move || {
        let length = match line {
            Ok(l) => parse_line(l),
            Err(_) => 0,
        };

        let mut count = count.lock().unwrap();
        let answer = *count + length;

        tx.send(answer).unwrap();
    });
}


fn parse_line(line: String) -> i32 {
    let mut split = line.split(",");
    let parsed_string = split.nth(1)
                             .map_or("0".to_string(), |s| s.replace("\"", ""));
    let line_length = parsed_string.parse::<i32>();
     match line_length {
         Ok(len) => len,
         Err(_) => 0,
     }
}

// efficient_unsafe(line);
fn efficient_unsafe(line: Result<String, String>) -> i32 {
    let unwrapped_line = line.unwrap();
    unwrapped_line.split(',')
                  .nth(1)
                  .map(|s| s.replace("\"", ""))
                  .map(|s| s.parse::<i32>().unwrap())
                  .unwrap()
}
