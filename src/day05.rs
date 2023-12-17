use crate::utils;

struct AlmanachRange {
    destination: u64,
    source: u64,
    length: u64,
}

pub fn main() {
    let input = utils::string_from_file("05.txt");

    // let mut seeds: Vec<usize> = (0..MAX_SIZE_VEC).collect();
    let mut seed_to_soil: Vec<AlmanachRange> = Vec::new();
    let mut soil_to_fertilizer: Vec<AlmanachRange> = Vec::new();
    let mut fertilizer_to_water: Vec<AlmanachRange> = Vec::new();
    let mut water_to_light: Vec<AlmanachRange> = Vec::new();
    let mut light_to_temperature: Vec<AlmanachRange> = Vec::new();
    let mut temperature_to_humidity: Vec<AlmanachRange> = Vec::new();
    let mut humidity_to_location: Vec<AlmanachRange> = Vec::new();

    let mut lines = input.lines();
    let seeds_to_plant = lines.next().unwrap().split(':').collect::<Vec<&str>>()[1]
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    while let Some(line) = lines.next() {
        match line {
            "seed-to-soil map:" => fill_vec(&mut lines, &mut seed_to_soil),
            "soil-to-fertilizer map:" => fill_vec(&mut lines, &mut soil_to_fertilizer),
            "fertilizer-to-water map:" => fill_vec(&mut lines, &mut fertilizer_to_water),
            "water-to-light map:" => fill_vec(&mut lines, &mut water_to_light),
            "light-to-temperature map:" => fill_vec(&mut lines, &mut light_to_temperature),
            "temperature-to-humidity map:" => fill_vec(&mut lines, &mut temperature_to_humidity),
            "humidity-to-location map:" => fill_vec(&mut lines, &mut humidity_to_location),
            _ => (),
        }
    }

    let mut lowest_location: u64 = u64::MAX;
    let mut it_seeds_to_plant = seeds_to_plant.iter();
    while let Some(range_start) = it_seeds_to_plant.next() {
        let range_end = it_seeds_to_plant.next();

        for seed in *range_start..range_start + range_end.unwrap() {
            let soil = source_to_map(seed, &seed_to_soil);
            let fertilizer = source_to_map(soil, &soil_to_fertilizer);
            let water = source_to_map(fertilizer, &fertilizer_to_water);
            let light = source_to_map(water, &water_to_light);
            let temperature = source_to_map(light, &light_to_temperature);
            let humdity = source_to_map(temperature, &temperature_to_humidity);
            let location = source_to_map(humdity, &humidity_to_location);
            if location < lowest_location {
                lowest_location = location;
            }
        }
    }
    println!("{}", lowest_location);
}

fn fill_vec<'a, I: Iterator<Item = &'a str>>(iterator: &mut I, vec: &mut Vec<AlmanachRange>) {
    for line in iterator {
        if line.is_empty() {
            break;
        } else {
            let values = line
                .split(' ')
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            let range = AlmanachRange {
                destination: values[0],
                source: values[1],
                length: values[2],
            };

            vec.push(range);
        }
    }
}

fn source_to_map(value: u64, map: &Vec<AlmanachRange>) -> u64 {
    let destination = value;
    for range in map {
        if (value >= range.source) & (value <= range.source + range.length) {
            let new_dest = range.destination + value - range.source;
            return new_dest;
        }
    }

    destination
}
