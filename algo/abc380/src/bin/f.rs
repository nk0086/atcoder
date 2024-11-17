#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
fn main() {
    input! {

    };
}


#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        q: usize,
        k: [usize; q],
    };

    let s_length = s.len();
    let mut result = Vec::new();
    for ki in k {
        let tmp = (ki + s_length - 1) / s_length;
        let tmp_mod = ki % s_length;
        let tmp_index = if tmp_mod == 0 {
            s_length - 1
        } else {
            tmp_mod - 1
        };

        if tmp % 2 != 0 {
            result.push(s[tmp_index]);
        } else {
            if s[tmp_index].is_lowercase() {
                result.push(s[tmp_index].to_ascii_uppercase());
            } else {
                result.push(s[tmp_index].to_ascii_lowercase());
            }
        }
    }

    for r in result {
        print!("{} ", r);
    }
}
