#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n]
    };

    /*
     * dp[i][0] := モンスターを倒した数が奇数の時, i番目を倒す時の最大
     * dp[i][1] := モンスターを倒した数が偶数の時, i番目を倒す時の最大
     * dp[i][2] := モンスターを倒した数が奇数の時, i番目を倒さない時の最大
     * dp[i][3] := モンスターを倒した数が偶数の時, i番目を倒さない時の最大
     */
    let mut dp = vec![vec![0; 4]; n];
    dp[0][0] = a[0];

    for i in 1..n {
        let tmp = if dp[i - 1][2] == 0 {
            -100000000000
        } else {
            dp[i - 1][2]
        };
        dp[i][0] = (dp[i - 1][1] + a[i]).max(dp[i - 1][3] + a[i]).max(dp[i][0]);
        dp[i][1] = (dp[i - 1][0] + a[i] * 2).max(tmp + a[i] * 2).max(dp[i][1]);

        dp[i][2] = dp[i - 1][0].max(tmp).max(dp[i][2]);
        dp[i][3] = dp[i - 1][1].max(dp[i - 1][3]).max(dp[i][3]);
    }

    // println!("{:?}", dp);

    // dp[n - 1]の最大
    let ans = dp[n - 1].iter().max().unwrap();
    println!("{}", ans);
}
