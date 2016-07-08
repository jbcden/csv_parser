use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let mut csv = std::fs::File::open("../FINAL.csv").unwrap();
    let mut buf_read = BufReader::new(&csv);

    let mut count: i32 = 0;
    for line in buf_read.lines() {
        // efficient_unsafe(line);
        let length = match line {
            Ok(l) => parse_line(l),
            Err(_) => 0,
        };
        count += length;
    }
    // Total: 617_806_098
    println!("Total: {}", count);
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

fn efficient_unsafe(line: Result<String, String>) -> i32 {
    let unwrapped_line = line.unwrap();
    unwrapped_line.split(',')
                  .nth(1)
                  .map(|s| s.replace("\"", ""))
                  .map(|s| s.parse::<i32>().unwrap())
                  .unwrap()
}
