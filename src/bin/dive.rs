use std::fs;

fn main() {
    let commands = read_input();
    let (horizontal, depth) = calculate_position(&commands);
    println!(
        "non-aimed horizontal: {}, depth: {}, product: {}",
        horizontal,
        depth,
        horizontal * depth
    );

    let (horizontal, depth) = calculate_position_aimed(&commands);
    println!(
        "aimed horizontal: {}, depth: {}, product: {}",
        horizontal,
        depth,
        horizontal * depth
    );
}

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn read_input() -> Vec<Command> {
    let mut result = Vec::new();
    if let Ok(string) = fs::read_to_string("input.txt") {
        for line in string.lines() {
            let mut iter = line.split(' ');
            if let (Some(command_string), Some(command_value)) = (iter.next(), iter.next()) {
                if let Ok(command_value) = command_value.parse() {
                    match command_string {
                        "forward" => result.push(Command::Forward(command_value)),
                        "down" => result.push(Command::Down(command_value)),
                        "up" => result.push(Command::Up(command_value)),
                        _ => (),
                    };
                }
            }
        }
    }
    return result;
}

fn calculate_position(commands: &Vec<Command>) -> (i32, i32) {
    let mut horizontal = 0;
    let mut depth = 0;
    for command in commands {
        match command {
            Command::Forward(value) => horizontal += value,
            Command::Up(value) => depth -= value,
            Command::Down(value) => depth += value,
        }
    }
    return (horizontal, depth);
}

fn calculate_position_aimed(commands: &Vec<Command>) -> (i32, i32) {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for command in commands {
        match command {
            Command::Forward(value) => {
                horizontal += value;
                depth += aim * value;
            }
            Command::Up(value) => {
                aim -= value;
            }
            Command::Down(value) => {
                aim += value;
            }
        }
    }
    return (horizontal, depth);
}
