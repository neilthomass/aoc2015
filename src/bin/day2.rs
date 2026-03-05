use std::fs;

fn main() {
    let input_path = "inputs/day2.txt";
    let input = fs::read_to_string(input_path).expect("Failed to read input file");
    let mut total = 0;
    let mut ribbon_total = 0;

    for line in input.lines() {
        let dims: Vec<i32> = line
            .split('x')
            .map(|s| s.parse::<i32>().expect("Invalid number"))
            .collect();

        if dims.len() != 3 {
            continue;
        }

        let l = dims[0];
        let w = dims[1];
        let h = dims[2];

        let side1 = l * w;
        let side2 = w * h;
        let side3 = h * l;
        let mut sorted = vec![l, w, h];
        sorted.sort(); 
        
        let area = 2 * side1 + 2 * side2 + 2 * side3;
        let extra = sorted[0];
        let perimeter = 2 * sorted[0] + 2 * sorted[1];
        let volume = l * w * h;

        ribbon_total += perimeter + volume;
        total += area + extra;
    }

    println!("Answer for Part 1: {}", total);
    println!("Answer for Part 2:: {}", ribbon_total);
}
