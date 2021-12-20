use std::collections::HashMap;

fn main() {
    let (base, rule) = read_input();
    print_values(&base, &rule, 10);
    print_values(&base, &rule, 40);
}

fn print_values(base: &Vec<char>, rule: &HashMap<(char, char), char>, steps: usize) {
    let freq = string_frequency(&base, steps, &rule);
    if let (Some(min), Some(max)) = (freq.values().min(), freq.values().max()) {
        println!(
            "steps: {}, most: {}, least: {}, difference: {}",
            steps,
            max,
            min,
            max - min
        );
    }
}

fn read_input() -> (Vec<char>, HashMap<(char, char), char>) {
    let mut result = HashMap::new();
    let mut base_string = Vec::new();
    if let Ok(string) = std::fs::read_to_string("input.txt") {
        let mut line_iter = string.lines();
        if let Some(string) = line_iter.next() {
            for c in string.chars() {
                base_string.push(c);
            }
        }
        for line in line_iter {
            let mut split_iter = line.split("->");
            if let (Some(lhs), Some(rhs)) = (split_iter.next(), split_iter.next()) {
                let mut lhs = lhs.trim().chars();
                let mut rhs = rhs.trim().chars();
                if let (Some(lhs_a), Some(lhs_b), Some(rhs)) = (lhs.next(), lhs.next(), rhs.next())
                {
                    result.insert((lhs_a, lhs_b), rhs);
                }
            }
        }
    }
    return (base_string, result);
}

fn merge(a: &HashMap<char, usize>, b: &HashMap<char, usize>) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    for (c, v) in a {
        *result.entry(*c).or_insert(0) += v;
    }
    for (c, v) in b {
        *result.entry(*c).or_insert(0) += v;
    }
    return result;
}

fn pair_frequency(
    cache: &mut HashMap<(char, char, usize), HashMap<char, usize>>,
    a: char,
    b: char,
    steps: usize,
    rule: &HashMap<(char, char), char>,
) -> HashMap<char, usize> {
    if let Some(value) = cache.get(&(a, b, steps)) {
        return value.clone();
    }
    if let Some(c) = rule.get(&(a, b)) {
        if steps > 0 {
            let a_map = pair_frequency(cache, a, *c, steps - 1, rule);
            let b_map = pair_frequency(cache, *c, b, steps - 1, rule);
            let merged = merge(&a_map, &b_map);
            cache.insert((a, b, steps), merged.clone());
            return merged;
        }
    }
    let map = HashMap::from([(a, 1)]);
    cache.insert((a, b, steps), map.clone());
    return map;
}

fn string_frequency(
    string: &Vec<char>,
    steps: usize,
    rule: &HashMap<(char, char), char>,
) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    let mut cache = HashMap::new();
    let mut iter = string.iter();
    let mut first = iter.next();
    while let (Some(a), Some(b)) = (first, iter.next()) {
        let map = pair_frequency(&mut cache, *a, *b, steps, rule);
        result = merge(&result, &map);
        first = Some(b);
    }
    if let Some(first) = first {
        if let Some(first) = result.get_mut(first) {
            *first += 1;
        }
    }

    return result;
}
