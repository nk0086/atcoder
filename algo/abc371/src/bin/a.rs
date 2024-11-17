#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
fn main() {
    input! {
        s_ab: String,
        s_ac: String,
        s_bc: String,
    }

    let s_ab = s_ab.trim();
    let s_ac = s_ac.trim();
    let s_bc = s_bc.trim();

    // 年齢順のリストを作成
    let mut brothers = vec!['A', 'B', 'C'];

    // 関係に基づいてソート
    brothers.sort_by(|&x, &y| {
        let relation = match (x, y) {
            ('A', 'B') => s_ab,
            ('B', 'A') => invert(s_ab),
            ('A', 'C') => s_ac,
            ('C', 'A') => invert(s_ac),
            ('B', 'C') => s_bc,
            ('C', 'B') => invert(s_bc),
            _ => "=",
        };
        match relation {
            ">" => std::cmp::Ordering::Less,    // x が年上
            "<" => std::cmp::Ordering::Greater, // y が年上
            _ => std::cmp::Ordering::Equal,
        }
    });

    // 二番目に年上の兄弟を出力
    println!("{}", brothers[1]);
}

fn invert(relation: &str) -> &str {
    match relation {
        ">" => "<",
        "<" => ">",
        _ => relation,
    }
}
