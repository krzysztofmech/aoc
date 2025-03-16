use std::fs;

#[derive(Debug)]
struct Groups {
    locks: Vec<Vec<i32>>,
    keys: Vec<Vec<i32>>,
}

fn main() {
    let file = match fs::read_to_string("src/input.txt") {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error: {}", error);
            return;
        }
    };

    let schematics = build_schematics(&file);

    let grouped_schematics: Groups = group_schematics(schematics); 
    let unique_pairs = get_unique_pairs(grouped_schematics);
    println!("{unique_pairs}");
}

fn build_schematics(file: &str) -> Vec<String> {
    let schematics: Vec<String> = file
        .split("\n\n")
        .map(|chunk| chunk.to_string())
        .collect();

    schematics
}

fn group_schematics(schematics: Vec<String>) -> Groups {
    const LOCK_PATTERN: &str = "#####";

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for schematic in schematics.iter() {
        let row = match schematic.lines().next() {
            Some(line) => line,
            None => {
                eprintln!("Error: Schematic is empty or invalid.");
                continue;
            }
        };

        let converted_schematic = convert_schematic_to_pin_heights(schematic);

        if row == LOCK_PATTERN {
            locks.push(converted_schematic);
        } else {
            keys.push(converted_schematic);
        }
    }

    Groups { locks, keys }
}

fn convert_schematic_to_pin_heights(schematic: &str) -> Vec<i32> {
    let mut pin_heights = vec![0; 5];

    for row in schematic.lines() {
        let cleared_row: String = row.chars().filter(|c| !c.is_whitespace()).collect();

        for (index, column) in cleared_row.chars().enumerate() {
            if column == '#' {
                pin_heights[index] += 1;
            }
        }
    }

    pin_heights.iter_mut().for_each(|height| *height -= 1);

    pin_heights
}

fn get_unique_pairs(grouped_schematics: Groups) -> i32 {
    const MAX_PIN_HEIGHT: i32 = 5;

    let keys = &grouped_schematics.keys;
    let locks = &grouped_schematics.locks;

    let mut unique_pairs = 0;
    for key in keys.iter() {
        for lock in locks.iter() {
            if key
                .iter()
                .zip(lock.iter())
                .all(|(key_pin, lock_pin)| key_pin + lock_pin <= MAX_PIN_HEIGHT)
            {
                unique_pairs += 1;
            }
        }
    }

    unique_pairs
}
