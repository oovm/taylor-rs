use std::str::FromStr;
use katex_wasmbind::KaTeXOptions;
use pi_search::PiBase10;
use wasm_bindgen_test::*;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let base = PiBase10::from_str(include_str!("../../../y-cruncher/Pi - Dec - Chudnovsky.txt"));
    println!("{}", base.unwrap())
}
