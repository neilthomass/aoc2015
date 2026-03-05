use std::fs;
use std::collections::HashSet;


fn main() {
    let input_path = "inputs/day3.txt";
    let input = fs::read_to_string(input_path).expect("Failed to read file");

    let mut x = 0;
    let mut y = 0;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((x,y));


    for c in input.chars() {
        match c {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => {}
        }
        visited.insert((x,y));
    }

    println!("Santa visited {} unique houses the first year!", visited.len());

    x = 0;
    y = 0;
    let mut rx = 0;
    let mut ry = 0;
    visited.clear();
    visited.insert((0, 0));

    let chars: Vec<char> = input.chars().collect();

    for chunk in chars.chunks(2) {
        let first = chunk[0];

        match first {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => {}
        }
        visited.insert((x, y));

        if chunk.len() > 1 {
            let second = chunk[1];

            match second {
                '^' => ry += 1,
                'v' => ry -= 1,
                '>' => rx += 1,
                '<' => rx -= 1,
                _ => {}
            }
            visited.insert((rx, ry));
        }
    }

    println!("Santa and robo visited {} unique houses the second year!", visited.len());
}
