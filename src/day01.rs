use crate::utils;

pub fn main() {
    let input = utils::string_from_file("01.txt");
    let digits = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];

    let mut all_digits: Vec<String> = Vec::new();

    let array: Vec<String> = input.lines().map(String::from).collect();
    for line in array {
        let (mut f_begin_index, mut f_end_index) = (line.len() - 1, 0);
        let (mut l_begin_index, mut l_end_index) = (0, 0);
        for digit in digits {
            if let Some(i) = line.find(digit) {
                if i <= f_begin_index {
                    f_begin_index = i;
                    f_end_index = i + digit.len();
                }
            }
            if let Some(i) = line.rfind(digit) {
                if l_begin_index <= i {
                    l_begin_index = i;
                    l_end_index = i + digit.len();
                }
            }
        }
        let temp_result =
            replace_number_string(&line[f_begin_index.to_owned()..f_end_index.to_owned()])
                .to_string()
                + replace_number_string(&line[l_begin_index.to_owned()..l_end_index.to_owned()]);
        all_digits.push(temp_result);
    }

    let mut result: i32 = 0;
    for num in all_digits {
        result += num.parse::<i32>().unwrap();
    }

    println!("{}", result);
}

fn replace_number_string(value: &str) -> &str {
    match value {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => value,
    }
}
