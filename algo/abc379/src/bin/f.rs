#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize, q: usize,
        h: [usize; n],
        lr: [(Usize1, Usize1); q],
    };

    // スタックを用いて各ビルから見える次のビルを計算
    let mut next_visible = vec![n; n];
    let mut stack: Vec<usize> = Vec::new();

    for i in 0..n {
        while let Some(&last) = stack.last() {
            if h[i] > h[last] {
                next_visible[last] = i;
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
    }

    // 各ビルから見えるビルのリストを作成
    let mut visible_from = vec![Vec::new(); n];
    for i in 0..n {
        if next_visible[i] < n {
            visible_from[i].push(next_visible[i]);
        }
    }

    // 各ビルから見えるビルを右から累積的に集める
    let mut suffix_counts = vec![0; n + 1];
    for i in (0..n).rev() {
        suffix_counts[i] = suffix_counts[i + 1];
        for &v in &visible_from[i] {
            suffix_counts[i] += 1;
            suffix_counts[v] -= 1;
        }
    }
    for i in (0..n).rev() {
        suffix_counts[i] += suffix_counts[i + 1];
    }

    // クエリに対する応答
    for &(l, r) in &lr {
        if r + 1 > n - 1 {
            println!("0");
            continue;
        }
        // ビルrの東側のビル数からlとrの両方から見えるビル数を計算
        println!("{}", suffix_counts[r + 1]);
    }
}
