use std::cmp;
use std::fs;

fn main() {
    let lines = read_input();
    let (width, height) = find_dimension(lines.as_slice());
    let mut map = vec![0; width * height];

    for line in &lines {
        if line.x1 == line.x2 || line.y1 == line.y2 {
            map_line(&mut map, width, &line);
        }
    }
    println!(
        "axis oriented common points: {}",
        count_points(map.as_slice())
    );

    for line in &lines {
        if line.x1 != line.x2 && line.y1 != line.y2 {
            map_line(&mut map, width, &line);
        }
    }

    println!("all common points: {}", count_points(map.as_slice()));
}

fn read_input() -> Vec<Line> {
    let mut results = Vec::new();
    if let Ok(string) = fs::read_to_string("input.txt") {
        for line in string.lines() {
            let mut iter = line.split(&[',', ' '][..]);
            let x1 = iter.next().unwrap_or_default().parse();
            let y1 = iter.next().unwrap_or_default().parse();
            iter.next();
            let x2 = iter.next().unwrap_or_default().parse();
            let y2 = iter.next().unwrap_or_default().parse();
            if let (Ok(x1), Ok(y1), Ok(x2), Ok(y2)) = (x1, y1, x2, y2) {
                results.push(Line { x1, y1, x2, y2 });
            }
        }
    }
    return results;
}

fn find_dimension(lines: &[Line]) -> (usize, usize) {
    let mut x = 0;
    let mut y = 0;
    for line in lines {
        x = cmp::max(x, cmp::max(line.x1, line.x2));
        y = cmp::max(y, cmp::max(line.y1, line.y2));
    }
    return (x + 1, y + 1);
}

fn map_line(map: &mut [usize], width: usize, line: &Line) {
    let x1 = line.x1 as isize;
    let y1 = line.y1 as isize;
    let x2 = line.x2 as isize;
    let y2 = line.y2 as isize;
    let xsign = (x2 - x1).signum();
    let ysign = (y2 - y1).signum();
    let mut x = x1;
    let mut y = y1;
    let len = cmp::max((x2 - x1).abs(), (y2 - y1).abs());
    for _ in 0..=len {
        let xx = x as usize;
        let yy = y as usize;
        if let Some(value) = map.get_mut(xx + yy * width) {
            *value += 1;
        }
        x += xsign;
        y += ysign;
    }
}

fn count_points(map: &[usize]) -> usize {
    let mut counter = 0;
    for value in map {
        if *value > 1 {
            counter += 1;
        }
    }
    return counter;
}

struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}
