use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use regex::Regex;

use structopt::StructOpt;

// Input args 
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let filename = args.path.into_os_string().into_string().unwrap();
    println!("In file {}", filename);

    let re = Regex::new(r"^(\w+)\s(\d+)$").unwrap();

    if let Ok(lines) = read_lines(filename) {
        let mut depth: i32 = 0;
        let mut forward: i32 = 0;
        let mut aim: i32 = 0;

        for line in lines {
            let myline = line.unwrap();
            println!("The line is: {}", myline.trim());


            // Parse line
            let matches = re.captures(&myline).unwrap();
            let direction = &matches[1];
            let value: i32 = (&matches[2]).parse().unwrap();
            
            println!("I read direction={}, value={}", direction, value);

            if direction == "forward" {
                forward += value;
                depth += (aim * value);
            } else if direction == "up" {
                aim -= value;
            } else if direction == "down" {
                aim += value;
            }
        }  
        println!("Depth: {}, Forward: {}, Aim: {}, Multiplied Result: {}", depth, forward, aim, (depth * forward));  
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}