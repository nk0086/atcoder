#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };

    let mut pre = vec![0; n - 1];

    for i in 0..n - 1 {
        pre[i] = a[i + 1] - a[i];
    }

    let mut ans = 0;
    let mut tmp = -1;
    let mut cnt = 0;
    for i in 0..n - 1 {
        if tmp == -1 {
            tmp = pre[i];
            cnt += 1;
            continue;
        }

        if tmp == pre[i] {
            cnt += 1;
        } else {
            ans += cnt * (cnt + 1) / 2;
            tmp = pre[i];
            cnt = 1;
        }
    }

    println!("{}", ans + cnt * (cnt + 1) / 2 + n as i64);
}
