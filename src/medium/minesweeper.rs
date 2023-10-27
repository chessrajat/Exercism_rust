use core::num;
use std::isize;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for (i, row) in minefield.iter().enumerate() {
        let mut new_row = String::new();
        for (j, ch) in row.chars().enumerate() {
            if (ch == ' ') {
                let number_of_mines = calculate_mines(i, j, minefield);
                if number_of_mines == 0 {
                    new_row += " ";
                } else {
                    new_row += number_of_mines.to_string().as_str();
                }
            } else {
                new_row += "*";
            }
        }
        result.push(new_row);
    }

    return result;
}

fn calculate_mines(row: usize, column: usize, board: &[&str]) -> usize {
    let mut number_of_mines = 0;
    let all_positions = [
        (row.checked_sub(1).unwrap_or(555), column),
        (row + 1, column),
        (row, column + 1),
        (row, column.checked_sub(1).unwrap_or(555)),
        (row.checked_sub(1).unwrap_or(555), column + 1),
        (
            row.checked_sub(1).unwrap_or(555),
            column.checked_sub(1).unwrap_or(555),
        ),
        (row + 1, column + 1),
        (row + 1, column.checked_sub(1).unwrap_or(555)),
    ];

    let n = board.len();
    let m = board[0].len();

    for (i, j) in all_positions.iter() {
        if (is_valid_pos(i, j, n, m)) {
            if (board[*i].chars().nth(*j).unwrap() == '*') {
                number_of_mines += 1;
            }
        }
    }

    return number_of_mines;
}

fn is_valid_pos(i: &usize, j: &usize, n: usize, m: usize) -> bool {
    if (i == &555 || j == &555) {
        return false;
    }
    if (i >= &n || j >= &m) {
        return false;
    }
    return true;
}
