use std::fmt;

pub struct BP {
    pub sys: i16,
    pub dia: i16
}

impl fmt::Display for BP {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}/{}", self.sys, self.dia)
    }
}