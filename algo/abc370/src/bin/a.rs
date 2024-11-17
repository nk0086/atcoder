#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
fn main() {
    input! {
        l: usize, r: usize
    };

    if l + r != 1 {
        println!("Invalid");
    } else if l == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
