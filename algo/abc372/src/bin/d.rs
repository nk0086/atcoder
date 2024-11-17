use proconio::input;
use std::io::{self, BufWriter, Write};

fn main() {
    // 高速な出力のためにバッファを使用
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    // 入力の読み込み
    input! {
        n: usize,
        h: [usize; n],
    }

    // p[j] を求めるためのスタック（1-based index）
    let mut stack: Vec<usize> = Vec::new();
    // p[j] を格納するベクター（1-based index）
    let mut p = vec![0; n + 1]; // p[1..n] を使用

    for j in 1..=n {
        // 現在のビルよりも低いビルをスタックからポップ
        while let Some(&last) = stack.last() {
            if h[last - 1] <= h[j - 1] {
                stack.pop();
            } else {
                break;
            }
        }
        // スタックが空でなければ p[j] はスタックのトップ
        if let Some(&last) = stack.last() {
            p[j] = last;
        } else {
            p[j] = 0;
        }
        // 現在のビルをスタックにプッシュ
        stack.push(j);
    }

    // 差分配列の初期化
    // 1-based index, delta[1..n+1]
    let mut delta = vec![0i64; n + 2];

    for j in 1..=n {
        let pj = p[j];
        // [p[j], j -1] を更新
        let left = if pj == 0 { 1 } else { pj };
        let right = j - 1;
        if left <= right {
            delta[left] += 1;
            delta[right + 1] -= 1;
        }
    }

    // 累積和を計算して counts を求める
    let mut counts = vec![0i64; n + 1]; // counts[1..n] を使用
    let mut current = 0i64;
    for i in 1..=n {
        current += delta[i];
        counts[i] = current;
    }

    // 結果の出力
    for i in 1..=n {
        write!(out, "{}{}", counts[i], if i < n { ' ' } else { '\n' }).unwrap();
    }
}
