use std::collections::HashMap;

#[derive(Debug)]
struct Results {
    distances: i32,
    similarity_score: i32,
}

fn main() {
    let results: Results = match std::fs::read_to_string("src/input.txt") {
        Ok(file) => get_results(&file),
        Err(error) => {
            eprintln!("Error: {}", error);
            return;
        }
    };

    println!("{:?}", results);
}

fn get_results(file: &str) -> Results {
    let pairs = build_pairs(file);

    let (left, right) = split_into_seperate_lists(pairs);

    let mut left_vec = left.clone();
    let mut right_vec = right.clone();

    left_vec.sort();
    right_vec.sort();

    let distances = count_distances(&left_vec, &right_vec);
    let similarity_score = count_similarities(&left, &right);

    Results {
        distances: distances.iter().sum(),
        similarity_score,
    }
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

fn count_distances(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
    let distances: Vec<i32> = left
        .iter()
        .zip(right.iter())
        .map(|(left, right)| (left - right).abs())
        .collect();

    distances
}

fn count_similarities(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let left_duplicates: HashMap<i32, i32> = build_duplicates_map(left);
    let right_duplicates: HashMap<i32, i32> = build_duplicates_map(right);

    let mut result = 0;

    for left_pair in left_duplicates.iter() {
        let (left_num, left_occurences) = left_pair;
        let left_pair_multiplied = left_num * left_occurences;

        if right_duplicates.contains_key(left_num) {
            let right_pair_value = match right_duplicates.get(left_num) {
                Some(&value) => value,
                _ => 0,
            };

            result += left_pair_multiplied * right_pair_value;
        }
    }

    result
}

fn build_duplicates_map(list: &Vec<i32>) -> HashMap<i32, i32> {
    let mut duplicates_map = HashMap::new();

    for num in list.iter() {
        if !duplicates_map.contains_key(num) {
            duplicates_map.insert(*num, 1);
        } else {
            let mut value = match duplicates_map.get(num) {
                Some(&value) => value,
                _ => 0,
            };

            value += 1;

            duplicates_map.insert(*num, value);
        }
    }

    duplicates_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_results() {
        let mock_input = "\
                          3 4
                          4 3
                          2 5
                          1 3
                          3 9
                          3 3";

        let results = get_results(mock_input);

        assert_eq!(results.distances, 11);
        assert_eq!(results.similarity_score, 31);
    }

    #[test]
    fn test_build_pairs() {
        let mock_input = "\
                          3 4
                          4 3";

        let pairs = build_pairs(mock_input);

        assert_eq!(pairs, vec![vec!["3", "4"], vec!["4", "3"]]);
    }

    #[test]
    fn test_split_into_seperate_lists() {
        let pairs = vec![vec!["3", "4"], vec!["4", "3"]];

        let (left, right) = split_into_seperate_lists(pairs);

        assert_eq!(left, vec![3, 4]);
        assert_eq!(right, vec![4, 3]);
    }

    #[test]
    fn test_count_distances() {
        let left = vec![1, 2, 3];
        let right = vec![3, 2, 1];

        let distances = count_distances(&left, &right);

        assert_eq!(distances, vec![2, 0, 2]);
    }

    #[test]
    fn test_count_similarities() {
        let left = vec![1, 2, 3, 3];
        let right = vec![3, 2, 1, 3];

        let similarity_score = count_similarities(&left, &right);

        assert_eq!(similarity_score, 15);
    }
}
