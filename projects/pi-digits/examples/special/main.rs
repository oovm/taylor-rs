use std::path::{Path, PathBuf};

use pi_search::PiBase10;

fn main() {
    let base10 = PiBase10::load(&Path::new("src/base10.bin")).unwrap();
    let path = PathBuf::from(format!("src/computed_sp.csv"));
    let map = base10.compute_map_special();
    map.dump_readable(&path).unwrap();
}
