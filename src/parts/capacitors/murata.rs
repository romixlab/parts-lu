use crate::parts::{EIAInchCode, SizeCode, capacitors::{Capacitor, Capacitance, Error}, Height, Dielectric, RatedVoltage, Tolerance};
use strum_macros::EnumString;
use std::str::FromStr;

#[derive(EnumString, Debug)]
pub enum Series {
    GA2,
    GA3,
    GJM,
    GMA,
    GMD,
    GQM,
    GR3,
    GR4,
    GRJ,
    GRM,
    KR3,
    KRM,
    LLA,
    LLL,
    LLM,
    LLR
}

pub enum Dimensions {
    _01,
    _02,
    _0D,
    _03,
    _05,
    _08,
    _1U,
    _15,
    _18,
    _21,
    _22,
    _31,
    _32,
    _42,
    _43,
    _52,
    _55
}

impl FromStr for Dimensions {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Dimensions::*;
        match s {
            "01" => Ok(_01),
            "02" => Ok(_02),
            "0D" => Ok(_0D),
            "03" => Ok(_03),
            "05" => Ok(_05),
            "08" => Ok(_08),
            "1U" => Ok(_1U),
            "15" => Ok(_15),
            "18" => Ok(_18),
            "21" => Ok(_21),
            "22" => Ok(_22),
            "31" => Ok(_31),
            "32" => Ok(_32),
            "42" => Ok(_42),
            "43" => Ok(_43),
            "52" => Ok(_52),
            "55" => Ok(_55),
            _ => Err(Error::WrongDimensionCode)
        }
    }
}

impl SizeCode for Dimensions {
    type MFCode = Self;

    fn to_eia(code: Self) -> EIAInchCode {
        use Dimensions::*;
        match code {
            _01 => { EIAInchCode::_008004 },
            _02 => { EIAInchCode::_01005 },
            _0D => { EIAInchCode::_015015 },
            _03 => { EIAInchCode::_0201 },
            _05 => { EIAInchCode::_0202 },
            _08 => { EIAInchCode::_0303 },
            _1U => { EIAInchCode::_02404 },
            _15 => { EIAInchCode::_0402 },
            _18 => { EIAInchCode::_0603 },
            _21 => { EIAInchCode::_0805 },
            _22 => { EIAInchCode::_1111 },
            _31 => { EIAInchCode::_1206 },
            _32 => { EIAInchCode::_1210 },
            _42 => { EIAInchCode::_1808 },
            _43 => { EIAInchCode::_1812 },
            _52 => { EIAInchCode::_2211 },
            _55 => { EIAInchCode::_2220 },
        }
    }

    fn to_mfcode(size: EIAInchCode) -> Option<Self> {
        use Dimensions::*;
        match size {
            EIAInchCode::_008004 => { Some(_01) },
            EIAInchCode::_01005 => { Some(_02) },
            EIAInchCode::_015015 => { Some(_0D) },
            EIAInchCode::_0201 => { Some(_03) },
            EIAInchCode::_0202 => { Some(_05) },
            EIAInchCode::_0303 => { Some(_08) },
            EIAInchCode::_02404 => { Some(_1U) },
            EIAInchCode::_0402 => { Some(_15) },
            EIAInchCode::_0603 => { Some(_18) },
            EIAInchCode::_0805 => { Some(_21) },
            EIAInchCode::_1111 => { Some(_22) },
            EIAInchCode::_1206 => { Some(_31) },
            EIAInchCode::_1210 => { Some(_32) },
            EIAInchCode::_1808 => { Some(_42) },
            EIAInchCode::_1812 => { Some(_43) },
            EIAInchCode::_2211 => { Some(_52) },
            EIAInchCode::_2220 => { Some(_55) },
            _ => None
        }
    }
}

fn parse_height_code(code: char) -> Result<Height, Error> {
    match code {
        '1' => Ok(Height::new(0, 125)),
        '2' => Ok(Height::new(0, 2)),
        '3' => Ok(Height::new(0, 3)),
        '4' => Ok(Height::new(0, 4)),
        '5' => Ok(Height::new(0, 5)),
        '6' => Ok(Height::new(0, 6)),
        '7' => Ok(Height::new(0, 7)),
        '8' => Ok(Height::new(0, 8)),
        '9' => Ok(Height::new(0, 85)),
        'A' => Ok(Height::new(1, 0)),
        'B' => Ok(Height::new(1, 25)),
        'C' => Ok(Height::new(1, 6)),
        'D' => Ok(Height::new(2, 0)),
        'E' => Ok(Height::new(2, 5)),
        'M' => Ok(Height::new(1, 15)),
        'Q' => Ok(Height::new(1, 5)),
        'X' => Ok(Height::new(0, 0)),
        _ => Err(Error::WrongHeightCode)
    }
}

pub fn parse_dielectric(code: &str) -> Result<Dielectric, Error> {
    use Dielectric::*;
    match code {
        "5C" => Ok(C0G),
        "R7" => Ok(X7R),
        "R6" => Ok(X5R),
        "C8" => Ok(X6S),
        "1X" => Ok(SL),
        "2C" => Ok(CH),
        "3C" => Ok(CJ),
        "3U" => Ok(UJ),
        "4C" => Ok(CK),
        "5G" => Ok(X8G),
        "7U" => Ok(U2J),
        "B1" => Ok(B),
        "B3" => Ok(B),
        "C7" => Ok(X7S),
        "D7" => Ok(X7T),
        "D8" => Ok(X6T),
        "E7" => Ok(X7U),
        "R1" => Ok(R),
        _ => Err(Error::WrongDielectricCode)
    }
}

pub fn parse_voltage(code: &str) -> Result<RatedVoltage, Error> {
    use RatedVoltage::*;
    match code {
        "0E" => Ok(DC_2V5),
        "0G" => Ok(DC_4V),
        "0J" => Ok(DC_6V3),
        "1A" => Ok(DC_10V),
        "1C" => Ok(DC_16V),
        "1E" => Ok(DC_25V),
        "1H" => Ok(DC_50V),
        "1J" => Ok(DC_63V),
        "2A" => Ok(DC_100V),
        "2D" => Ok(DC_200V),
        "2E" => Ok(DC_250V),
        "2W" => Ok(DC_450V),
        "2H" => Ok(DC_500V),
        "2J" => Ok(DC_630V),
        "3A" => Ok(DC_1kV),
        "3D" => Ok(DC_2kV),
        "3F" => Ok(CustomDC(3150)),
        "E2" => Ok(AC_250V),
        "GB" => Ok(AC_250V),
        "GD" => Ok(AC_250V),
        "GF" => Ok(AC_250V),
        "YA" => Ok(DC_35V),
        _ => Err(Error::WrongVoltageCode)
    }
}

pub fn parse_capacitance(code: &str) -> Result<Capacitance, Error> {
    if !code.is_ascii() {
        return Err(Error::WrongCapacitanceCode);
    }
    let c0 = code.bytes().nth(0).unwrap();
    let c1 = code.bytes().nth(1).unwrap();
    let c2 = code.bytes().nth(2).unwrap();
    let d0 = c0 - '0' as u8;
    let d1 = c1 - '0' as u8;
    let d2 = c2 - '0' as u8;
    if c0 == 'R' as u8 {
        let c = d1 as u16 * 10 + d2 as u16;
        Ok(Capacitance::AttoFarads(c * 10))
    } else if c1 == 'R' as u8 {
        let c = d0 as u16 * 10 + d2 as u16;
        Ok(Capacitance::AttoFarads(c))
    } else if c2 == 'R' as u8 {
        Err(Error::WrongCapacitanceCode)
    } else {
        let c: u64 = d0 as u64 * 10 + d1 as u64;
        let base: u32 = 10;
        let z = base.pow(d2.into());
        Ok(Capacitance::PicoFarads(c * z as u64))
    }
}

pub fn parse_tolerance(capacitance: &Capacitance, code: char) -> Result<Tolerance, Error> {
    use Tolerance::*;
    match code {
        'B' => Ok(AttoFarads(100, 100)),
        'C' => Ok(AttoFarads(250, 250)),
        'D' => {
            if let Capacitance::AttoFarads(_) = capacitance {
                Ok(AttoFarads(500, 500))
            } else if let Capacitance::PicoFarads(p) = capacitance {
                if *p < 10 {
                    Ok(AttoFarads(500, 500))
                } else {
                    Ok(PM05)
                }
            } else {
                Err(Error::WrongToleranceCode)
            }
        },
        'F' => Ok(PM1),
        'G' => Ok(PM2),
        'J' => Ok(PM5),
        'K' => Ok(PM10),
        'M' => Ok(PM20),
        'W' => Ok(AttoFarads(50, 50)),
        _ => Err(Error::WrongToleranceCode)
    }
}

pub fn parse(part_number: &str) -> Result<Capacitor, Error> {
    if part_number.len() < 3 {
        return Err(Error::UnknownSeries);
    }
    let series = Series::from_str(&part_number[0..3]).map(|s| crate::parts::capacitors::Series::Murata(s)).map_err(|_| Error::UnknownSeries)?;
    if part_number.len() < 14 {
        return Err(Error::InsufficientData);
    }
    let dimensions = Dimensions::from_str(&part_number[3..=4])?;
    let max_height = parse_height_code(part_number.chars().nth(5).unwrap())?;
    let dielectric = parse_dielectric(&part_number[6..=7])?;
    let voltage = parse_voltage(&part_number[8..=9])?;
    let capacitance = parse_capacitance(&part_number[10..=12])?;
    let tolerance = parse_tolerance(&capacitance, part_number.chars().nth(13).unwrap())?;

    let other = if part_number.len() >= 14 {
        &part_number[14..]
    } else {
        ""
    };
    Ok(Capacitor{
        series,
        dimensions: Dimensions::to_eia(dimensions),
        max_height,
        dielectric,
        voltage,
        capacitance,
        tolerance,
        other: String::from(other)
    })
}