#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars},
};

#[fastout]
fn main() {
    input! {
        s: Chars,
    };

    let a = parse_s(&s);
    println!("{}", a.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}

fn parse_s(s: &[char]) -> Vec<usize> {
    let mut a = Vec::new();
    let mut count = 0;
    for &c in s.iter().skip(1) {
        if c == '-' {
            count += 1;
        } else if c == '|' {
            a.push(count);
            count = 0;
        }
    }
    a
}
