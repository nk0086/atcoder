use num::{BigUint, FromPrimitive, Zero};
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    // Initialize variables
    let mut total_sum = BigUint::zero();
    let mut prev = BigUint::zero();
    let ten = BigUint::from_u8(10).unwrap();

    for (i, &ch) in s.iter().enumerate() {
        let num = BigUint::from_u32(ch.to_digit(10).unwrap()).unwrap();
        let index = BigUint::from_usize(i + 1).unwrap();
        prev = &prev * &ten + num * index;
        total_sum += &prev;
    }

    println!("{}", total_sum);
}
