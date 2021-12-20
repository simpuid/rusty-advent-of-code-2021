use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let map = read_input();

    println!("total flashes: {}", total_flash(map.clone(), 100));
    println!("sync flash step: {}", sync_flash(map));
}

fn total_flash(mut map: HashMap<(i8, i8), u8>, steps: usize) -> usize {
    let mut flashes = 0;
    for _ in 0..steps {
        flashes += step(&mut map);
    }
    return flashes;
}

fn sync_flash(mut map: HashMap<(i8, i8), u8>) -> usize {
    let mut steps = 1;
    loop {
        let flash = step(&mut map);
        if flash == map.len() {
            return steps;
        }
        steps += 1;
    }
}

fn read_input() -> HashMap<(i8, i8), u8> {
    let mut result = HashMap::new();
    if let Ok(string) = fs::read_to_string("input.txt") {
        for (y, line) in string.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                result.insert((x as i8, y as i8), c as u8 - '0' as u8);
            }
        }
    }
    return result;
}

fn step(map: &mut HashMap<(i8, i8), u8>) -> usize {
    let mut flashed = HashSet::new();
    let mut stack = Vec::new();

    for ((x, y), value) in map.iter_mut() {
        *value += 1;
        if *value > 9 {
            flashed.insert((*x, *y));
            stack.push((*x, *y));
        }
    }

    while let Some((x, y)) = stack.pop() {
        let offsets = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
        ];
        for (x_off, y_off) in offsets {
            let xx = x + x_off;
            let yy = y + y_off;
            if let Some(value) = map.get_mut(&(xx, yy)) {
                *value += 1;
                if *value > 9 {
                    if !flashed.contains(&(xx, yy)) {
                        flashed.insert((xx, yy));
                        stack.push((xx, yy));
                    }
                }
            }
        }
    }
    let result = flashed.len();
    for (x, y) in flashed.iter() {
        if let Some(value) = map.get_mut(&(*x, *y)) {
            *value = 0;
        }
    }
    return result;
}
