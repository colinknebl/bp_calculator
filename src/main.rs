use std::path::Path;
use std::io::BufReader;
use std::fs::File;
use std::io::Error;
use std::io::prelude::*;

mod calc;

fn main() {
    println!("BP Calculator!");
    let path = Path::new("readings.txt");

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("couldn't open {}: {}", path.display(), Error::to_string(&e)),
    };
    let reader = BufReader::new(file);

    let mut calc = calc::Calculator::new();

    for line in reader.lines() {
        calc.add_reading(line);
    }

    println!("Average over {} readings: {}", calc.num_of_readings(), calc.calculate_average());
}
