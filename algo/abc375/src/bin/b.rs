#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

fn main() {
    input! {
        n: usize,
        points: [(i64, i64); n],
    }

    let mut total_cost = 0f64;
    let mut current_x = 0i64;
    let mut current_y = 0i64;

    // 各点を順番に訪れる
    for &(x, y) in &points {
        total_cost += calculate_cost(current_x, current_y, x, y);
        current_x = x;
        current_y = y;
    }

    // 最後の点から原点に戻る
    total_cost += calculate_cost(current_x, current_y, 0, 0);

    println!("{}", total_cost);
}

fn calculate_cost(x1: i64, y1: i64, x2: i64, y2: i64) -> f64 {
    let dx = (x1 - x2) as f64;
    let dy = (y1 - y2) as f64;
    (dx * dx + dy * dy).sqrt()
}
