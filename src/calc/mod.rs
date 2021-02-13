use std::io;
use std::i16;
use std::convert::TryFrom;

mod reading;
mod bp;

use reading::Reading;

pub struct Calculator {
    readings: Vec<Reading>,
}

impl Calculator {
    pub fn new() -> Calculator {
        Calculator {
            readings: vec![]
        }
    }

    pub fn add_reading(&mut self, line: Result<String, io::Error>) {
        self.readings.push(Reading::parse(line));
    }

    pub fn num_of_readings(&self) -> i16 {
        i16::try_from(self.readings.len()).unwrap()
    }

    pub fn calculate_average(&mut self) -> Reading {
        let mut total_sys: i16 = 0;
        let mut total_dia: i16 = 0;
        let mut total_hr: i16 = 0;
        for reading in self.readings.iter() {
            let bp = reading.get_bp();
            total_sys += bp.sys;
            total_dia += bp.dia;
            total_hr += reading.get_hr();
        }
        let num_of_readings = self.num_of_readings();
        Reading::new(
            total_sys / num_of_readings,
            total_dia / num_of_readings,
            total_hr / num_of_readings
        )
    }
}