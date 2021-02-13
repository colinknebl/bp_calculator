use std::fmt;
use std::io;

use super::bp::BP;

pub struct Reading {
    bp: BP,
    hr: i16
}

impl Reading {
    pub fn new(sys: i16, dia: i16, hr: i16) -> Reading {
        Reading {
            bp: BP {
                sys,
                dia,
            },
            hr
        }
    }

    pub fn get_bp(&self) -> &BP {
        &self.bp
    }

    pub fn get_hr(&self) -> i16 {
        self.hr
    }

    pub fn parse(line: Result<String, io::Error>) -> Reading {
        let reading = line.unwrap().to_string();
        let v: Vec<&str> = reading.split("\t").collect();

        let raw_bp = v[0];
        let raw_hr = v[1];

        let bp: Vec<&str> = raw_bp.split("/").collect();

        Reading::new(
            bp[0].parse::<i16>().unwrap(), 
            bp[1].parse::<i16>().unwrap(),
            raw_hr.parse::<i16>().unwrap()
        )
    }
}

impl fmt::Display for Reading {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "BP = {}, HR = {}", self.bp, self.hr)
    }
} 