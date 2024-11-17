#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [(i64, char); n],
    };

    let mut ans = 0;

    let mut l_hand = -1;
    let mut r_hand = -1;

    for &(ai, hand) in &a {
        if l_hand == -1 && hand == 'L' {
            l_hand = ai;
        }

        if r_hand == -1 && hand == 'R' {
            r_hand = ai;
        }
    }

    for (ai, hand) in a {
        if hand == 'L' {
            ans += (ai - l_hand).abs();
            l_hand = ai;
        } else {
            ans += (r_hand - ai).abs();
            r_hand = ai;
        }
    }

    println!("{}", ans);
}
