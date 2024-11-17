use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,k: usize,
        s: Chars,
    }

    let mut s = s;
    let mut count = 0;
    let mut i = 0;

    while i + k <= n {
        // Check if all K consecutive teeth are strong ('O')
        if s[i..i + k].iter().all(|&c| c == 'O') {
            count += 1;
            // Mark these K teeth as weak ('X') after eating a strawberry
            for j in i..i + k {
                s[j] = 'X';
            }
            // Move past this segment to avoid overlapping
            i += k;
        } else {
            i += 1;
        }
    }

    println!("{}", count);
}
