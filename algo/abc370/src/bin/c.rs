#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
fn main() {
    input! {
        mut s: Chars,
        t: Chars,
    };

    let mut d = vec![];
    for i in 0..s.len() {
        if s[i] > t[i] {
            d.push((0, i));
        } else if s[i] < t[i] {
            d.push((s.len() + 5 - i, i));
        }
    }

    d.sort();

    let mut x = Vec::new();
    for (_, i) in d {
        if s[i] != t[i] {
            s[i] = t[i];
            x.push(s.clone());
        }
    }

    println!("{}", x.len());
    for i in x {
        println!("{}", i.iter().collect::<String>());
    }
}
