mod parts;
use parts::capacitors::Capacitor;
use std::str::FromStr;
use crate::parts::capacitors::Capacitance;
use crate::parts::{EIAInchCode, IECMetricCode, RatedVoltage};

fn main() {
    let cap = Capacitor::from_str("GRM033R61A224ME90#").unwrap();
    println!("{}", cap);
    println!("{:#}", cap);
    println!("{:?}", cap);
    // let c = Capacitance::PicoFarads(1_500_000);
    // println!("{:#}", c);

    // let ic = EIAInchCode::_0603;
    // println!("{:#}", ic);

    // let im = IECMetricCode::_0603;
    // println!("{:#}", im);

    // let v = RatedVoltage::DC_6V3;
    // println!("{:#}", v);

}
