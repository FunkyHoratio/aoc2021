use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let filename = "input.txt";
    println!("In file {}", filename);
    let mut num_increases: i32 = 0;

    if let Ok(lines) = read_lines(filename) {
        let mut previous_value: i32 = 0;

        for line in lines {
            let myline = line.unwrap();
            println!("The line is: {}", myline.trim());
            let value: i32 = myline.trim().parse().unwrap();

            if previous_value != 0 && value > previous_value {
                num_increases += 1;
            }
            previous_value = value;
        }    
    }

    println!("Number of increases: {}", num_increases);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}