use crate::utils;

pub fn main() {
    let input = utils::string_from_file("06.txt");
    let mut lines = input.lines();

    let times = line_to_vec(lines.next().unwrap());
    let record_distances = line_to_vec(lines.next().unwrap());

    let mut result = 1;
    for (i, time) in times.iter().enumerate() {
        let mut record_beaten = 0;
        for duration in 0..*time {
            let distance = duration * (time - duration);
            if distance > record_distances[i] {
                record_beaten += 1;
            }
        }
        result *= record_beaten;
    }

    println!("{}", result);
}

fn line_to_vec(line: &str) -> Vec<u64> {
    line.split(':').collect::<Vec<&str>>()[1]
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}
