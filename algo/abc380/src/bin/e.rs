use proconio::{fastout, input};

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    color: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize, initial_colors: Vec<usize>) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
            color: initial_colors,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> usize {
        let fx = self.find(x);
        let fy = self.find(y);
        if fx == fy {
            return fx;
        }
        // Merge smaller group into larger group
        if self.size[fx] < self.size[fy] {
            self.parent[fx] = fy;
            self.size[fy] += self.size[fx];
            fy
        } else {
            self.parent[fy] = fx;
            self.size[fx] += self.size[fy];
            fx
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let initial_colors: Vec<usize> = (1..=n).collect();
    let mut uf = UnionFind::new(n, initial_colors.clone());

    let mut color_count = vec![0; n + 1];
    for &c in &initial_colors {
        color_count[c] += 1;
    }

    for _ in 0..q {
        input! {
            query_type: usize,
        }

        if query_type == 1 {
            input! {
                x: usize,
                c: usize,
            }
            let x_idx = x - 1;
            let root = uf.find(x_idx);
            let current_color = uf.color[root];

            if current_color != c {
                color_count[current_color] -= uf.size[root];
                color_count[c] += uf.size[root];

                uf.color[root] = c;

                if x_idx > 0 {
                    let left = x_idx - 1;
                    let left_root = uf.find(left);
                    if uf.color[left_root] == c {
                        let new_root = uf.union(root, left_root);
                        uf.color[new_root] = c;
                    }
                }

                if x_idx + 1 < n {
                    let right = x_idx + 1;
                    let right_root = uf.find(right);
                    if uf.color[right_root] == c {
                        let new_root = uf.union(root, right_root);
                        uf.color[new_root] = c;
                    }
                }
            }
        } else if query_type == 2 {
            input! {
                c: usize,
            }
            if c <= n {
                println!("{}", color_count[c]);
            } else {
                println!("0");
            }
        }
    }
}
