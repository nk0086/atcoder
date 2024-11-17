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

    let mut a = vec![];
    for i in 0..n {
        input! {
            a_i: [usize; i + 1],
        };

        a.push(a_i);
    }

    let mut e = 1;
    for i in 1..=n {
        if e < i {
            e = a[i - 1][e - 1];
        } else {
            e = a[e - 1][i - 1];
        }
    }

    println!("{}", e);
}
