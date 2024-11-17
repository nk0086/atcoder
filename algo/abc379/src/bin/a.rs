#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
fn main() {
    input! {
        n: usize,
    };

    let a = n / 100;
    let b = (n / 10) % 10;
    let c = n % 10;

    // output bca, cab
    let bca = b * 100 + c * 10 + a;
    let cab = c * 100 + a * 10 + b;

    println!("{} {}", bca, cab);
}
