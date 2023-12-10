use crate::utils;

const MAX_SIZE: usize = 140;
pub struct Matrix {
    content: [[char; MAX_SIZE]; MAX_SIZE],
}

impl Matrix {
    pub fn get(&self, row: usize, col: usize) -> Option<char> {
        let val = self.content.get(row)?.get(col)?;
        Some(*val)
    }

    pub fn set(&mut self, row: usize, col: usize, value: char) {
        self.content[row][col] = value;
    }

    pub fn result_around_asterisk(&self, row: usize, col: usize) -> i32 {
        let mut numbers: Vec<i32> = Vec::new();
        let around: [i32; 3] = [-1, 0, 1];
        for a_row in around {
            let new_row = row as i32 + a_row;
            match new_row {
                -1 | 140 => continue,
                _ => (),
            }
            let mut already_number = false;
            for a_column in around {
                let new_column = col as i32 + a_column;
                match new_column {
                    -1 | 140 => continue,
                    _ => (),
                }

                let c = self.get(new_row as usize, new_column as usize).unwrap();
                if c.is_ascii_digit() & !already_number {
                    let mut number = String::new();
                    already_number = true;
                    let mut i = new_column;
                    while i >= 0 {
                        let d = self.get(new_row as usize, i as usize).unwrap();
                        if d.is_ascii_digit() {
                            number.insert(0, self.get(new_row as usize, i as usize).unwrap());
                        } else {
                            break;
                        }
                        i -= 1;
                    }
                    i = new_column + 1;
                    while i < MAX_SIZE as i32 {
                        let d = self.get(new_row as usize, i as usize).unwrap();
                        if d.is_ascii_digit() {
                            number.push(self.get(new_row as usize, i as usize).unwrap());
                        } else {
                            break;
                        }
                        i += 1;
                    }
                    // println!("{}", number);
                    numbers.push(number.parse::<i32>().unwrap());
                } else if !c.is_ascii_digit() {
                    already_number = false;
                }
            }
        }
        if numbers.len() > 1 {
            let mut result = 1;
            for i in numbers {
                result *= i;
            }
            result
        } else {
            0
        }
    }

    pub fn new() -> Self {
        let columns = ['.'; MAX_SIZE];
        let rows = [columns; MAX_SIZE];
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
        for column in 0..MAX_SIZE {
            matrix.set(row, column, line.chars().nth(column).unwrap());
        }
    }

    let mut numbers: Vec<i32> = Vec::new();
    for row in 0..MAX_SIZE {
        for column in 0..MAX_SIZE {
            if matrix.get(row, column).unwrap() == '*' {
                numbers.push(matrix.result_around_asterisk(row, column));
            }
        }
    }

    let mut result = 0;
    for number in numbers {
        result += number;
    }

    println!("{}", result);
}
