pub mod capacitors;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum IECMetricCode {
    _0201,
    _03015,
    _0402,
    _0404,
    _0505,
    _0603,
    _0610, // *
    _0805,
    _0808,
    _1005,
    _1310,
    _1608,
    _2012,
    _2520,
    _2828,
    _3216,
    _3225,
    _3625,
    _3838,
    _4516,
    _4520,
    _4532,
    _4564,
    _5025,
    _5050,
    _5664,
    _5728, // *
    _5750,
    _5764,
    _6332,
    _6432,
    _6450,
    _7450,
    _8484,
    _9210,
    _100100,
    _140127,
    _203153
}

impl fmt::Display for IECMetricCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let d = format!("{:?}", self);
        if f.alternate() {
            write!(f, "{} Metric", &d[1..])
        } else {
            write!(f, "{}", &d[1..])
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EIAInchCode {
    _008004,
    _009005,
    _01005,
    _015015,
    _0201,
    _0202,
    _02404,
    _0302,
    _0303,
    _0402,
    _0504,
    _0603,
    _0805,
    _1008,
    _1111,
    _1206,
    _1210,
    _1410,
    _1515,
    _1806,
    _1808,
    _1812,
    _1825,
    _2010,
    _2020,
    _2211,
    _2220,
    _2225,
    _2512,
    _2520,
    _2920,
    _3333,
    _3640,
    _4040,
    _5550,
    _8060
}

impl fmt::Display for EIAInchCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let d = format!("{:?}", self);
        if f.alternate() {
            write!(f, "{} Inch", &d[1..])
        } else {
            write!(f, "{}", &d[1..])
        }
    }
}

impl From<IECMetricCode> for EIAInchCode {
    fn from(metric: IECMetricCode) -> Self {
        match metric {
            IECMetricCode::_0201 => EIAInchCode::_008004,
            IECMetricCode::_03015 => EIAInchCode::_009005,
            IECMetricCode::_0402 => EIAInchCode::_01005,
            IECMetricCode::_0404 => EIAInchCode::_015015,
            IECMetricCode::_0505 => EIAInchCode::_0202,
            IECMetricCode::_0603 => EIAInchCode::_0201,
            IECMetricCode::_0610 => EIAInchCode::_02404,
            IECMetricCode::_0805 => EIAInchCode::_0302,
            IECMetricCode::_0808 => EIAInchCode::_0303,
            IECMetricCode::_1005 => EIAInchCode::_0402,
            IECMetricCode::_1310 => EIAInchCode::_0504,
            IECMetricCode::_1608 => EIAInchCode::_0603,
            IECMetricCode::_2012 => EIAInchCode::_0805,
            IECMetricCode::_2520 => EIAInchCode::_1008,
            IECMetricCode::_2828 => EIAInchCode::_1111,
            IECMetricCode::_3216 => EIAInchCode::_1206,
            IECMetricCode::_3225 => EIAInchCode::_1210,
            IECMetricCode::_3625 => EIAInchCode::_1410,
            IECMetricCode::_3838 => EIAInchCode::_1515,
            IECMetricCode::_4516 => EIAInchCode::_1806,
            IECMetricCode::_4520 => EIAInchCode::_1808,
            IECMetricCode::_4532 => EIAInchCode::_1812,
            IECMetricCode::_4564 => EIAInchCode::_1825,
            IECMetricCode::_5025 => EIAInchCode::_2010,
            IECMetricCode::_5050 => EIAInchCode::_2020,
            IECMetricCode::_5728 => EIAInchCode::_2211,
            IECMetricCode::_5750 => EIAInchCode::_2220,
            IECMetricCode::_5664 => EIAInchCode::_2225, // *
            IECMetricCode::_5764 => EIAInchCode::_2225, // *
            IECMetricCode::_6332 => EIAInchCode::_2512, // *
            IECMetricCode::_6432 => EIAInchCode::_2512, // *
            IECMetricCode::_6450 => EIAInchCode::_2520,
            IECMetricCode::_7450 => EIAInchCode::_2920,
            IECMetricCode::_8484 => EIAInchCode::_3333,
            IECMetricCode::_9210 => EIAInchCode::_3640,
            IECMetricCode::_100100 => EIAInchCode::_4040,
            IECMetricCode::_140127 => EIAInchCode::_5550,
            IECMetricCode::_203153 => EIAInchCode::_8060,
        }
    }
}

impl From<EIAInchCode> for IECMetricCode {
    fn from(inch: EIAInchCode) -> Self {
        match inch {
            EIAInchCode::_008004 => IECMetricCode::_0201,
            EIAInchCode::_009005 => IECMetricCode::_03015,
            EIAInchCode::_01005 => IECMetricCode::_0402,
            EIAInchCode::_015015 => IECMetricCode::_0404,
            EIAInchCode::_0201 => IECMetricCode::_0603,
            EIAInchCode::_0202 => IECMetricCode::_0505,
            EIAInchCode::_02404 => IECMetricCode::_0610,
            EIAInchCode::_0302 => IECMetricCode::_0805,
            EIAInchCode::_0303 => IECMetricCode::_0808,
            EIAInchCode::_0402 => IECMetricCode::_1005,
            EIAInchCode::_0504 => IECMetricCode::_1310,
            EIAInchCode::_0603 => IECMetricCode::_1608,
            EIAInchCode::_0805 => IECMetricCode::_2012,
            EIAInchCode::_1008 => IECMetricCode::_2520,
            EIAInchCode::_1111 => IECMetricCode::_2828,
            EIAInchCode::_1206 => IECMetricCode::_3216,
            EIAInchCode::_1210 => IECMetricCode::_3225,
            EIAInchCode::_1410 => IECMetricCode::_3625,
            EIAInchCode::_1515 => IECMetricCode::_3838,
            EIAInchCode::_1806 => IECMetricCode::_4516,
            EIAInchCode::_1808 => IECMetricCode::_4520,
            EIAInchCode::_1812 => IECMetricCode::_4532,
            EIAInchCode::_1825 => IECMetricCode::_4564,
            EIAInchCode::_2010 => IECMetricCode::_5025,
            EIAInchCode::_2020 => IECMetricCode::_5050,
            EIAInchCode::_2211 => IECMetricCode::_5728,
            EIAInchCode::_2220 => IECMetricCode::_5750,
            EIAInchCode::_2225 => IECMetricCode::_5764, // or 5664
            EIAInchCode::_2512 => IECMetricCode::_6332, // or 6432
            EIAInchCode::_2520 => IECMetricCode::_6450,
            EIAInchCode::_2920 => IECMetricCode::_7450,
            EIAInchCode::_3333 => IECMetricCode::_8484,
            EIAInchCode::_3640 => IECMetricCode::_9210,
            EIAInchCode::_4040 => IECMetricCode::_100100,
            EIAInchCode::_5550 => IECMetricCode::_140127,
            EIAInchCode::_8060 => IECMetricCode::_203153,
        }
    }
}

pub trait SizeCode {
    type MFCode;

    fn to_eia(code: Self::MFCode) -> EIAInchCode;
    fn to_mfcode(size: EIAInchCode) -> Option<Self::MFCode>;
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RatedVoltage {
    DC_2V5,
    DC_4V,
    DC_6V3,
    DC_10V,
    DC_16V,
    DC_25V,
    DC_35V,
    DC_50V,
    DC_63V,
    DC_100V,
    DC_200V,
    DC_250V,
    DC_450V,
    DC_500V,
    DC_630V,
    DC_1kV,
    DC_2kV,
    DC_3kV,
    DC_3kV15,
    AC_250V,
    CustomDC(u32),
    CustomAC(u32)
}

impl fmt::Display for RatedVoltage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use RatedVoltage::*;
        let (decimal, fractional, is_dc) = match self {
            DC_2V5 => (2, 5, true),
            DC_4V => (4, 0, true),
            DC_6V3 => (6, 3, true),
            DC_10V => (10, 0, true),
            DC_16V => (16, 0, true),
            DC_25V => (25, 0, true),
            DC_35V => (35, 0, true),
            DC_50V => (50, 0, true),
            DC_63V => (63, 0, true),
            DC_100V => (100, 0, true),
            DC_200V => (200, 0, true),
            DC_250V => (250, 0, true),
            DC_450V => (450, 0, true),
            DC_500V => (500, 0, true),
            DC_630V => (630, 0, true),
            DC_1kV => (1000, 0, true),
            DC_2kV => (2000, 0, true),
            DC_3kV => (3000, 0, true),
            DC_3kV15 => (3150, 0, true),
            AC_250V => (250, 0, false),
            CustomDC(dc) => (*dc, 0, true),
            CustomAC(ac) => (*ac, 0, false),
        };
        if fractional == 0 {
            if is_dc {
                write!(f, "{}V", decimal)
            } else {
                write!(f, "{}VAC", decimal)
            }
        } else {
            if f.alternate() {
                write!(f, "{}V{}", decimal, fractional)
            } else {
                write!(f, "{}.{}V", decimal, fractional)
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tolerance {
    /// ±0.5%
    PM05,
    /// ±1%
    PM1,
    /// ±2%
    PM2,
    /// +5%
    P5,
    /// -5%
    M5,
    /// ±5%
    PM5,
    /// ±10%
    PM10,
    /// ±20%
    PM20,
    /// -.0% +.1%
    Percent(u8, u8),
    /// ±0.1pF
    PM0pF1,
    /// ±0.25pF
    PM0pF25,
    /// ±0.5pF
    PM0pF5,
    /// ±1pF
    PM1pF,
    /// -.0aF +.1aF
    AttoFarads(u64, u64)
}

impl fmt::Display for Tolerance {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            match self {
                Tolerance::PM05 => write!(f, "U"), // ultra precise
                Tolerance::PM1 => write!(f, "P"), // precise
                Tolerance::PM2 => write!(f, "P"), // precise
                Tolerance::PM5 => write!(f, "P"), // precise
                Tolerance::PM10 => write!(f, "S"), // standard
                Tolerance::PM20 => write!(f, "C"), // coarse
                Tolerance::PM0pF1 => write!(f, "U"), // ultra precise
                Tolerance::PM0pF25 =>write!(f, "P"), // precise
                Tolerance::PM0pF5 => write!(f, "S"), // standard
                Tolerance::PM1pF => write!(f, "C"), // coarse
                _ => { write!(f, "") }
            }
        } else {
            match self {
                Tolerance::PM05 => write!(f, "±0.5%"),
                Tolerance::PM1 => write!(f, "±1%"),
                Tolerance::PM2 => write!(f, "±2%"),
                Tolerance::P5 => write!(f, "+5%"),
                Tolerance::M5 => write!(f, "-5%"),
                Tolerance::PM5 => write!(f, "±5%"),
                Tolerance::PM10 => write!(f, "±10%"),
                Tolerance::PM20 => write!(f, "±20%"),
                Tolerance::Percent(b, a) => {
                    if a == b {
                        write!(f, "±{}%", a)
                    } else {
                        write!(f, "-{}+{}%", a, b)
                    }
                },
                Tolerance::PM0pF1 => write!(f, "±0.1pF"),
                Tolerance::PM0pF25 => write!(f, "±0.25pF"),
                Tolerance::PM0pF5 => write!(f, "±0.5pF"),
                Tolerance::PM1pF => write!(f, "±1pF"),
                Tolerance::AttoFarads(b, a) => {
                    if a == b {
                        write!(f, "±{}pF", a)
                    } else {
                        write!(f, "-{}+{}pF", a, b)
                    }
                },
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Height {
    pub decimal: u8,
    pub fractional: u8
}

impl Height {
    pub fn new(decimal: u8, fractional: u8) -> Self {
        Height { decimal, fractional }
    }
}

impl fmt::Display for Height {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.fractional == 0 {
            write!(f, "{}mm", self.decimal)
        } else {
            write!(f, "{}.{}mm", self.decimal, self.fractional)
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Dielectric {
    SL,
    CH,
    CJ,
    UJ,
    CK,
    C0G,
    X8G,
    U2J,
    B,
    X5R,
    X6S,
    X6T,
    X7R,
    X7S,
    X7T,
    X7U,
    R,
    Y5V,
}