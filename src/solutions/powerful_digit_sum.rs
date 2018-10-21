use failure::Error;
use num::bigint::BigUint;
use num::{Integer, ToPrimitive};

pub fn solve() -> u64 {
    let mut largest_sum = None;
    for a in 0..100u64 {
        let a = BigUint::from(a);
        for b in 0..100 {
            let power = ::num::pow::pow(a.clone(), b);
            let sum = sum_big_digits(power);
            largest_sum = largest_sum.max(Some(sum));
        }
    }
    largest_sum.unwrap()
}
lazy_static! {
    static ref DIGIT_TABLE: Vec<u8> = {
        (0..1000).map(sum_digits).collect()
    };
}
fn sum_big_digits(mut target: BigUint) -> u64 {
    let thousand = BigUint::from(1000u64);
    let table = &**DIGIT_TABLE;
    let mut sum = 0;
    while target > thousand {
        let (updated_target, modulo) = target.div_mod_floor(&thousand);
        sum += table[modulo.to_usize().unwrap()] as u64;
        target = updated_target;
    }
    sum += sum_digits(target.to_u64().unwrap()) as u64;
    sum
}
fn sum_digits(mut target: u64) -> u8 {
    let mut sum = 0;
    while target > 0 {
        let digit = target % 10;
        sum += digit as u8;
        target /= 10;
    }
    sum
}