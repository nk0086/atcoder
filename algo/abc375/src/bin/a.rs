#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let mut count = 0;
    for i in 0..n.saturating_sub(2) {
        if s[i] == '#' && s[i + 1] == '.' && s[i + 2] == '#' {
            count += 1;
        }
    }

    println!("{}", count);
}
