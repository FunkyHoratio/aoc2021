use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let filename = "input.txt";
    println!("In file {}", filename);
    let mut num_increases: i32 = 0;

    if let Ok(lines) = read_lines(filename) {
        let mut first_value: i32 = 0;
        let mut second_value: i32 = 0;
        let mut third_value: i32 = 0;
        let mut previous_sum: i32 = 0;

        for line in lines {
            let myline = line.unwrap();
            println!("The line is: {}", myline.trim());
            let value: i32 = myline.trim().parse().unwrap();
            if first_value == 0 {
                first_value = value;
            } else if second_value == 0 {
                second_value = value;
            } else if third_value == 0 {
                third_value = value;
                previous_sum = first_value + second_value + third_value;
            } else {
                // Shift down
                first_value = second_value;
                second_value = third_value;
                third_value = value;

                let new_sum = first_value + second_value + third_value;
                if new_sum > previous_sum {
                    num_increases += 1;
                }
                previous_sum = new_sum
            }
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