use proconio::input;

fn main() {
    input! {
        n: usize,
        mut grid: [String; n],
    }

    // Convert grid to mutable vectors of chars for easier manipulation
    let mut grid: Vec<Vec<char>> = grid.into_iter().map(|s| s.chars().collect()).collect();

    let layers = n / 2;
    for layer in 0..layers {
        let first = layer;
        let last = n - 1 - layer;
        for i in 0..(last - first) {
            // Positions of the four elements to be rotated
            let top = grid[first][first + i];
            grid[first][first + i] = grid[last - i][first];
            grid[last - i][first] = grid[last][last - i];
            grid[last][last - i] = grid[first + i][last];
            grid[first + i][last] = top;
        }
    }

    // Convert grid back to strings and print
    for row in grid {
        let s: String = row.into_iter().collect();
        println!("{}", s);
    }
}
