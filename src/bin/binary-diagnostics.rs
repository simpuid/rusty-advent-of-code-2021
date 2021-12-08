use std::fs;

fn main() {
    let inputs = read_input();
    let (gamma, epsilon) = calculate_rate(&inputs);
    let gamma = vec_to_int(&gamma);
    let epsilon = vec_to_int(&epsilon);

    println!(
        "gamma: {}, epsilon: {}, power: {}",
        gamma,
        epsilon,
        gamma * epsilon
    );

    let oxygen = {
        let oxygen = calculate_oxygen(&inputs);
        if let Some(oxygen) = oxygen {
            Some(vec_to_int(&oxygen))
        } else {
            None
        }
    };

    let co2 = {
        let co2 = calculate_co2(&inputs);
        if let Some(co2) = co2 {
            Some(vec_to_int(&co2))
        } else {
            None
        }
    };
    if let (Some(oxygen), Some(co2)) = (oxygen, co2) {
        println!(
            "oxygen: {}, co2: {}, life support: {}",
            oxygen,
            co2,
            oxygen * co2
        );
    } else {
        println!("failed computing oxygen and co2");
    }
}

fn read_input() -> Vec<Vec<bool>> {
    let mut result = Vec::new();
    if let Ok(string) = fs::read_to_string("input.txt") {
        for line in string.lines() {
            let mut bin = Vec::new();
            for c in line.chars() {
                if c == '0' {
                    bin.push(false);
                } else if c == '1' {
                    bin.push(true);
                }
            }
            result.push(bin);
        }
    }
    return result;
}

fn binary_count(input: &Vec<Vec<bool>>, position: usize) -> (usize, usize) {
    let mut zero = 0;
    let mut one = 0;
    for num in input {
        if let Some(value) = num.get(position) {
            if *value == false {
                zero += 1;
            } else {
                one += 1;
            }
        }
    }
    return (zero, one);
}

fn binary_size(input: &Vec<Vec<bool>>) -> usize {
    if let Some(first) = input.first() {
        return first.len();
    }
    return 0;
}

fn calculate_rate(input: &Vec<Vec<bool>>) -> (Vec<bool>, Vec<bool>) {
    let mut gamma = Vec::new();
    let mut epsilon = Vec::new();
    for i in 0..binary_size(input) {
        let (zero, one) = binary_count(input, i);
        gamma.push(one > zero);
        epsilon.push(zero > one);
    }
    return (gamma, epsilon);
}

fn vec_to_int(vec: &Vec<bool>) -> i32 {
    let mut value = 0;
    let mut mask = 1;
    for bit in vec.iter().rev() {
        if *bit {
            value = value | mask;
        }
        mask = mask << 1;
    }
    return value;
}

fn filter_list(mut input: Vec<Vec<bool>>, index: usize, check_value: bool) -> Vec<Vec<bool>> {
    let mut filtered_list = Vec::new();
    while let Some(bin) = input.pop() {
        if let Some(value) = bin.get(index) {
            if *value == check_value {
                filtered_list.push(bin);
            }
        }
    }
    return filtered_list;
}

fn calculate_oxygen(input: &Vec<Vec<bool>>) -> Option<Vec<bool>> {
    let mut list = input.clone();
    let mut index = 0;
    let max_index = binary_size(input);
    while list.len() > 1 && index < max_index {
        let (zero, one) = binary_count(&list, index);
        list = filter_list(list, index, one >= zero);
        index += 1
    }
    return list.pop();
}

fn calculate_co2(input: &Vec<Vec<bool>>) -> Option<Vec<bool>> {
    let mut list = input.clone();
    let mut index = 0;
    let max_index = binary_size(input);
    while list.len() > 1 && index < max_index {
        let (zero, one) = binary_count(&list, index);
        list = filter_list(list, index, zero > one);
        index += 1
    }
    return list.pop();
}
