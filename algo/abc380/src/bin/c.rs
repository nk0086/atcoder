use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        s: Chars,
    }

    let mut blocks = vec![];
    let mut i = 0;
    while i < n {
        if s[i] == '1' {
            let start = i;
            while i < n && s[i] == '1' {
                i += 1;
            }
            let end = i;
            blocks.push((start, end));
        } else {
            i += 1;
        }
    }

    if k == 1 {
        println!("{}", s.iter().collect::<String>());
        return;
    }

    let (_prev_start, prev_end) = blocks[k - 2];
    let (k_start, k_end) = blocks[k - 1];

    let mut result = Vec::with_capacity(n);
    result.extend_from_slice(&s[..prev_end]);
    result.extend_from_slice(&s[k_start..k_end]);
    result.extend_from_slice(&s[prev_end..k_start]);
    result.extend_from_slice(&s[k_end..]);

    println!("{}", result.iter().collect::<String>());
}
