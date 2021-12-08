use std::{collections::HashMap, fs};

type Cache = HashMap<usize, usize>;

fn main() {
    let input = read_input();
    let mut cache = HashMap::new();
    for i in 0..=256 {
        println!("days:{}, fish: {}", i, count_fish(&mut cache, &input, i));
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

fn offspring_count(cache: &mut Cache, timer: usize, duration: usize) -> usize {
    if timer > duration + 8 {
        return 1;
    }
    let duration = duration + 8 - timer;
    if let Some(value) = cache.get(&duration) {
        return *value;
    } else {
        let mut fish_count = 1;
        let mut lifetime = duration as isize;
        lifetime -= 9;
        while lifetime >= 0 {
            fish_count += offspring_count(cache, 8, lifetime as usize);
            lifetime -= 7;
        }
        cache.insert(duration, fish_count);
        return fish_count;
    }
}

fn count_fish(cache: &mut Cache, timers: &[usize], duration: usize) -> usize {
    let mut fish_count = 0;
    for timer in timers {
        fish_count += offspring_count(cache, *timer, duration);
    }
    return fish_count;
}
