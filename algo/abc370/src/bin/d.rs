#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
fn main() {
    input! {
        h: usize, w: usize, q: usize,
        rc: [(usize, usize); q]
    };

    let mut a = vec![vec![1; w]; h];
    let mut h_line = vec![(1..=h).collect::<Vec<usize>>(); w];
    let mut w_line = vec![(1..=w).collect::<Vec<usize>>(); h];

    for (r, c) in rc {
        println!("{:?}", a);
        println!("{:?}", h_line);
        println!("{:?}", w_line);
        println!("");
        if a[r - 1][c - 1] == 1 {
            a[r - 1][c - 1] = 0;
            h_line[c - 1].remove(r);
            w_line[r - 1].remove(c);
            continue;
        }

        if h_line[c - 1].len() != 0 {
            let (h_l, h_r) = binary_serach(r, &h_line[c - 1]);
            println!("{:?}", (h_l, h_r));
            a[h_l][c - 1] = 0;
            a[h_r][c - 1] = 0;
            h_line[c - 1].remove(h_r);
            h_line[c - 1].remove(h_l);
        }

        if w_line[r - 1].len() != 0 {
            let (w_l, w_r) = binary_serach(c, &w_line[r - 1]);
            println!("{:?}", (w_l, w_r));
            a[r - 1][w_l] = 0;
            a[r - 1][w_r] = 0;
            w_line[r - 1].remove(w_r);
            w_line[r - 1].remove(w_l);
        }
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            ans += a[i][j];
        }
    }

    println!("{}", ans);
}

fn binary_serach(n: usize, m: &Vec<usize>) -> (usize, usize) {
    let mut l = 0;
    let mut r = m.len() - 1;
    while r - l > 1 {
        let mid = (l + r) / 2;
        if m[mid] < n {
            l = mid;
        } else {
            r = mid;
        }
    }

    (m[l] - 1, m[r] - 1)
}
