use std::io::{self, prelude::*};


fn main() {
    let stdin = io::stdin();
    let mut before: [i64; 4] = [0,0,0,0];
    let mut instruction: (usize, usize, usize, usize) = (0,0,0,0);
    let mut after: [i64; 4] = [0,0,0,0];
    for (line_num, line) in stdin.lock().lines().enumerate() {
        let un_line = line.unwrap();
        match line_num % 3 {
            0 => {
                // set the before array to the array
                parse_array(&mut before, un_line)
            },
            1 => {
                // set this to the instruction
                instruction = parse_instruction(un_line);
            },
            2 => {
                // set the after array to the value
                parse_array(&mut after, un_line);
                // simulate all the instructions then compare the outputs to the
                // output array
                simulate_instructions(&mut before, &mut after);
            },
            _ => {panic!("Math no longer works");},
        }
    }
}

fn parse_array(array: &mut [i64; 4], line: String) {
    // parse the string
    // write the values into the array
}

fn parse_instruction(line: String) -> (usize, usize, usize, usize) {
    // parse the string into a tuple then return the tuple
    return (0,0,0,0);
}

fn simulate_instructions(before: &mut [i64; 4], after: &mut [i64; 4]) -> usize {

}
