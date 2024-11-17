use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    // 各値の最後の出現位置を記録するベクタ
    let mut last_occurrence = vec![0; n + 1];
    let mut total_sum: usize = 0;

    for i in 0..n {
        let value = a[i];
        let prev_index = last_occurrence[value];
        // 現在の位置での寄与を計算
        let contribution = (i + 1 - prev_index) * (n - i);
        total_sum += contribution;
        // 最後の出現位置を更新
        last_occurrence[value] = i + 1; // 1-based index に注意
    }

    println!("{}", total_sum);
}
