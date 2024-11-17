use std::collections::BTreeSet;

struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
    members: Vec<BTreeSet<usize>>, // 各連結成分の頂点番号を保持
}

impl DSU {
    fn new(n: usize) -> Self {
        let mut members = Vec::with_capacity(n + 1);
        for i in 0..=n {
            let mut set = BTreeSet::new();
            set.insert(i);
            members.push(set);
        }
        DSU {
            parent: (0..=n).collect(),
            size: vec![1; n + 1],
            members,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let fx = self.find(x);
        let fy = self.find(y);
        if fx == fy {
            return;
        }

        // Union by size: fxが小さい場合、fyにマージ
        if self.size[fx] < self.size[fy] {
            self.parent[fx] = fy;
            self.size[fy] += self.size[fx];

            // 一時的なベクターにコピー
            let temp: Vec<usize> = self.members[fx].iter().cloned().collect();
            for val in temp {
                self.members[fy].insert(val);
            }
            // 必要に応じてfxのメンバーをクリア
            self.members[fx].clear();
        } else {
            self.parent[fy] = fx;
            self.size[fx] += self.size[fy];

            let temp: Vec<usize> = self.members[fy].iter().cloned().collect();
            for val in temp {
                self.members[fx].insert(val);
            }
            self.members[fy].clear();
        }
    }
}

fn main() {
    use proconio::input;
    input! {
        n: usize,
        q: usize,
        queries: [(usize, usize, usize); q],
    }

    let mut dsu = DSU::new(n);

    for query in queries {
        if query.0 == 1 {
            // タイプ1: 1 u v
            let u = query.1;
            let v = query.2;
            dsu.union(u, v);
        } else {
            // タイプ2: 2 v k
            let v = query.1;
            let k = query.2;
            let fv = dsu.find(v);
            if dsu.size[fv] < k {
                println!("-1");
            } else {
                // BTreeSetはソートされているので、逆イテレータを使ってk番目を取得
                let mut iter = dsu.members[fv].iter().rev();
                let mut ans = -1;
                for _ in 0..k {
                    if let Some(&val) = iter.next() {
                        ans = val as isize;
                    } else {
                        ans = -1;
                        break;
                    }
                }
                println!("{}", ans);
            }
        }
    }
}
