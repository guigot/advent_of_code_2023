use std::env;
use std::fs::File;
use std::io::prelude::*;

pub fn string_from_file(input: &str) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd.join("inputs").join(input);

    let mut file = match File::open(filepath) {
        Err(why) => panic!("couldn't open : {}", why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read : {}", why),
        Ok(_) => s,
    }
}
