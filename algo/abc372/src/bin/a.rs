#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
fn main() {
    input! {
        s: Chars
    };

    // remove `.` in s
    let mut ans = vec![];
    for i in s {
        if i == '.' {
            continue;
        }

        ans.push(i);
    }

    for i in ans {
        print!("{}", i);
    }
}
