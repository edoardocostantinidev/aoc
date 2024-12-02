use std::{collections::HashMap, str};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let (mut left, mut right) = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| split_line(&l))
        .fold((vec![], vec![]), |mut lists, pair| {
            lists.0.push(pair.0);
            lists.1.push(pair.1);
            lists
        });

    left.sort();
    right.sort();

    let distance: u64 = left
        .iter()
        .zip(right.iter())
        .fold(0, |distance, (left_min, right_min)| {
            distance + (left_min.abs_diff(right_min.to_owned()))
        });

    Some(distance)
}

fn split_line(line: &str) -> (u64, u64) {
    let space_index = line.find(|c: char| !c.is_digit(10)).expect("wrong_format");
    let (left, right) = line.split_at(space_index);
    let (trimmed_left, trimmed_right) = (left.trim(), right.trim());
    (
        trimmed_left.parse().unwrap(),
        trimmed_right.parse().unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (left, right) = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| split_line(&l))
        .fold((vec![], vec![]), |mut lists, pair| {
            lists.0.push(pair.0);
            lists.1.push(pair.1);
            lists
        });

    let mut set = HashMap::<u64, u64>::new();
    right.iter().for_each(|n| {
        if set.contains_key(n) {
            if let Some(current) = set.get_mut(n) {
                *current = *current + 1;
            }
        } else {
            set.insert(*n, 1);
        }
    });

    let similarity = left.into_iter().fold(0, |acc, n| {
        if let Some(occurrences) = set.get(&n) {
            acc + (n * (*occurrences))
        } else {
            acc
        }
    });

    Some(similarity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
