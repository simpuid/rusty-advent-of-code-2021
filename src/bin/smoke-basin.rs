use std::{
    collections::{HashSet, VecDeque},
    fs, usize,
};

fn main() {
    let input_map = read_input();

    let mut basin_sizes = Vec::new();
    let mut risk_sum = 0;
    for (y, row) in input_map.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            let mut low_point = true;
            let offsets = [(1, 0), (0, 1), (-1, 0), (0, -1)];
            for (x_offset, y_offset) in offsets {
                let xx = x as isize + x_offset;
                let yy = y as isize + y_offset;
                if let Some(adjacent_value) = browse_map(&input_map, xx, yy) {
                    if adjacent_value <= *value {
                        low_point = false;
                        break;
                    }
                }
            }
            if low_point {
                risk_sum += (*value + 1) as usize;
                basin_sizes.push(calculate_basin_size(&input_map, x as isize, y as isize));
            }
        }
    }
    println!("risk sum: {}", risk_sum);

    basin_sizes.sort();
    let mut size_product = 1;
    for (i, value) in basin_sizes.iter().rev().enumerate() {
        if i < 3 {
            size_product *= value;
        } else {
            break;
        }
    }
    println!("product of basin sizes: {}", size_product);
}

fn read_input() -> Vec<Vec<u8>> {
    let mut result = Vec::new();
    if let Ok(string) = fs::read_to_string("input.txt") {
        for line in string.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c as u8 - '0' as u8);
            }
            result.push(row);
        }
    }
    return result;
}

fn browse_map(map: &Vec<Vec<u8>>, x: isize, y: isize) -> Option<u8> {
    if x < 0 || y < 0 {
        return None;
    }
    if let Some(row) = map.get(y as usize) {
        return row.get(x as usize).cloned();
    }
    return None;
}

fn calculate_basin_size(map: &Vec<Vec<u8>>, point_x: isize, point_y: isize) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut size = 1;
    queue.push_back((point_x, point_y));
    visited.insert((point_x, point_y));
    while let Some((x, y)) = queue.pop_front() {
        if let Some(value) = browse_map(map, x, y) {
            let offsets = [(1, 0), (0, 1), (-1, 0), (0, -1)];
            for (x_offset, y_offset) in offsets {
                let xx = x + x_offset;
                let yy = y + y_offset;
                if let Some(map_value) = browse_map(map, xx, yy) {
                    if map_value > value {
                        if !visited.contains(&(xx, yy)) {
                            visited.insert((xx, yy));
                            queue.push_back((xx, yy));
                            if map_value != 9 {
                                size += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    return size;
}
