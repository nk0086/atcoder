#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
fn main() {
    input! {
        n: String,
    };

    let mut count1 = 0;
    let mut count2 = 0;
    let mut count3 = 0;

    for c in n.chars() {
        if c == '1' {
            count1 += 1;
        } else if c == '2' {
            count2 += 1;
        } else if c == '3' {
            count3 += 1;
        }
    }

    if count1 == 1 && count2 == 2 && count3 == 3 {
        println!("Yes");
    } else {
        println!("No");
    }
}
