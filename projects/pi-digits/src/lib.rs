/***************************************************************
 * Computing pi by Binary Splitting Algorithm with GMP libarary.
 **************************************************************/
use std::io::Write;
use std::time::Instant;
use dashu::base::SquareRoot;
use dashu::integer::IBig;
use num::Zero;

mod chudnovsky;

struct PQT
{
    p: IBig,
    q: IBig,
    t: IBig,
}

struct Chudnovsky
{
    // Declaration
    A: IBig,
    B: IBig,
    C: IBig,
    D: IBig,
    E: IBig,
    C3_24: IBig,
    // GMP Integer
    DIGITS: i32,
    PREC: i32,
    N: i32,
    DIGITS_PER_TERM: f64,
    // Long
    t0: Instant,
    t1: Instant,
    t2: Instant,
}

impl Chudnovsky {
    fn new() -> Chudnovsky {
        // Constants
        let digits = 100;
        let a = IBig::from(13591409);
        let b = IBig::from(545140134);
        let c = IBig::from(640320);
        let d = IBig::from(426880);
        let e = IBig::from(10005);
        // log(53360^3) / log(10)
        let digits_per_term = 14.1816474627254776555;
        let c3_24 = &c * &c * &c / 24;
        let n = (digits as f64 / digits_per_term) as i32;
        let prec = digits * (10 as i32).ilog2() as i32;
        Chudnovsky {
            A: a,
            B: b,
            C: c,
            D: d,
            E: e,
            C3_24: c3_24,
            DIGITS: digits,
            PREC: prec,
            N: n,
            DIGITS_PER_TERM: digits_per_term,
            t0: Instant::now(),
            t1: Instant::now(),
            t2: Instant::now(),
        }
    }

    /*
     * Compute PQT (by Binary Splitting Algorithm)
     */
    fn comp_pqt(&self, n1: i32, n2: i32) -> PQT {
        let m: i32;
        let mut res = PQT {
            p: IBig::zero(),
            q: IBig::zero(),
            t: IBig::zero(),
        };

        if n1 + 1 == n2 {
            res.p = (2 * n2 - 1).into();
            res.p *= (6 * n2 - 1);
            res.p *= (6 * n2 - 5);
            res.q = &self.C3_24 * n2 * n2 * n2;
            res.t = (&self.A + &self.B * n2) * &res.p;
            if n2 & 1 == 1 {
                res.t = -&res.t;
            }
        } else {
            m = (n1 + n2) / 2;
            let res1 = self.comp_pqt(n1, m);
            let res2 = self.comp_pqt(m, n2);
            res.p = &res1.p * &res2.p;
            res.q = &res1.q * &res2.q;
            res.t = &res1.t * &res2.q + &res1.p * &res2.t;
        }

        res
    }

    /*
     * Compute PI
     */
    fn comp_pi(&mut self) {
        println!("**** PI Computation ( {} digits )", self.DIGITS);

        // Time (start)
        self.t0 = Instant::now();

        // Compute Pi
        let pqt = self.comp_pqt(0, self.N);
        let mut pi = IBig::zero();
        pi = &self.D * (&self.E).sqrt() * &pqt.q;
        pi /= &self.A * &pqt.q + &pqt.t;

        // Time (end of computation)
        self.t1 = Instant::now();
        println!("TIME (COMPUTE): {:.9} seconds.", (self.t1 - self.t0).as_secs_f64());

        // Output
        let mut file = std::fs::File::create("pi.txt").unwrap();
        let mut buf = Vec::new();
        write!(&mut buf, "{}", pi).unwrap();
        file.write_all(&buf).unwrap();

        // Time (end of writing)
        self.t2 = Instant::now();
        println!("TIME (WRITE)  : {:.9} seconds.", (self.t2 - self.t1).as_secs_f64());
    }
}

#[test]
fn main() {
    let mut obj_main = Chudnovsky::new();

    // Compute PI
    obj_main.comp_pi();
}
