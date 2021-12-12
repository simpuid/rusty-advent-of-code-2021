use std::{fmt::Debug, fs};

fn main() {
    let inputs = read_input();

    let mut total_syntax_score = 0;
    let mut completion_score_vec = Vec::new();
    for input in inputs {
        let (stack, error) = simulate(input.as_slice());
        if let Some(error) = error {
            total_syntax_score += illegal_score(&error);
        } else {
            let mut score = 0;
            for bracket in stack.iter().rev() {
                score *= 5;
                score += completion_score(bracket);
            }
            completion_score_vec.push(score);
        }
    }
    println!("total syntax error score: {}", total_syntax_score);
    completion_score_vec.sort();
    if let Some(score) = completion_score_vec.get(completion_score_vec.len() / 2) {
        println!("median completion score: {}", score);
    }
}

fn read_input() -> Vec<Vec<Bracket>> {
    let mut result = Vec::new();
    if let Ok(string) = fs::read_to_string("input.txt") {
        for line in string.lines() {
            let mut queue = Vec::new();
            for c in line.chars() {
                if let Some(bracket) = char_to_bracket(c) {
                    queue.push(bracket);
                }
            }
            result.push(queue);
        }
    }
    return result;
}

fn char_to_bracket(c: char) -> Option<Bracket> {
    let result = match c {
        '(' => Bracket::RoundOpen,
        ')' => Bracket::RoundClose,
        '[' => Bracket::SquareOpen,
        ']' => Bracket::SquareClose,
        '{' => Bracket::CurlyOpen,
        '}' => Bracket::CurlyClose,
        '<' => Bracket::AngleOpen,
        '>' => Bracket::AngleClose,
        _ => return None,
    };
    return Some(result);
}

fn opening(bracket: &Bracket) -> Option<Bracket> {
    let result = match bracket {
        Bracket::RoundClose => Bracket::RoundOpen,
        Bracket::SquareClose => Bracket::SquareOpen,
        Bracket::CurlyClose => Bracket::CurlyOpen,
        Bracket::AngleClose => Bracket::AngleOpen,
        _ => return None,
    };
    return Some(result);
}

fn illegal_score(bracket: &Bracket) -> usize {
    return match bracket {
        Bracket::RoundClose => 3,
        Bracket::SquareClose => 57,
        Bracket::CurlyClose => 1197,
        Bracket::AngleClose => 25137,
        _ => 0,
    };
}

fn completion_score(bracket: &Bracket) -> usize {
    return match bracket {
        Bracket::RoundOpen => 1,
        Bracket::SquareOpen => 2,
        Bracket::CurlyOpen => 3,
        Bracket::AngleOpen => 4,
        _ => 0,
    };
}

fn simulate(brackets: &[Bracket]) -> (Vec<Bracket>, Option<Bracket>) {
    let mut stack = Vec::new();
    for bracket in brackets {
        if let Some(closing) = opening(bracket) {
            if let Some(last_bracket) = stack.pop() {
                if last_bracket != closing {
                    return (stack, Some(*bracket));
                }
            } else {
                return (stack, Some(*bracket));
            }
        } else {
            stack.push(*bracket);
        }
    }
    return (stack, None);
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Bracket {
    RoundOpen,
    RoundClose,
    SquareOpen,
    SquareClose,
    CurlyOpen,
    CurlyClose,
    AngleOpen,
    AngleClose,
}
