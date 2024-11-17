use nalgebra::min;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    }

    let mut grid = a.clone();

    for i in 0..n {
        for j in 0..n {
            let d = min(i, j)+ 1;
            let e = min(n - i, n - j);
            let num = min(d, e);

            let mut ni = i;
            let mut nj = j;
            for k in 0..num % 4 {
                let tmp_i = nj;
                let tmp_j = n - 1 - ni;
                ni = tmp_i;
                nj = tmp_j;
            }

            grid[ni][nj] = a[i][j];
        }
    }

    // Print the final grid
    for row in grid {
        let s: String = row.into_iter().collect();
        println!("{}", s);
    }
}
