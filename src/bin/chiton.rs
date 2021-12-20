use std::collections::{HashMap, VecDeque};

fn main() {
    let map = read_input();
    print_risk(&map);
    let map_5x = scale_5x(&map);
    print_risk(&map_5x);
}

fn read_input() -> HashMap<(isize, isize), usize> {
    let mut results = HashMap::new();
    if let Ok(string) = std::fs::read_to_string("input.txt") {
        for (y, line) in string.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                results.insert((x as isize, y as isize), c as usize - '0' as usize);
            }
        }
    }
    return results;
}

fn print_risk(map: &HashMap<(isize, isize), usize>) {
    if let (Some((x_start, y_start)), Some((x_end, y_end))) = (start(&map), end(&map)) {
        let risk_map = generate_risk_map(&map, x_start, y_start);
        if let Some(value) = risk_map.get(&(x_end, y_end)) {
            println!("risk: {:?}", value);
        }
    }
}

fn generate_risk_map(
    map: &HashMap<(isize, isize), usize>,
    start_x: isize,
    start_y: isize,
) -> HashMap<(isize, isize), usize> {
    let mut risk_map = HashMap::new();
    risk_map.insert((start_x, start_y), 0);
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));
    while let Some((x, y)) = queue.pop_front() {
        let offset = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        let risk = risk_map.get(&(x, y)).unwrap().clone();
        for (x_off, y_off) in offset {
            let xx = x + x_off;
            let yy = y + y_off;
            if let Some(point_risk) = map.get(&(xx, yy)) {
                let risk_sum = point_risk + risk;
                if let Some(risk) = risk_map.get_mut(&(xx, yy)) {
                    if risk_sum < *risk {
                        *risk = risk_sum;
                        queue.push_back((xx, yy));
                    }
                } else {
                    risk_map.insert((xx, yy), risk_sum);
                    queue.push_back((xx, yy));
                }
            }
        }
    }
    return risk_map;
}

fn start(map: &HashMap<(isize, isize), usize>) -> Option<(isize, isize)> {
    if let Some((mut xmin, mut ymin)) = map.keys().next() {
        for (x, y) in map.keys() {
            xmin = xmin.min(*x);
            ymin = ymin.min(*y);
        }
        return Some((xmin, ymin));
    }
    return None;
}

fn end(map: &HashMap<(isize, isize), usize>) -> Option<(isize, isize)> {
    if let Some((mut xmax, mut ymax)) = map.keys().next() {
        for (x, y) in map.keys() {
            xmax = xmax.max(*x);
            ymax = ymax.max(*y);
        }
        return Some((xmax, ymax));
    }
    return None;
}

fn scale_5x(map: &HashMap<(isize, isize), usize>) -> HashMap<(isize, isize), usize> {
    let mut result = HashMap::new();
    if let (Some((x_start, y_start)), Some((x_end, y_end))) = (start(&map), end(&map)) {
        let width = x_end - x_start + 1;
        let height = y_end - y_start + 1;
        for x in 0..5 {
            for y in 0..5 {
                for ((xx, yy), value) in map {
                    result.insert(
                        (x * width + *xx, y * height + *yy),
                        (*value + (x + y) as usize - 1) % 9 + 1,
                    );
                }
            }
        }
    }

    return result;
}
