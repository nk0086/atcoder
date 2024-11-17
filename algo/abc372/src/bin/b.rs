#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::cmp::min;

#[fastout]
fn main() {
    input! {
        m: usize
    };

    // 3^0から3^10までの値を事前に計算
    let powers: Vec<usize> = (0..=10).map(|i| 3_usize.pow(i as u32)).collect();

    // dp[i] = i を作るための最小の項数 j
    let mut dp = vec![std::usize::MAX; m + 1];
    dp[0] = 0;

    // prev[i] = i を作るために最後に使った power のインデックス
    let mut prev = vec![None; m + 1];

    for (a, &power) in powers.iter().enumerate() {
        for i in 0..=m {
            if dp[i] != std::usize::MAX && i + power <= m && dp[i + power] > dp[i] + 1 {
                dp[i + power] = dp[i] + 1;
                prev[i + power] = Some(a); // `a` は A_i の値
            }
        }
    }

    // dp[m] が存在することは問題文で保証されています
    let n = dp[m];
    println!("{}", n);

    // A の列を復元
    let mut a_list = Vec::new();
    let mut current = m;
    while current > 0 {
        if let Some(a) = prev[current] {
            a_list.push(a);
            current -= powers[a];
        } else {
            // ここに来ることはないはず
            break;
        }
    }

    // 逆順にして出力
    a_list.reverse();
    for a in a_list {
        print!("{} ", a);
    }
    println!();
}
