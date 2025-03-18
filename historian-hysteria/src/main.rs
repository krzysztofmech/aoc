fn main() {
    let total_distance = match std::fs::read_to_string("src/input.txt") {
        Ok(file) => get_total_distance(&file),
        Err(error) => {
            eprintln!("Error: {}", error);
            return;
        }
    };

    println!("{total_distance}");
}

fn get_total_distance(file: &str) -> i32 {
    let pairs = build_pairs(file);

    let (left, right) = split_into_seperate_lists(pairs);

    let mut left_vec = left.clone();
    let mut right_vec = right.clone();

    left_vec.sort();
    right_vec.sort();

    let distances = count_distances(left_vec, right_vec);

    distances.iter().sum()
}

fn build_pairs(file: &str) -> Vec<Vec<&str>> {
    let pairs: Vec<Vec<&str>> = file
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter(|value| !value.is_empty())
                .collect()
        })
        .collect();

    pairs
}

fn split_into_seperate_lists(pairs: Vec<Vec<&str>>) -> (Vec<i32>, Vec<i32>) {
    let lists: (Vec<i32>, Vec<i32>) = pairs
        .iter()
        .map(|pair| {
            (
                pair[0].parse::<i32>().unwrap(),
                pair[1].parse::<i32>().unwrap(),
            )
        })
        .unzip();

    lists
}

fn count_distances(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let distances: Vec<i32> = left
        .iter().zip(right.iter())
        .map(|(left, right)| (left - right).abs())
        .collect();

    distances
}
