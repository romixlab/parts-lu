use crate::parts::{EIAInchCode, SizeCode};
use strum_macros::EnumString;

#[derive(EnumString, Debug)]
pub enum Series {
    CL,
}

pub enum Dimensions {
    _02,
    _03,
    _05,
    _10,
    _21,
    _31,
    _32,
    _42,
    _43,
    _55
}

impl SizeCode for Dimensions {
    type MFCode = Self;

    fn to_eia(code: Self) -> EIAInchCode {
        use Dimensions::*;
        match code {
            _02 => { EIAInchCode::_01005 },
            _03 => { EIAInchCode::_0201 },
            _05 => { EIAInchCode::_0402 },
            _10 => { EIAInchCode::_0603 },
            _21 => { EIAInchCode::_0805 },
            _31 => { EIAInchCode::_1206 },
            _32 => { EIAInchCode::_1210 },
            _42 => { EIAInchCode::_1808 },
            _43 => { EIAInchCode::_1812 },
            _55 => { EIAInchCode::_2220 },
        }
    }

    fn to_mfcode(size: EIAInchCode) -> Option<Self> {
        use Dimensions::*;
        match size {
            EIAInchCode::_01005 => { Some(_02) },
            EIAInchCode::_0201 => { Some(_03) },
            EIAInchCode::_0402 => { Some(_05) },
            EIAInchCode::_0603 => { Some(_10) },
            EIAInchCode::_0805 => { Some(_21) },
            EIAInchCode::_1206 => { Some(_31) },
            EIAInchCode::_1210 => { Some(_32) },
            EIAInchCode::_1808 => { Some(_42) },
            EIAInchCode::_1812 => { Some(_43) },
            EIAInchCode::_2220 => { Some(_55) },
            _ => None
        }
    }
}