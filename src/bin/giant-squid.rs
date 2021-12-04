use std::fs;

fn main() {
    let (sequence, mut boards) = read_input();
    println!("{:?}\n\n\n{:?}", sequence, boards);
    let mut rank = 1;
    for number in sequence {
        let mut new_boards = Vec::new();
        while let Some(mut board) = boards.pop() {
            mark_number(&mut board, number);
            if !check_board(&board) {
                new_boards.push(board);
            } else {
                println!(
                    "rank: {}, final score: {}",
                    rank,
                    unmarked_sum(&board) * (number as u32)
                );
                rank += 1;
            }
        }
        boards = new_boards;
    }
}

fn read_input() -> (Vec<u8>, Vec<Vec<Option<u8>>>) {
    let mut sequence = Vec::new();
    let mut boards = Vec::new();
    if let Ok(mut string) = fs::read_to_string("input.txt") {
        string.push('\n');
        let mut line_iter = string.lines();
        if let Some(first_line) = line_iter.next() {
            for numbers in first_line.split(',') {
                if let Ok(value) = numbers.parse() {
                    sequence.push(value);
                }
            }
        }
        let mut board_vec = Vec::new();
        while let Some(line) = line_iter.next() {
            let mut number_count = 0;
            for numbers in line.split(' ') {
                if let Ok(value) = numbers.parse() {
                    board_vec.push(Some(value));
                    number_count += 1;
                }
            }
            if number_count == 0 {
                boards.push(board_vec.clone());
                board_vec.clear();
            }
        }
    }
    return (sequence, boards);
}

fn check_board(board: &Vec<Option<u8>>) -> bool {
    if board.len() != 25 {
        return false;
    }

    for x in 0..5 {
        let mut marked = true;
        for y in 0..5 {
            if let Some(Some(_)) = board.get(x + y * 5) {
                marked = false;
            }
        }
        if marked {
            return true;
        }
    }

    for y in 0..5 {
        let mut marked = true;
        for x in 0..5 {
            if let Some(Some(_)) = board.get(x + y * 5) {
                marked = false;
            }
        }
        if marked {
            return true;
        }
    }

    return false;
}

fn mark_number(board: &mut Vec<Option<u8>>, marker_number: u8) {
    for number in board.iter_mut() {
        if *number == Some(marker_number) {
            *number = None;
        }
    }
}

fn unmarked_sum(board: &Vec<Option<u8>>) -> u32 {
    let mut sum: u32 = 0;
    for number in board {
        if let Some(number) = number {
            sum += *number as u32;
        }
    }
    return sum;
}
