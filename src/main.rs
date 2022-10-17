extern crate ddc;

use ddc::Ddc;
use ddc_winapi::Monitor;

fn ddc<D: Ddc>(mut ddc: D)
where
    D::Error: ::std::fmt::Debug,
{
    let input = ddc.get_vcp_feature(0x60).expect("failed to read VCP value");
    println!("input is {:?}", input);
}

fn main() {
    for monitor in Monitor::enumerate().unwrap() {
        println!("Monitor: {:?}", monitor);
        ddc(monitor)
    }
}
