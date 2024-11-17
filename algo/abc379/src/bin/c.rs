#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        x: [usize; m],
        a: [usize; m],
    };

    let mut stones: Vec<(usize, usize)> = x.into_iter().zip(a.into_iter()).collect();
    stones.sort_unstable_by_key(|k| k.0);

    let mut total_moves: isize = 0;
    let mut surplus: isize = 0;
    let mut current_pos = 1;
    let mut idx = 0;

    while current_pos <= n {
        if idx < m && stones[idx].0 == current_pos {
            surplus += stones[idx].1 as isize;
            idx += 1;
        }

        surplus -= 1;
        if surplus < 0 {
            println!("-1");
            return;
        }

        total_moves += surplus;
        current_pos += 1;
    }

    if surplus != 0 {
        println!("-1");
    } else {
        println!("{}", total_moves);
    }
}
