#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
fn main() {
    input! {
        n: usize, q: usize,
        mut s: Chars,
        xc: [(usize, char); q]
    };

    // 初期の "ABC" のカウントを計算
    let mut count = 0;
    for i in 0..n - 2 {
        if s[i] == 'A' && s[i + 1] == 'B' && s[i + 2] == 'C' {
            count += 1;
        }
    }

    // 各クエリを処理
    for &(x, c) in &xc {
        // 影響を受ける可能性のある開始位置
        // 1-based to 0-based
        let pos = x - 1;

        // チェックする開始位置は pos-2, pos-1, pos
        let mut affected_positions = Vec::new();
        if pos >= 2 && pos <= n - 1 {
            affected_positions.push(pos - 2);
        }
        if pos >= 1 && pos <= n - 2 {
            affected_positions.push(pos - 1);
        }
        if pos <= n - 3 {
            affected_positions.push(pos);
        }

        // 変更前に "ABC" であった箇所をカウントから減らす
        for &i in &affected_positions {
            if s[i] == 'A' && s[i + 1] == 'B' && s[i + 2] == 'C' {
                count -= 1;
            }
        }

        // 文字を変更
        s[pos] = c;

        // 変更後に "ABC" である箇所をカウントに加える
        for &i in &affected_positions {
            if s[i] == 'A' && s[i + 1] == 'B' && s[i + 2] == 'C' {
                count += 1;
            }
        }

        // 現在の "ABC" の総数を出力
        println!("{}", count);
    }
}
