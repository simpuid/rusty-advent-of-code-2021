use std::collections::VecDeque;
use std::fs;

fn main() {
    let inputs = read_input();
    println!(
        "number of times a depth measurement increases: {}",
        simple_depth_increase(&inputs)
    );
    println!(
        "number of times sums are larger than the previous sum: {}",
        sliding_window_depth_increase(3, &inputs)
    );
}

fn read_input() -> Vec<i32> {
    let mut result = Vec::new();
    if let Ok(string) = fs::read_to_string("input.txt") {
        for line in string.lines() {
            if let Ok(value) = line.parse() {
                result.push(value);
            }
        }
    }
    return result;
}

fn simple_depth_increase(input: &Vec<i32>) -> i32 {
    let mut previous = None;
    let mut counter = 0;
    for value in input {
        if let Some(previous) = previous {
            if value > previous {
                counter += 1;
            }
        }
        previous = Some(value)
    }
    return counter;
}

fn sliding_window_depth_increase(max_window_size: usize, input: &Vec<i32>) -> i32 {
    let mut previous = None;
    let mut queue = VecDeque::new();
    let mut sum = 0;
    let mut counter = 0;
    for value in input {
        queue.push_back(value);
        sum += value;
        while queue.len() > max_window_size {
            if let Some(front) = queue.pop_front() {
                sum -= front;
            }
        }
        if queue.len() == max_window_size {
            if let Some(previous) = previous {
                if sum > previous {
                    counter += 1;
                }
            }
            previous = Some(sum)
        }
    }
    return counter;
}
