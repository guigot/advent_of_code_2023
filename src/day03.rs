use crate::utils;

const MAX_ROWS: usize = 140;
const MAX_COLUMNS: usize = 140;
pub struct Matrix {
    content: [[char; MAX_COLUMNS]; MAX_ROWS],
}

impl Matrix {
    pub fn get(&self, row: usize, col: usize) -> Option<char> {
        let val = self.content.get(row)?.get(col)?;
        Some(*val)
    }

    pub fn set(&mut self, row: usize, col: usize, value: char) {
        self.content[row][col] = value;
    }

    pub fn is_near_symbol(&self, row: usize, col: usize) -> bool {
        let mut is_near = false;

        let around: [i32; 3] = [-1, 0, 1];
        for a_row in around {
            let new_row = row as i32 + a_row;
            match new_row {
                -1 | 140 => continue,
                _ => (),
            }
            for a_column in around {
                let new_column = col as i32 + a_column;
                match new_column {
                    -1 | 140 => continue,
                    _ => (),
                }

                let c = self.get(new_row as usize, new_column as usize).unwrap();
                if !c.is_ascii_digit() & (c != '.') {
                    is_near = true;
                }

            }
        }
        is_near
    }

    pub fn new() -> Self {
        let columns = ['.'; MAX_COLUMNS];
        let rows = [columns; MAX_ROWS];
        Self { content: rows }
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Matrix::new()
    }
}

pub fn main() {
    let input = utils::string_from_file("03.txt");

    let mut matrix: Matrix = Matrix::new();
    for (row, line) in input.lines().enumerate() {
        for column in 0..MAX_COLUMNS {
            matrix.set(row, column, line.chars().nth(column).unwrap());
        }
    }

    let mut numbers: Vec<i32> = Vec::new();
    for row in 0..MAX_ROWS {
        let mut is_near_symbol = false;
        let mut current_number = false;
        let mut number = String::new();
        for column in 0..MAX_COLUMNS {
            let c = matrix.get(row, column).unwrap();
            if c.is_ascii_digit() {
                number.push(c);
                current_number = true;
                if !is_near_symbol {
                    is_near_symbol = matrix.is_near_symbol(row, column);
                } else if column == MAX_COLUMNS - 1 {
                    numbers.push(number.parse::<i32>().unwrap());
                    number.clear();
                    current_number = false;
                    is_near_symbol = false;
                }
            } else {
                // end of number
                if (current_number) & is_near_symbol {
                    numbers.push(number.parse::<i32>().unwrap());
                }
                number.clear();
                current_number = false;
                is_near_symbol = false;
            }
        }
    }

    let mut result = 0;
    for number in numbers {
        result += number;
    }

    println!("{}", result);
}

fn retrieve_numbers() -> String {}


fn jarrive:q
