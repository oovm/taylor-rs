#![feature(iter_from_generator)]
#![feature(generators)]

use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufReader, Read};
use std::iter::from_generator;
use std::path::Path;
use rand::prelude::SmallRng;
use rand::{Rng, SeedableRng};

// mod chudnovsky;

fn main() -> std::io::Result<()> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../y-cruncher/Pi - Dec - Chudnovsky.txt");
    for record in read_until_same(&path) {
        println!("{:?}", record);
    }
    Ok(())
}

pub fn read_random<const N: usize>() -> impl Iterator<Item=Record<N>> {
    let mut count = 0;
    let mut map = Record::default();
    // let file = BufReader::new(File::open(path).expect("Failed to open file"));
    let mut rng = SmallRng::from_entropy();
    from_generator(move || {
        loop {
            map.record_index(rng.gen_range(0..N));
            if map.variance() == 0.0 {
                yield map;
                if count > 100 {
                    yield return;
                }


            }
            count += 1;
            if count % 1000000 == 0 {
                println!("已搜索 {} 位", count);
                println!("{:?}", map);
            }
        }
    })
}

pub fn read_until_same(path: &Path) -> impl Iterator<Item=Record<10>> {
    let mut count = 0;
    let mut map = Record::default();
    let file = BufReader::new(File::open(path).expect("Failed to open file"));
    from_generator(move || {
        for c in file.bytes() {
            match c {
                Ok(b) => {
                    map.record_byte(b);
                    if map.variance() < 100.0 {
                        yield map;
                    }
                }
                Err(e) => panic!("{}", e),
            }
            count += 1;
            if count % 1000000 == 0 {
                println!("已搜索 {} 位", count);
                println!("{:?}", map);
            }
        }
    })
}

#[derive(Copy, Clone)]
pub struct Record<const N: usize> {
    map: [usize; N],
}

impl<const N: usize> Default for Record<N> {
    fn default() -> Self {
        Self {
            map: [0; N],
        }
    }
}

impl<const N: usize> Debug for Record<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let map: BTreeMap<usize, usize> = self.map.iter().enumerate().map(|(i, &v)| (i, v)).collect();
        f.debug_struct("Record")
            .field("mean", &self.mean())
            .field("variance", &self.variance())
            .field("map", &map)
            .finish()
    }
}


impl<const N: usize> Record<N> {
    pub fn record_byte(&mut self, b: u8) {
        if b >= b'0' && b <= b'9' {
            self.map[(b - b'0') as usize] += 1;
        }
    }
    pub fn record_char(&mut self, c: char) {
        if c.is_ascii() {
            self.record_byte(c as u8);
        }
    }
    pub fn record_index(&mut self, n: usize) {
        self.map[n] += 1;
    }

    pub fn mean(&self) -> f64 {
        (self.map.iter().sum::<usize>() / N) as f64
    }
    pub fn variance(&self) -> f64 {
        let mut delta = 0.0;
        let mean = self.mean();
        for &v in self.map.iter() {
            delta += (v as f64 - mean).powi(2);
        }
        delta / N as f64
    }
}
