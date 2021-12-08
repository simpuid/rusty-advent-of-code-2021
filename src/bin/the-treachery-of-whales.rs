use std::{cmp, fs};

fn main() {
    let mut input = read_input();
    input.sort();

    if let Some(median) = input.get(input.len() / 2) {
        println!("linear cost: {}", calculate_fuel(input.as_slice(), *median));
    }

    let first = *input.first().unwrap_or(&0);
    for values in &mut input {
        *values -= first;
    }

    let width = input.last().unwrap_or(&0) + 1;
    let mut crab_density = vec![0; width];
    for position in input {
        if let Some(value) = crab_density.get_mut(position) {
            *value += 1;
        }
    }

    let mut left_align_cost = vec![0; width];
    let mut rate = 0;
    let mut crabs = 0;
    let mut fuel = 0;
    for x in 0..width {
        fuel += rate;
        if let Some(value) = left_align_cost.get_mut(x) {
            *value = fuel;
        }
        if let Some(crab_count) = crab_density.get(x) {
            crabs += *crab_count;
        }
        rate += crabs;
    }

    let mut right_align_cost = vec![0; width];
    let mut rate = 0;
    let mut crabs = 0;
    let mut fuel = 0;
    for x in (0..width).rev() {
        fuel += rate;
        if let Some(value) = right_align_cost.get_mut(x) {
            *value = fuel;
        }
        if let Some(crab_count) = crab_density.get(x) {
            crabs += *crab_count;
        }
        rate += crabs;
    }

    let mut fuel = None;
    for x in 0..width {
        if let (Some(left), Some(right)) = (left_align_cost.get(x), right_align_cost.get(x)) {
            fuel = Some(cmp::min(fuel.unwrap_or(left + right), left + right));
        }
    }

    if let Some(fuel) = fuel {
        println!("quadratic cost: {}", fuel);
    }
}

fn read_input() -> Vec<usize> {
    let mut result = Vec::new();
    if let Ok(string) = fs::read_to_string("input.txt") {
        for num in string.split(&[' ', ',', '\n'][..]) {
            if let Ok(value) = num.parse() {
                result.push(value);
            }
        }
    }
    return result;
}

fn calculate_fuel(positions: &[usize], target: usize) -> usize {
    let mut fuel = 0;
    let mut min = 1000;
    let mut max = 0;
    for position in positions {
        fuel += (*position as isize - target as isize).abs() as usize;
        if *position < min {
            min = *position;
        }
        if *position > max {
            max = *position;
        }
    }
    return fuel;
}
