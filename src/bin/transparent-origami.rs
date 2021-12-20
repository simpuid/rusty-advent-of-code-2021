use std::{collections::HashSet, fs};

fn main() {
    let (mut points, folds) = read_input();
    println!("total points: {}", points.len());
    for fold in folds {
        match &fold {
            Fold::X(x_pos) => points = fold_x(points, *x_pos),
            Fold::Y(y_pos) => points = fold_y(points, *y_pos),
        };
        println!("fold {:?}, total points: {}", fold, points.len());
    }
    plot_points(&points);
}

fn read_input() -> (HashSet<(isize, isize)>, Vec<Fold>) {
    let mut points = HashSet::new();
    let mut folds = Vec::new();
    if let Ok(string) = fs::read_to_string("input.txt") {
        for line in string.lines() {
            if line.starts_with("fold along") {
                if let Ok(value) = line.trim_start_matches("fold along x=").parse() {
                    folds.push(Fold::X(value));
                } else if let Ok(value) = line.trim_start_matches("fold along y=").parse() {
                    folds.push(Fold::Y(value));
                }
            } else {
                let mut iter = line.split(',');
                if let (Some(x), Some(y)) = (iter.next(), iter.next()) {
                    if let (Ok(x), Ok(y)) = (x.parse(), y.parse()) {
                        points.insert((x, y));
                    }
                }
            }
        }
    }
    return (points, folds);
}

fn fold_y(points: HashSet<(isize, isize)>, y_pos: isize) -> HashSet<(isize, isize)> {
    let mut result = HashSet::new();
    for (x, mut y) in points.into_iter() {
        if y >= y_pos {
            y = y_pos * 2 - y;
        }
        result.insert((x, y));
    }
    return result;
}

fn fold_x(points: HashSet<(isize, isize)>, x_pos: isize) -> HashSet<(isize, isize)> {
    let mut result = HashSet::new();
    for (mut x, y) in points.into_iter() {
        if x >= x_pos {
            x = x_pos * 2 - x;
        }
        result.insert((x, y));
    }
    return result;
}

fn plot_points(points: &HashSet<(isize, isize)>) {
    if let Some((first_x, first_y)) = points.iter().next() {
        let mut x_min = *first_x;
        let mut x_max = *first_x;
        let mut y_min = *first_y;
        let mut y_max = *first_y;
        for (x, y) in points.iter() {
            x_min = x_min.min(*x);
            x_max = x_max.max(*x);
            y_min = y_min.min(*y);
            y_max = y_max.max(*y);
        }
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                print!("{}", if points.contains(&(x, y)) { '#' } else { '.' });
            }
            println!();
        }
    }
}

#[derive(Debug)]
enum Fold {
    X(isize),
    Y(isize),
}
