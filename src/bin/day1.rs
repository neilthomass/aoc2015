use std::fs;

fn main() {
    let input_path = "inputs/day1.txt";
    let input = fs::read_to_string(input_path).expect("Failed to read input file");

    let mut curr_floor = 0;
    let mut first_basement = 0;
    let mut basement_set = false;

    for (i, c) in input.chars().enumerate() {
        if c == '(' {
            curr_floor += 1;
        } else if c == ')' {
            curr_floor -= 1;
        }

        if !basement_set && curr_floor < 0 {
            first_basement = i + 1; // +1 because positions are 1-indexed
            basement_set = true;
        }
    }

    println!("Part 1 answer: {}", curr_floor);
    println!("Part 2 answer: {}", first_basement);
}
