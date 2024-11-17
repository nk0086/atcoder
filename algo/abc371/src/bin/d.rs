#[allow(unused_imports)]
use proconio::{fastout, input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        x: [i64; n],
        p: [u64; n],
        q: usize,
        lr: [(i64, i64); q],
    }

    // 村の座標と人口を結合してソート（座標は既にソート済みであると仮定）
    let x_list = x;
    let p_list = p;

    // 累積人口を計算
    let mut cum_p = vec![0u64; n + 1];
    for i in 0..n {
        cum_p[i + 1] = cum_p[i] + p_list[i];
    }

    // クエリを処理
    for &(l_i, r_i) in &lr {
        // 左端のインデックスを二分探索で見つける (X_i >= L_i)
        let left = x_list.partition_point(|&x| x < l_i);

        // 右端のインデックスを二分探索で見つける (X_i <= R_i)
        let right = x_list.partition_point(|&x| x <= r_i);

        if left >= right {
            // 該当する村がない場合
            println!("0");
        } else {
            // 累積和を使用して人口の合計を計算
            let total_p = cum_p[right] - cum_p[left];
            println!("{}", total_p);
        }
    }
}
