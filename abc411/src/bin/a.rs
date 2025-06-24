#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
fn main() {
    input! {
        p: Chars, l: usize
    };

    if p.len() >= l {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
}
