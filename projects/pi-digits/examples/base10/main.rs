use std::path::Path;

use pi_search::PiBase10;

fn main() -> std::io::Result<()> {
    unsafe { PiBase10::str_to_bin(&Path::new("../y-cruncher/Pi - Dec - Chudnovsky.txt"), &Path::new("src/base10.bin")) }
}
