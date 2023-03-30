use std::path::Path;

use pi_search::{PiBase10, PiBase256};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
#[ignore]
fn dump_base10() -> std::io::Result<()> {
    unsafe { PiBase10::str_to_bin(&Path::new("../y-cruncher/Pi - Dec - Chudnovsky.txt"), &Path::new("src/base10.bin")) }
}

#[test]
#[ignore]
fn dump_base256() -> std::io::Result<()> {
    unsafe { PiBase256::str_to_bin(&Path::new("../y-cruncher/Pi - Hex - Chudnovsky.txt"), &Path::new("src/base256.bin")) }
}
