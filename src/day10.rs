use std::fs;
use std::cmp;

use itertools::Itertools;

pub fn run() {
  let input = load("inputs/day10/input.txt");
  let differences = count_joltage_differences(&input);

  println!("=== Day 10 ===");
  println!("1-jolt diffs * 3-jolt diffs: {}", differences.0 * differences.2);
}


fn load(filepath: &str) -> Vec<u32> {
  let contents = fs::read_to_string(filepath)
    .expect("Failed to open a file.");

  let contents = String::from(contents);

  contents
    .trim()
    .split_whitespace()
    .map(|number| number.parse::<u32>().unwrap())
    .sorted()
    .collect()
}

fn count_joltage_differences(input: &Vec<u32>) -> (u32, u32, u32) {
  (
    count_n_differences(&input, 1),
    count_n_differences(&input, 2),
    count_n_differences(&input, 3) + 1
  )
}

fn count_n_differences(input: &Vec<u32>, difference: u32) -> u32 {
  let mut sum = if input[0] == difference { 1 } else { 0 };
  for i in 0..input.len() - 1 {
    if input[i + 1] - input[i] == difference {
      sum += 1;
    }
  }

  sum
}

fn count_combinations(input: &Vec<u32>, target: u32) -> u64 {
  count_combinations_step(&input[..], target)
}

fn count_combinations_step(input: &[u32], target: u32) -> u64 {
  // println!("input {:?} target {}", input, target);

  if input[0] == target - 3 {
    println!("HIT!");
    return 1;
  }

  if input.len() == 1 {
    return 0;
  }

  let mut acc = 0;
  for i in 1..cmp::min(3, input.len() - 1) {
    acc += count_combinations_step(&input[i..], target);
  }
  acc
}

#[test]
fn test_small_example() {
  let input = load("inputs/day10/example_small.txt");
  println!("{:?}", &input);
  
  let expected_d1 = 7;
  let expected_d3 = 5;
  let differences = count_joltage_differences(&input);

  assert_eq!(expected_d1, differences.0);
  assert_eq!(expected_d3, differences.2);

  let target = input[input.len() - 1] + 3;
  let expected_combinations = 8;
  let combinations = count_combinations(&input, target);

  assert_eq!(expected_combinations, combinations);
}

#[test]
fn test_big_example() {
  let input = load("inputs/day10/example_big.txt");
  println!("{:?}", &input);
  
  let expected_d1 = 22;
  let expected_d3 = 10;
  let differences = count_joltage_differences(&input);

  assert_eq!(expected_d1, differences.0);
  assert_eq!(expected_d3, differences.2);

  // let target = input[input.len() - 1] + 3;
  // let expected_combinations = 19208;
  // let combinations = count_combinations(&input, target);

  // assert_eq!(expected_combinations, combinations);
}