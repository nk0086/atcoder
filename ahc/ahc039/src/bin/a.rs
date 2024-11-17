use rand::prelude::*;
use proconio::{fastout, input};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Edge {
    start: Point,
    end: Point,
}

#[derive(Debug, Clone)]
struct Polygon {
    vertices: Vec<Point>,
    edges: Vec<Edge>,
}

impl Polygon {
    // 多角形の初期化
    fn new(vertices: Vec<Point>) -> Self {
        let edges = Polygon::create_edges(&vertices);
        Polygon { vertices, edges }
    }

    // 頂点から辺を生成
    fn create_edges(vertices: &Vec<Point>) -> Vec<Edge> {
        let mut edges = Vec::new();
        let n = vertices.len();
        for i in 0..n {
            let start = vertices[i].clone();
            let end = vertices[(i + 1) % n].clone();
            edges.push(Edge { start, end });
        }
        edges
    }

    // 辺を更新
    fn update_edges(&mut self) {
        self.edges = Polygon::create_edges(&self.vertices);
    }

    // ランダムに辺を選択
    fn select_random_edge(&self) -> Option<usize> {
        if self.edges.is_empty() {
            return None;
        }
        let mut rng = thread_rng();
        let idx = rng.gen_range(0,self.edges.len());
        Some(idx)
    }

    // 垂線を降ろし交点を求める
    fn drop_perpendicular(&self, edge: &Edge, point: &Point) -> Option<Point> {
        let (x1, y1) = (edge.start.x, edge.start.y);
        let (x2, y2) = (edge.end.x, edge.end.y);

        if x1 == x2 {
            // 垂直な辺の場合、垂線は水平
            Some(Point { x: point.x, y: y1 })
        } else if y1 == y2 {
            // 水平な辺の場合、垂線は垂直
            Some(Point { x: x1, y: point.y })
        } else {
            // 軸に平行でない場合は未対応
            None
        }
    }

    // 辺の変更手順
    fn change_edge(&mut self, edge_idx: usize, new_point1: Point, new_point2: Point) {
        if edge_idx >= self.edges.len() {
            return;
        }

        let edge = &self.edges[edge_idx];

        // 垂線降下を行い交点を求める
        let perp_point1 = match self.drop_perpendicular(edge, &new_point1) {
            Some(p) => p,
            None => return,
        };
        let perp_point2 = match self.drop_perpendicular(edge, &new_point2) {
            Some(p) => p,
            None => return,
        };

        // 多角形の頂点を更新
        self.vertices.insert(edge_idx + 1, perp_point1.clone());
        self.vertices.insert(edge_idx + 2, perp_point2.clone());

        // 辺を更新
        self.update_edges();
    }

    // 多角形を変更するための統合関数
    fn modify_polygon(&mut self) -> bool {
        // 辺をランダムに選択
        let edge_idx = match self.select_random_edge() {
            Some(idx) => idx,
            None => return false,
        };

        // 新しい2点を決定（ここではランダムに選択）
        let mut rng = thread_rng();
        let new_x1 = rng.gen_range(0,100_000);
        let new_y1 = rng.gen_range(0,100_000);
        let new_x2 = rng.gen_range(0,100_000);
        let new_y2 = rng.gen_range(0,100_000);
        let new_point1 = Point { x: new_x1, y: new_y1 };
        let new_point2 = Point { x: new_x2, y: new_y2 };

        // 辺の変更を試みる
        self.change_edge(edge_idx, new_point1, new_point2);
        true
    }

    // 総辺長を計算
    fn total_edge_length(&self) -> usize {
        let mut total = 0;
        for edge in &self.edges {
            total += (edge.start.x as isize - edge.end.x as isize).abs() as usize +
                     (edge.start.y as isize - edge.end.y as isize).abs() as usize;
        }
        total
    }

    // 制約をチェック
    fn check_constraints(&self) -> bool {
        self.vertices.len() <= 1000 && self.total_edge_length() <= 400_000
    }

    // 制約を満たすように多角形を簡略化
    fn enforce_constraints(&mut self) {
        // 頂点数が多い場合、頂点を削減
        while self.vertices.len() > 1000 {
            // 最もスコアに影響の少ない頂点を削除（ここでは単純に削減）
            self.vertices.pop();
            self.update_edges();
        }

        // 辺の総長が制約を超える場合、辺を短くする
        while self.total_edge_length() > 400_000 && self.edges.len() > 4 {
            // 最も長い辺を見つけて分割
            let mut max_len = 0;
            let mut max_idx = 0;
            for (idx, edge) in self.edges.iter().enumerate() {
                let len = (edge.start.x as isize - edge.end.x as isize).abs() +
                          (edge.start.y as isize - edge.end.y as isize).abs();
                if len > max_len as isize {
                    max_len = len as usize;
                    max_idx = idx;
                }
            }

            // 辺を2分割
            let edge = &self.edges[max_idx];
            let mid_x = (edge.start.x + edge.end.x) / 2;
            let mid_y = (edge.start.y + edge.end.y) / 2;
            let mid_point = Point { x: mid_x, y: mid_y };

            // 点を挿入
            self.vertices.insert(max_idx + 1, mid_point.clone());
            self.update_edges();
        }
    }
}

struct ScoreMap {
    saba: HashSet<(usize, usize)>,
    iwashi: HashSet<(usize, usize)>,
}

impl ScoreMap {
    fn new(sxy: Vec<(usize, usize)>, ixy: Vec<(usize, usize)>) -> Self {
        let saba: HashSet<(usize, usize)> = sxy.into_iter().collect();
        let iwashi: HashSet<(usize, usize)> = ixy.into_iter().collect();
        ScoreMap { saba, iwashi }
    }

    // 点が多角形内部に含まれるか判定（辺上も内部とみなす）
    fn is_inside(&self, point: &Point, polygon: &Polygon) -> bool {
        let mut inside = false;
        let n = polygon.vertices.len();

        for i in 0..n {
            let a = &polygon.vertices[i];
            let b = &polygon.vertices[(i + 1) % n];

            if self.on_segment(a, b, point) {
                return true;
            }

            // Ray Casting Algorithm
            if (a.y > point.y) != (b.y > point.y) {
                // 計算を整数で行い、オーバーフローを避けるためにスケール調整を検討
                let slope = (b.x as isize - a.x as isize) as f64 / (b.y as isize - a.y as isize) as f64;
                let intersect_x = slope * (point.y as isize - a.y as isize) as f64 + a.x as isize as f64;
                if intersect_x as usize > point.x {
                    inside = !inside;
                }
            }
        }

        inside
    }

    // 点が線分上にあるか判定
    fn on_segment(&self, a: &Point, b: &Point, p: &Point) -> bool {
        let cross = (b.x as isize - a.x as isize) * (p.y as isize - a.y as isize) -
                    (b.y as isize - a.y as isize) * (p.x as isize - a.x as isize);
        if cross != 0 {
            return false;
        }

        let dot = (p.x as isize - a.x as isize) * (p.x as isize - b.x as isize) +
                  (p.y as isize - a.y as isize) * (p.y as isize - b.y as isize);
        dot <= 0
    }

    // スコアの計算
    fn calculate_score(&self, polygon: &Polygon) -> i32 {
        let mut score = 0;
        for &(x, y) in &self.saba {
            if self.is_inside(&Point { x, y }, polygon) {
                score += 1;
            }
        }
        for &(x, y) in &self.iwashi {
            if self.is_inside(&Point { x, y }, polygon) {
                score -= 1;
            }
        }
        score
    }
}

struct AnnealingParams {
    initial_temp: f64,
    final_temp: f64,
    alpha: f64, // 冷却率
    iterations: usize,
}

impl AnnealingParams {
    fn new(initial_temp: f64, final_temp: f64, alpha: f64, iterations: usize) -> Self {
        AnnealingParams { initial_temp, final_temp, alpha, iterations }
    }
}

fn simulated_annealing(polygon: &mut Polygon, score_map: &ScoreMap, params: AnnealingParams) {
    let mut rng = thread_rng();
    let mut current_score = score_map.calculate_score(polygon);
    let mut best_score = current_score;
    let mut best_polygon = polygon.clone();
    let mut temp = params.initial_temp;

    for _ in 0..params.iterations {
        if temp < params.final_temp {
            break;
        }

        // 多角形を変更
        let old_polygon = polygon.clone();
        let changed = polygon.modify_polygon();

        if !changed {
            continue;
        }

        // スコアを再計算
        let new_score = score_map.calculate_score(polygon);

        // スコアの改善を評価
        if new_score > current_score {
            current_score = new_score;
            if new_score > best_score {
                best_score = new_score;
                best_polygon = polygon.clone();
            }
        } else {
            // 確率的に受け入れ
            let delta = current_score - new_score;
            let probability = (-delta as f64 / temp).exp();
            let roll: f64 = rng.gen();
            if roll < probability {
                current_score = new_score;
            } else {
                // 変更を元に戻す
                *polygon = old_polygon;
            }
        }

        // 温度を下げる
        temp *= params.alpha;
    }

    // 最良の多角形を戻す
    *polygon = best_polygon;
}

#[fastout]
fn main() {
    input! {
        n: usize,
        sxy: [(usize, usize); n],
        ixy: [(usize, usize); n],
    }

    // 多角形の初期化（全体を覆う矩形）
    let initial_vertices = vec![
        Point { x: 0, y: 0 },
        Point { x: 100_000, y: 0 },
        Point { x: 100_000, y: 100_000 },
        Point { x: 0, y: 100_000 },
    ];
    let mut polygon = Polygon::new(initial_vertices);

    // スコアマップの作成
    let score_map = ScoreMap::new(sxy, ixy);

    // 焼きなまし法のパラメータ設定
    let annealing_params = AnnealingParams::new(
        10000.0, // 初期温度
        1.0,     // 終了温度
        0.995,   // 冷却率
        10_000,  // 反復回数
    );

    // 焼きなまし法の実行
    simulated_annealing(&mut polygon, &score_map, annealing_params);

    // 制約の確認と強制
    polygon.enforce_constraints();

    // 結果の出力
    println!("{}", polygon.vertices.len());
    for vertex in &polygon.vertices {
        println!("{} {}", vertex.x, vertex.y);
    }
}
