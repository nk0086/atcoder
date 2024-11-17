#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
        q: usize,
        k: [usize; q],
    };

    let max_level = 60;
    let mut lengths = vec![0usize; max_level + 1];
    lengths[0] = s.len();
    for i in 1..=max_level {
        let prev = lengths[i - 1];
        if prev > 1e18 as usize {
            lengths[i] = 1usize << 60; // A large number to signify infinity
        } else {
            lengths[i] = prev.checked_mul(2).unwrap_or(1usize << 60);
        }
    }

    let mut results = Vec::with_capacity(q);
    for &query in &k {
        let mut idx = query;
        let mut level = max_level;
        let mut invert = false;

        while level > 0 {
            if lengths[level - 1] >= idx {
                level -= 1;
            } else {
                idx -= lengths[level - 1];
                invert = !invert;
                level -= 1;
            }
        }

        if idx <= s.len() {
            let mut ch = s[idx - 1];
            if invert {
                if ch.is_uppercase() {
                    ch = ch.to_lowercase().next().unwrap();
                } else {
                    ch = ch.to_uppercase().next().unwrap();
                }
            }
            results.push(ch);
        } else {
            results.push('?');
        }
    }

    for i in results.iter() {
        print!("{} ", i);
    }
}
