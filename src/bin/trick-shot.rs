fn main() {
    if let Some(((xmin, xmax), (ymin, ymax))) = read_input() {
        let max_y_velocity = ymin.abs() - 1;
        println!("max height: {}", max_y_velocity * (max_y_velocity + 1) / 2);

        let mut counter = 0;
        for x in (((xmin * 2) as f64).sqrt() as isize)..=xmax {
            for y in -ymin.abs()..=ymin.abs() {
                if check(xmin, xmax, ymin, ymax, x, y) {
                    counter += 1;
                }
            }
        }

        println!("total velocities: {}", counter);
    }
}

fn check(
    xmin: isize,
    xmax: isize,
    ymin: isize,
    ymax: isize,
    mut xvel: isize,
    mut yvel: isize,
) -> bool {
    let mut x = 0;
    let mut y = 0;
    while y >= ymin {
        if x >= xmin && x <= xmax && y >= ymin && y <= ymax {
            return true;
        }
        x += xvel;
        y += yvel;
        yvel -= 1;
        if xvel > 0 {
            xvel -= 1;
        } else if xvel < 0 {
            xvel += 1;
        }
    }
    return false;
}

fn read_input() -> Option<((isize, isize), (isize, isize))> {
    if let Ok(string) = std::fs::read_to_string("input.txt") {
        let mut iter = string
            .trim()
            .trim_start_matches("target area: x=")
            .split(", y=");
        if let (Some(xx), Some(yy)) = (iter.next(), iter.next()) {
            let mut xx = xx.split("..");
            let mut yy = yy.split("..");
            if let (Ok(xmin), Ok(xmax), Ok(ymin), Ok(ymax)) = (
                xx.next().unwrap_or_default().parse(),
                xx.next().unwrap_or_default().parse(),
                yy.next().unwrap_or_default().parse(),
                yy.next().unwrap_or_default().parse(),
            ) {
                return Some(((xmin, xmax), (ymin, ymax)));
            }
        }
    }
    return None;
}
