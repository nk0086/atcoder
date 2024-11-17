#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

fn main() {
    input! {
        s: Chars,
    };

    let n = s.len();
    let mut positions = vec![Vec::new(); 26]; // A-Z の各文字の位置を保存

    for (idx, &c) in s.iter().enumerate() {
        positions[(c as usize - 'A' as usize)].push(idx + 1); // 1-based index
    }

    let mut total = 0u64;

    for pos in positions.iter() {
        let m = pos.len();
        if m < 2 {
            continue;
        }

        // プレフィックスサムを計算
        let mut prefix_sum = vec![0usize; m + 1];
        for i in 0..m {
            prefix_sum[i + 1] = prefix_sum[i] + pos[i];
        }

        for i in 0..m - 1 {
            // i 番目の位置より後の全ての p_r を取得
            let sum_p_r = prefix_sum[m] - prefix_sum[i + 1];
            let count = (m - (i + 1)) as u64;

            // (p_r - p_l - 1) の合計
            total += sum_p_r as u64 - pos[i] as u64 * (m - i - 1) as u64 - (m - i - 1) as u64;
        }
    }

    println!("{}", total);
}
