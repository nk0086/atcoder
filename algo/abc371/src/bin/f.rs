use proconio::input;
use std::collections::{BTreeMap, HashSet};

fn main() {
    input! {
        n: usize,
        mut x: [i64; n], // 初期位置
        q: usize,
        t_g: [(usize, i64); q], // (T_i, G_i)
    }

    // 座標をキーとして、高橋くんのインデックスを保持するマップ
    let mut position_map = BTreeMap::new();
    for (i, &pos) in x.iter().enumerate() {
        position_map.insert(pos, i);
    }

    // 高橋くんの現在の位置を保持するベクタ
    let mut positions = x;

    // 合計移動回数
    let mut total_moves = 0;

    for &(t_i, g_i) in &t_g {
        let idx = t_i - 1; // インデックスは0始まり
        let s = positions[idx];

        if s == g_i {
            // 既に目的地にいる場合
            continue;
        }

        let direction = if g_i > s { 1 } else { -1 };

        // 指定された高橋くんを目的地に向けて移動
        let mut current_pos = s;
        while current_pos != g_i {
            let next_pos = current_pos + direction;
            // 移動先に誰かいるか確認
            if let Some(&other_idx) = position_map.get(&next_pos) {
                // 他の高橋くんがいる場合、その高橋くんを同じ方向に押す
                let moved = push(other_idx, direction, &mut position_map, &mut positions);
                total_moves += moved;
            }
            // 高橋くんを移動
            position_map.remove(&current_pos);
            position_map.insert(next_pos, idx);
            positions[idx] = next_pos;
            current_pos = next_pos;
            total_moves += 1;
        }
    }

    println!("{}", total_moves);
}

// 再帰的に高橋くんを押し出す関数
fn push(
    idx: usize,
    direction: i64,
    position_map: &mut BTreeMap<i64, usize>,
    positions: &mut Vec<i64>,
) -> i64 {
    let current_pos = positions[idx];
    let next_pos = current_pos + direction;
    let mut total_moves = 0;

    // 移動先に誰かいる場合は再帰的に押し出す
    if let Some(&other_idx) = position_map.get(&next_pos) {
        let moved = push(other_idx, direction, position_map, positions);
        total_moves += moved;
    }

    // 高橋くんを移動
    position_map.remove(&current_pos);
    position_map.insert(next_pos, idx);
    positions[idx] = next_pos;
    total_moves += 1;

    total_moves
}
