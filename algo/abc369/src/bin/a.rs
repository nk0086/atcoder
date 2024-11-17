#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
    };

    if a - b == 0 {
        println!("1");
    } else if (a - b).abs() % 2 == 0 {
        println!("3");
    } else {
        println!("2");
    }
}
