use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        ab: [(usize, i64); n],
    }

    // 各チームの強さを計算
    let mut team_strength = vec![0; 4]; // チーム1,2,3のインデックスを1,2,3とする
    for (a, b) in &ab {
        team_strength[*a] += *b;
    }

    let total_strength: i64 = team_strength.iter().sum();
    if total_strength % 3 != 0 {
        println!("-1");
        return;
    }

    let target = total_strength / 3;
    let mut differences = vec![0; 4];
    for i in 1..=3 {
        differences[i] = target - team_strength[i];
    }

    // 必要な各チームの変化量
    // 差が正なら増加、負なら減少
    let d1 = differences[1];
    let d2 = differences[2];
    let d3 = differences[3];

    // 最大強さが500まで (1500 / 3)
    const MAX: usize = 1501;
    let mut dp = vec![vec![std::i32::MAX; (target as usize) + 1]; (target as usize) + 1];
    dp[0][0] = 0;

    for &(team, strength) in &ab {
        // 新しいDPテーブルを作成
        let mut new_dp = vec![vec![std::i32::MAX; (target as usize) + 1]; (target as usize) + 1];
        for sum1 in 0..=(target as usize) {
            for sum2 in 0..=(target as usize) {
                if dp[sum1][sum2] == std::i32::MAX {
                    continue;
                }
                // チーム1に割り当てる
                let new_sum1 = sum1 + (if 1 == team { strength } else { 0 }) as usize;
                if new_sum1 <= target as usize {
                    let change = if 1 != team { 1 } else { 0 };
                    new_dp[new_sum1][sum2] = min(new_dp[new_sum1][sum2], dp[sum1][sum2] + change);
                }
                // チーム2に割り当てる
                let new_sum2 = sum2 + (if 2 == team { strength } else { 0 }) as usize;
                if new_sum2 <= target as usize {
                    let change = if 2 != team { 1 } else { 0 };
                    new_dp[sum1][new_sum2] = min(new_dp[sum1][new_sum2], dp[sum1][sum2] + change);
                }
                // チーム3に割り当てる
                // チーム3の割り当てはsum1, sum2には影響しない
                let change = if 3 != team { 1 } else { 0 };
                new_dp[sum1][sum2] = min(new_dp[sum1][sum2], dp[sum1][sum2] + change);
            }
        }
        dp = new_dp;
    }

    // 最終的に sum1 = target, sum2 = target を目指す
    let result = dp[target as usize][target as usize];
    if result != std::i32::MAX {
        println!("{}", result);
    } else {
        println!("-1");
    }
}
