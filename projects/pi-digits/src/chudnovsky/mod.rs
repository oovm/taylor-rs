// http://www.craig-wood.com/nick/articles/pi-chudnovsky/

use std::time::Instant;
use std::ops::Mul;
use dashu::base::SquareRoot;
use dashu::integer::IBig;
use num::Zero;

fn pi_chudnovsky_bs(digits: u32) -> IBig {
    // c3_24 = 640320^3 / 24
    let c3_24 = IBig::from(10939058860032000_u128);

    // 计算多少项
    // d = log10(c3_24 / 6 / 2 / 6)
    let n = IBig::from(digits / 151931373056000_u128.ilog10() + 1);
    // 计算P(0,N)和Q(0,N)
    let (p, q, t) = with_pqt(&IBig::zero(), &n, &c3_24);
    let one_squared = IBig::from(10).pow(2 * digits as usize);
    let sqrt_c: IBig = one_squared.mul(10005);
    (q * 426880 * sqrt_c.sqrt()) / t
}

fn with_pqt(a: &IBig, b: &IBig, c3_24: &IBig) -> (IBig, IBig, IBig) {
    /*
    计算二进制分裂Chudnovsky无限级数的项

    a(a) = +/- (13591409 + 545140134*a)
    p(a) = (6*a-5)*(2*a-1)*(6*a-1)
    b(a) = 1
    q(a) = a*a*a*C3_OVER_24

    返回P(a,b), Q(a,b)和T(a,b)
    */
    if (b - a).is_one() {
        // 直接计算P(a,a+1), Q(a,a+1)和T(a,a+1)
        if a.is_zero() {
            (IBig::from(1), IBig::from(1), IBig::from(1))
        } else {
            let pab: IBig = (6 * a - 5) * (2 * a - 1) * (6 * a - 1);
            let qab: IBig = a.pow(3) * c3_24;
            let tab: IBig = pab.clone() * (13591409 + 545140134 * a); // a(a) * p(a)
            if a % 2 == 1 {
                (-tab, qab, pab)
            } else {
                (tab, qab, pab)
            }
        }
    } else {
        // 递归计算P(a,b), Q(a,b)和T(a,b)
        // m是a和b的中点
        let m = IBig::from((a + b) / 2);
        // 递归计算P(a,m), Q(a,m)和T(a,m)
        let (pam, qam, tam) = with_pqt(a, &m, c3_24);
        // 递归计算P(m,b), Q(m,b)和T(m,b)
        let (pmb, qmb, tmb) = with_pqt(&m, b, c3_24);
        // 现在结合
        let pab = pam.clone() * pmb;
        let qab = qam * qmb.clone();
        let tab = qmb * tam + pam * tmb;
        (pab, qab, tab)
    }
}

#[test]
fn main() {
    // 不同位数的pi的最后5位数字
    let check_digits = vec![
        (100, 70679),
        (1000, 1989),
        (10000, 75678),
        (100000, 24646),
        (1000000, 58151),
        (10000000, 55897),
    ];

    let digits = 100;
    let start = Instant::now();
    let pi = pi_chudnovsky_bs(digits);
    println!("{}", pi);
    println!("time: {:?}", start.elapsed());
    for (log10_digits, check_digit) in check_digits {
        let digits = 10_i32.pow(log10_digits as u32) as u32;
        let start = Instant::now();
        let pi = pi_chudnovsky_bs(digits);
        println!("chudnovsky_gmpy_mpz_bs: digits {}, time {:?}", digits, start.elapsed());
        let last_five_digits = pi % 100000;
        if check_digit == last_five_digits {
            println!("Last 5 digits {:05} OK", last_five_digits);
        } else {
            println!("Last 5 digits {:05} wrong should be {:05}", last_five_digits, check_digit);
        }
    }
}
