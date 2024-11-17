#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::time::{Duration, Instant};

// 点の構造体
#[derive(Debug, Clone)]
struct Point {
    a: usize,
    b: usize,
    index: usize, // 元のインデックス
}

#[fastout]
fn main() {
    input! {
        n: usize, // n = 1000
        mut ab: [(usize, usize); n]
    }

    // 点のリストを構築し、元のインデックスを保持
    let mut points: Vec<Point> = ab
        .into_iter()
        .enumerate()
        .map(|(i, (a, b))| Point { a, b, index: i })
        .collect();

    // 現在時刻を取得
    let start_time = Instant::now();
    // 許容時間を設定（例：1.9秒）
    let time_limit = Duration::from_millis(1900);

    // グラフの構築
    let mut adj = build_graph(n, &mut points, start_time, time_limit);

    // 1 ~ n から2点を選び、中間点を追加する
    for i in 0..n {
        for j in i + 1..n {
            if !add_middle_point(&mut adj, i, j, &mut points) {
                break;
            }
        }
    }

    // グラフの巡回
    let moves = traverse_graph(n, &points, &adj);

    // 出力
    println!("{}", moves.len());
    for ((x1, y1), (x2, y2)) in moves.iter() {
        println!("{} {} {} {}", x1, y1, x2, y2);
    }
}

// 2点指定して、中間点を追加する関数
fn add_middle_point(graph: &mut Vec<Vec<usize>>, a: usize, b: usize, ab: &mut Vec<Point>) -> bool {
    let A = &ab[a];
    let B = &ab[b];

    let middle = (A.a.min(B.a), A.b.min(B.b));
    let middle_index = ab.len();

    let middl_point = Point {
        a: middle.0,
        b: middle.1,
        index: middle_index,
    };

    ab.push(middl_point);
    // graph にも追加
    graph[a].push(middle_index);
    graph[b].push(middle_index);

    graph.push(vec![a, b]);

    if ab.len() == 5000 {
        return false;
    }

    true
}

/// 木グラフを構築する関数
/// (0, 0) を仮想的なルートとし、各点を親子関係で接続します。
/// 許容時間を超えた場合、途中で計算を中断します。
fn build_graph(
    n: usize,
    points: &mut Vec<Point>,
    start_time: Instant,
    time_limit: Duration,
) -> Vec<Vec<usize>> {
    // (0,0) を仮想的な点として扱うため、インデックスを n とする
    let virtual_index = n;

    // 点をソート: aの昇順、aが同じ場合はbの昇順
    points.sort_unstable_by(|p1, p2| {
        if p1.a != p2.a {
            p1.a.cmp(&p2.a)
        } else {
            p1.b.cmp(&p2.b)
        }
    });

    // 各点の親を保持するベクター。初期値は仮想点 (0,0) のインデックス
    let mut parent: Vec<usize> = vec![virtual_index; n];

    // ソートされた点のリストにアクセスしやすくする
    for i in 0..n {
        // 時間制約をチェック
        if start_time.elapsed() > time_limit {
            eprintln!("時間制約に達したため、グラフの構築を中断します。");
            break;
        }

        let current = &points[i];
        // 最適な親を探す
        // 最も a_j が大きく、かつ b_j が小さい点を親とする
        let mut best_parent = virtual_index;
        let mut min_cost = current.a + current.b; // (0,0) からのコスト

        for j in 0..i {
            let candidate = &points[j];
            if candidate.a <= current.a && candidate.b <= current.b {
                let cost = (current.a - candidate.a) + (current.b - candidate.b);
                if cost < min_cost {
                    best_parent = candidate.index;
                    min_cost = cost;
                }
            }
        }

        // 親を割り当てる
        if best_parent != virtual_index {
            parent[current.index] = best_parent;
        }
    }

    // 木グラフの隣接リストを構築
    // 仮想点のインデックスは n とする
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    for i in 0..n {
        let p = parent[i];
        if p != virtual_index {
            adj[p].push(i);
        } else {
            adj[virtual_index].push(i);
        }
    }

    adj
}

/// 木グラフを巡回し、移動ステップを記録する関数
/// DFSを用いて親から子へ移動し、移動ステップを記録します。
fn traverse_graph(
    n: usize,
    points: &Vec<Point>,
    adj: &Vec<Vec<usize>>,
) -> Vec<((usize, usize), (usize, usize))> {
    let virtual_index = n;
    let mut moves: Vec<((usize, usize), (usize, usize))> = Vec::with_capacity(n);

    // 点の座標をインデックスで参照できるようにする
    let mut index_to_point: Vec<(usize, usize)> = vec![(0, 0); n + 1];
    index_to_point[virtual_index] = (0, 0); // 仮想点 (0,0)
    for p in points.iter() {
        index_to_point[p.index] = (p.a, p.b);
    }

    // DFSスタック: (現在の点のインデックス)
    let mut stack: Vec<usize> = Vec::new();
    stack.push(virtual_index);

    while let Some(current) = stack.pop() {
        // 子点を逆順に追加しておくと、DFSで先に追加された子が先に処理される
        for &child in adj[current].iter().rev() {
            // 現在の点から子点への移動を記録
            let from = index_to_point[current];
            let to = index_to_point[child];
            moves.push((from, to));

            // 子点をスタックに追加
            stack.push(child);
        }
    }

    moves
}
