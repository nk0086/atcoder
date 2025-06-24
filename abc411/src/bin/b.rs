#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: [usize; n - 1],
    };

    let mut ans = vec![vec![]; n];
    let mut pre = vec![0; n + 1];

    for i in 0..n - 1 {
        pre[i + 1] = pre[i] + d[i];
    }

    for i in 0..n - 1 {
        for j in i..n - 1 {
            ans[i].push(pre[j + 1] - pre[i]);
        }
    }

    for i in 0..n - 1 {
        for j in 0..ans[i].len() {
            if j > 0 {
                print!(" ");
            }
            print!("{}", ans[i][j]);
        }
        println!();
    }
}
