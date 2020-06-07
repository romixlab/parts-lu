pub mod samsung;
pub mod murata;
use crate::parts::{EIAInchCode, Height, Dielectric, RatedVoltage, Tolerance, IECMetricCode};
use std::str::FromStr;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum Capacitance {
    AttoFarads(u16),
    PicoFarads(u64),
    NonStandard
}

fn strip_zeros_right(x: u16) -> u16 {
    if x % 100 == 0 {
        x / 100
    } else if x % 10 == 0 {
        x / 10
    } else {
        x
    }
}

fn format_capacitance(c: &Capacitance, infix: bool, f: &mut fmt::Formatter) -> fmt::Result {
    let unit = if infix {
        ("P", "N", "U")
    } else {
        ("pF", "nF", "uF")
    };
    match c {
        Capacitance::AttoFarads(a) => {
            let a = *a;
            if a < 1_000 {
                if infix {
                    write!(f, "0P{}", a / 100)
                } else {
                    write!(f, "0.{}pF", a / 100)
                }
            } else {
                let decimal = a / 1000;
                let fractional = a % 1000;
                if infix {
                    write!(f, "{}P{}", decimal, strip_zeros_right(fractional))
                } else {
                    write!(f, "{}.{}pF", decimal, strip_zeros_right(fractional))
                }
            }
        },
        Capacitance::PicoFarads(p) => {
            let p = *p;
            if p < 1_000 {
                write!(f, "{}{}", p, unit.0)
            } else if p >= 1_000 && p < 1_000_000 {
                let decimal = p / 1000;
                let fractional = (p % 1000) as u16;
                if fractional == 0 {
                    write!(f, "{}{}", decimal, unit.1)
                } else {
                    if infix {
                        write!(f, "{}N{}", decimal, strip_zeros_right(fractional))
                    } else {
                        write!(f, "{}.{}nF", decimal,strip_zeros_right(fractional))
                    }
                }
            } else {
                let p = p / 1000;
                let decimal = p / 1000;
                let fractional = (p % 1000) as u16;
                if fractional == 0 {
                    write!(f, "{}{}", decimal, unit.2)
                } else {
                    if infix {
                        write!(f, "{}U{}", decimal, strip_zeros_right(fractional))
                    } else {
                        write!(f, "{}.{}uF", decimal, strip_zeros_right(fractional))
                    }
                }
            }

        },
        Capacitance::NonStandard => { write!(f, "NonSTD") },
    }
}

impl std::fmt::Display for Capacitance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() { // 3P3, 4N7, 2U2
            format_capacitance(self, true, f)
        } else { // 2.2uF
            format_capacitance(self, false, f)
        }
    }
}

#[derive(Debug)]
pub enum Series {
    Samsung(samsung::Series),
    Murata(murata::Series),
}

#[derive(Debug)]
pub struct Capacitor {
    pub series: Series,
    pub dimensions: EIAInchCode,
    pub max_height: Height,
    pub dielectric: Dielectric,
    pub voltage: RatedVoltage,
    pub capacitance: Capacitance,
    pub tolerance: Tolerance,
    pub other: String,
}

impl fmt::Display for Capacitor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "C{}_{:#}{:#}{:#}{:?}",
                self.dimensions,
                self.capacitance,
                self.tolerance,
                self.voltage,
                self.dielectric
            )
        } else {
            write!(f, "CAP {}{} {} {:?} {}({:#}) Height={}",
                self.capacitance,
                self.tolerance,
                self.voltage,
                self.dielectric,
                self.dimensions,
                IECMetricCode::from(self.dimensions.clone()),
                self.max_height
            )
        }
    }
}

#[derive(Debug)]
pub enum Error {
    UnknownSeries,
    WrongDimensionCode,
    WrongHeightCode,
    WrongDielectricCode,
    WrongVoltageCode,
    WrongCapacitanceCode,
    WrongToleranceCode,
    InsufficientData
}

macro_rules! skip_unknown {
    ($parser:expr) => {
        match $parser {
            Ok(cap) => { return Ok(cap); },
            Err(e) => {
                if let Error::UnknownSeries = e {
                } else {
                    return Err(e)
                }
            }
        }
    }
}

impl FromStr for Capacitor {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        skip_unknown!(murata::parse(s));

        Err(Error::UnknownSeries)
    }
}