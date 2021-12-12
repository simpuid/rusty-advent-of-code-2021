use std::fs;
type Segment = [bool; 7];

fn main() {
    let input_list = read_input();

    let mut unique_counter = 0;
    for (_, output) in &input_list {
        for segment in output {
            let len = count_segment(segment, true);
            if len == 2 || len == 3 || len == 4 || len == 7 {
                unique_counter += 1;
            }
        }
    }
    println!("unique numbers: {}", unique_counter);

    let mut sum = 0;
    for (inputs, outputs) in input_list {
        let mapping = build_mapping(inputs);
        let mut output_value = 0;
        for output in outputs {
            output_value = output_value * 10 + find_number(&output, &mapping).unwrap_or(0);
        }
        sum += output_value;
    }
    println!("output sum: {}", sum);
}

fn read_input() -> Vec<(Vec<Segment>, Vec<Segment>)> {
    let mut result = Vec::new();
    if let Ok(string) = fs::read_to_string("input.txt") {
        for line in string.lines() {
            let mut found_seperator = false;
            let mut input = Vec::new();
            let mut output = Vec::new();
            for string in line.split_ascii_whitespace() {
                if string == "|" {
                    found_seperator = true;
                } else if found_seperator {
                    output.push(string_to_segment(string));
                } else {
                    input.push(string_to_segment(string));
                }
            }
            result.push((input, output));
        }
    }
    return result;
}

fn string_to_segment(code: &str) -> Segment {
    let mut segment = [false; 7];
    for c in code.chars() {
        if let Some(value) = segment.get_mut(c as usize - 'a' as usize) {
            *value = true;
        }
    }
    return segment;
}

fn count_segment(segment: &Segment, check: bool) -> usize {
    let mut count = 0;
    for value in segment {
        if *value == check {
            count += 1;
        }
    }
    return count;
}

fn and_segment(a: &Segment, b: &Segment) -> Segment {
    let mut output = [false; 7];
    for i in 0..7 {
        output[i] = a[i] && b[i];
    }
    return output;
}

fn build_mapping(inputs: Vec<Segment>) -> [Segment; 10] {
    let mut mapping = [[false; 7]; 10];
    let mut five_segments = Vec::new();
    let mut six_segments = Vec::new();
    for input in inputs {
        match count_segment(&input, true) {
            2 => mapping[1] = input,
            3 => mapping[7] = input,
            4 => mapping[4] = input,
            7 => mapping[8] = input,
            5 => five_segments.push(input),
            6 => six_segments.push(input),
            _ => (),
        };
    }

    for segment in five_segments {
        if count_segment(&and_segment(&mapping[1], &segment), true) == 2 {
            mapping[3] = segment;
        } else {
            if count_segment(&and_segment(&mapping[4], &segment), true) == 2 {
                mapping[2] = segment;
            } else {
                mapping[5] = segment;
            }
        }
    }

    for segment in six_segments {
        if count_segment(&and_segment(&mapping[1], &segment), true) == 1 {
            mapping[6] = segment;
        } else {
            if count_segment(&and_segment(&mapping[4], &segment), true) == 4 {
                mapping[9] = segment;
            } else {
                mapping[0] = segment;
            }
        }
    }
    return mapping;
}

fn find_number(segment: &Segment, mapping: &[Segment; 10]) -> Option<usize> {
    for (i, map_segment) in mapping.iter().enumerate() {
        if map_segment == segment {
            return Some(i);
        }
    }
    return None;
}
