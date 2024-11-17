#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
fn main() {
    input! {
        n: usize,
        mg: usize,
        uv: [(Usize1, Usize1); mg],
        mh: usize,
        ab: [(Usize1, Usize1); mh],
    };

    // グラフGの隣接行列を作成
    let mut adj_g = vec![vec![false; n]; n];
    for &(u, v) in &uv {
        adj_g[u][v] = true;
        adj_g[v][u] = true;
    }

    // グラフHの隣接行列を作成
    let mut adj_h = vec![vec![false; n]; n];
    for &(a, b) in &ab {
        adj_h[a][b] = true;
        adj_h[b][a] = true;
    }

    // コスト行列Aを読み込む
    let mut cost = vec![vec![0; n]; n];
    for i in 0..n - 1 {
        input! { ai: [usize; n - i - 1] };
        for (j, &c) in ai.iter().enumerate() {
            let j_idx = i + j + 1;
            cost[i][j_idx] = c;
            cost[j_idx][i] = c; // 対称性を利用
        }
    }

    let mut min_total_cost = std::usize::MAX;

    // 頂点の置換を生成し、最小コストを計算
    let mut permutation = vec![0; n];
    let mut used = vec![false; n];
    dfs_permutation(
        0,
        &mut permutation,
        &mut used,
        &adj_g,
        &adj_h,
        &cost,
        &mut min_total_cost,
    );

    println!("{}", min_total_cost);
}

// 再帰的に全ての置換を生成し、最小コストを計算
fn dfs_permutation(
    pos: usize,
    permutation: &mut Vec<usize>,
    used: &mut Vec<bool>,
    adj_g: &Vec<Vec<bool>>,
    adj_h: &Vec<Vec<bool>>,
    cost: &Vec<Vec<usize>>,
    min_total_cost: &mut usize,
) {
    let n = adj_g.len();
    if pos == n {
        // 完全な置換が生成されたので、コストを計算
        let mut inv_permutation = vec![0; n];
        for i in 0..n {
            inv_permutation[permutation[i]] = i;
        }

        let mut total_cost = 0;
        for i in 0..n {
            for j in (i + 1)..n {
                let edge_g = adj_g[i][j];
                let edge_h = adj_h[inv_permutation[i]][inv_permutation[j]];
                if edge_g != edge_h {
                    // エッジを反転させる必要があるのでコストを加算
                    total_cost += cost[inv_permutation[i]][inv_permutation[j]];
                }
            }
        }

        if total_cost < *min_total_cost {
            *min_total_cost = total_cost;
        }

        return;
    }

    for i in 0..n {
        if !used[i] {
            used[i] = true;
            permutation[pos] = i;
            dfs_permutation(
                pos + 1,
                permutation,
                used,
                adj_g,
                adj_h,
                cost,
                min_total_cost,
            );
            used[i] = false;
        }
    }
}
