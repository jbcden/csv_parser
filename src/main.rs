use std::io::BufReader;
use std::io::BufRead;
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;
// use std::io::Read;
// use std::fs::File;
// use std::sync::mpsc::{Sender};
// use std::io::{Error};

const F_LEN: i32 = 1411070052;

fn main() {
    let count = Arc::new(Mutex::new(0));
    let (tx, rx) = mpsc::channel();

    let csv = match std::fs::File::open("../FINAL.csv") {
        Ok(f) => f,
        Err(_) => std::fs::File::open("../FINAL.csv").unwrap(),
    };
    let buf_read = Arc::new(Mutex::new(BufReader::new(&csv)));
    // let mut lines = buf_read.lines().by_ref();
    // let line_nums = lines.count();


    //for line in lines {
    //while reader.read_line(&mut buf).unwrap() > 0 {
    for x in 0..F_LEN {
        let mut reader = match buf_read.lock() {
            Ok(reader) => reader,
            Err(_) => continue,
        };
        let mut buf = String::new();
        reader.read_line(&mut buf);
        {
            let buf = buf.clone();
            let (count, tx) = (count.clone(), tx.clone());
            thread::spawn(move || {
            //     let length = match line {
            //         Ok(l) => parse_line(l),
            //         Err(_) => 0,
            //     };
                // println!("BUF: {}", buf);
                let length = parse_line(buf);
                // println!("LEN: {}", length);

                let count: i32 = match count.lock() {
                    Ok(c) => *c,
                    Err(_) => 0,
                };
                let answer = count + length;

                match tx.send(answer) {
                    Ok(_) => (),
                    Err(_) => println!("ERROR!!!!!"),
                };
                //buf.clear();
            });
        }
        if x % 1_000_000 == 0 {
            println!("X: {}", x);
        }
    }

    println!("PANDA TEST");
    for _ in 0..F_LEN {
        let res = match rx.recv() {
            Ok(n) => n,
            Err(_) => -2
        };
        println!("RES: {}", res);
    }

    // Total: 617_806_098
    // println!("Total: {}", count);
}

// fn process_line(line: Result<String, std::io::Error>, count: Arc<Mutex<i32>>, tx: &std::sync::mpsc::Sender<i32>) {


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
#[allow(dead_code)]
fn efficient_unsafe(line: Result<String, String>) -> i32 {
    let unwrapped_line = line.unwrap();
    unwrapped_line.split(',')
                  .nth(1)
                  .map(|s| s.replace("\"", ""))
                  .map(|s| s.parse::<i32>().unwrap())
                  .unwrap()
}
