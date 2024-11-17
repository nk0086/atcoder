#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        ab: [(usize, char); m],
    };

    let mut ans = vec![0; 105];
    for (a, b) in ab {
        if b == 'F' {
            println!("{}", "No");
            continue;
        }

        if ans[a] == 0 {
            ans[a] = 1;
            println!("{}", "Yes");
        } else {
            println!("{}", "No");
        }
    }
}
