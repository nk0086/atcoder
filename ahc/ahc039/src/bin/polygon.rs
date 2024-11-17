use rand::thread_rng;
use rand::Rng;

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
        let idx = rng.gen_range(0..self.edges.len());
        Some(idx)
    }

    // 辺の変更（後述の機能で詳細化）
    fn modify_edge(&mut self, edge_idx: usize, new_start: Point, new_end: Point) {
        if edge_idx >= self.edges.len() {
            return;
        }
        self.edges[edge_idx].start = new_start;
        self.edges[edge_idx].end = new_end;
        // 頂点も更新
        self.vertices[edge_idx] = new_start.clone();
        self.vertices[(edge_idx + 1) % self.vertices.len()] = new_end.clone();
    }

    // 垂線を降ろし交点を求める
    fn drop_perpendicular(&self, edge: &Edge, point: &Point) -> Option<Point> {
        // 現在のロジックでは軸に平行な辺のみを想定
        // x軸または y軸に平行な場合、垂線は簡単に計算可能
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
        let perp_point1 = self.drop_perpendicular(edge, &new_point1).unwrap();
        let perp_point2 = self.drop_perpendicular(edge, &new_point2).unwrap();

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
        let new_x1 = rng.gen_range(0, 100_000);
        let new_y1 = rng.gen_range(0, 100_000);
        let new_x2 = rng.gen_range(0, 100_000);
        let new_y2 = rng.gen_range(0, 100_000);
        let new_point1 = Point {
            x: new_x1,
            y: new_y1,
        };
        let new_point2 = Point {
            x: new_x2,
            y: new_y2,
        };

        // 辺の変更を試みる
        self.change_edge(edge_idx, new_point1, new_point2);
        true
    }
}
