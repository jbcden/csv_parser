use std::io::BufReader;
use std::io::BufRead;
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;
use std::rc::Rc;
use std::cell::RefCell;

const F_LEN: i32 = 1411070052;

fn main() {
    let count = Arc::new(Mutex::new(0));
    let (tx, rx) = mpsc::channel();

    let csv = match std::fs::File::open("../FINAL.csv") {
        Ok(f) => f,
        Err(_) => {
            println!("WHY IS THIS HAPPENING AGAIN?!");
            std::fs::File::open("../FINAL.csv").unwrap()
        },
    };
    let buf_read = BufReader::new(&csv);
    let lines = Rc::new(RefCell::new(buf_read.lines()));

    let mut c = 0;
    let cloned = lines.clone();
    let mut reader = cloned.borrow_mut();
    while let Some(line) =  reader.next() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };

        {
            let (count, tx) = (count.clone(), tx.clone());
            thread::spawn(move || {
                let length = parse_line(line);

                let count: i32 = match count.lock() {
                    Ok(c) => *c,
                    Err(_) => 0,
                };
                let answer = count + length;

                match tx.send(answer) {
                    Ok(_) => (),
                    Err(_) => println!("ERROR!!!!!"),
                };
            });
        }
        c += 1;
        if c % 1_000_000 == 0 {
            println!("APPLES");
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
