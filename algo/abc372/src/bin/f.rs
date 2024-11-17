#[allow(unused_imports)]
use proconio::{fastout, input, marker::Usize1};

// 定数MODを定義
const MOD: usize = 998244353;

// 型エイリアス: n x n 行列
type Matrix = Vec<Vec<usize>>;

// 行列の乗算を行う関数
fn multiply(a: &Matrix, b: &Matrix, n: usize) -> Matrix {
    let mut result = vec![vec![0usize; n]; n];
    for i in 0..n {
        for k in 0..n {
            if a[i][k] == 0 {
                continue;
            }
            for j in 0..n {
                result[i][j] = (result[i][j] + a[i][k] * b[k][j]) % MOD;
            }
        }
    }
    result
}

// 行列の累乗を行う関数 (二分累乗)
fn power(mut a: Matrix, mut exponent: usize, n: usize) -> Matrix {
    // 単位行列を初期化
    let mut result = vec![vec![0usize; n]; n];
    for i in 0..n {
        result[i][i] = 1;
    }

    while exponent > 0 {
        if exponent % 2 == 1 {
            result = multiply(&result, &a, n);
        }
        a = multiply(&a, &a, n);
        exponent /= 2;
    }

    result
}

#[fastout]
fn main() {
    input! {
        n: usize, // 頂点数
        m: usize, // 追加の有向辺の数
        k: usize, // 移動回数
        xy: [(Usize1, Usize1); m] // 追加の有向辺のリスト (0-based)
    };

    // 遷移行列を初期化
    let mut adj = vec![vec![0usize; n]; n];

    // 基本サイクルの有向辺を追加
    for i in 0..n {
        let j = (i + 1) % n;
        adj[i][j] += 1;
    }

    // 追加の有向辺を追加
    for &(x, y) in &xy {
        adj[x][y] = (adj[x][y] + 1) % MOD;
    }

    // 行列Aをk乗
    let a_k = power(adj, k, n);

    // 頂点1 (0-indexed) から出発するので、行列の0行目を参照
    let total = a_k[0].iter().fold(0usize, |acc, &x| (acc + x) % MOD);

    // 結果を出力
    println!("{}", total);
}
