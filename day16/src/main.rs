use std::io::{self, prelude::*};


fn main() {
    let stdin = io::stdin();
    let mut before: [i64; 4] = [0,0,0,0];
    let mut instruction: (usize, usize, usize, usize) = (0,0,0,0);
    let mut after: [i64; 4] = [0,0,0,0];
    let mut double_newline: bool = false;

    for (line_num, line) in stdin.lock().lines().enumerate() {
        // TODO: test
        println!("Line num: {}", line_num); 
        let line = line.unwrap();
        println!("Line: {}", line);
        println!("Line Val: {}", line == "");
        match (line_num % 4, line == "") {
            (0, false) => {
                double_newline = false;
                // set the before array to the array
                parse_array(&mut before, line)
            },
            (1, false) => {
                double_newline = false;
                // set this to the instruction
                instruction = parse_instruction(line);
            },
            (2, false) => {
                double_newline = false;
                // set the after array to the value
                parse_array(&mut after, line);
                // simulate all the instructions then compare the outputs to the
                // output array
                simulate_instructions(&mut before, &mut after);
            },
            (_, true) => {
                if double_newline == true {
                    output_results();
                } else {
                    double_newline = true;
                }
            },
            (_, _) => {panic!("Math no longer works");},
        }
    }
}

fn parse_array(array: &mut [i64; 4], line: String) {
    // parse the string
    // write the values into the array
    let chars: Vec<char> = line.chars().collect();
    array[0] = chars[9].to_digit(10).unwrap() as i64;
    array[1] = chars[12].to_digit(10).unwrap() as i64;
    array[2] = chars[15].to_digit(10).unwrap() as i64;
    array[3] = chars[18].to_digit(10).unwrap() as i64;
}

fn parse_instruction(line: String) -> (usize, usize, usize, usize) {
    // parse the string into a tuple then return the tuple
    let instruct: Vec<usize> = line.split_whitespace().map(|val| val.parse::<usize>().unwrap()).collect();
    return (instruct[0], instruct[1], instruct[2], instruct[3]);
}

fn simulate_instructions(before: &mut [i64; 4], after: &mut [i64; 4]) -> usize {
    return 0;
}

fn output_results() {
    unimplemented!()
}