#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars};
use std::collections::BTreeMap;

fn main() {
    input! {
        q: usize,
        queries: [Chars; q],
    };

    let mut s: i64 = 0; // Cumulative sum of days waited
    let mut map = BTreeMap::new(); // base_height -> count

    for query in queries {
        // Convert Vec<char> to String
        let query_str: String = query.into_iter().collect();
        // Split the string into parts by whitespace
        let parts: Vec<&str> = query_str.split_whitespace().collect();
        println!("{:?}", parts);
        match parts[0] {
            "1" => {
                // Type 1: Plant a new plant with base height = -S
                *map.entry(-s).or_insert(0) += 1;
            }
            "2" => {
                // Type 2: Wait T days
                if parts.len() < 2 {
                    eprintln!("Invalid query for type 2: {:?}", parts);
                    continue;
                }
                let t: i64 = parts[1].parse().unwrap_or_else(|_| {
                    eprintln!("Failed to parse T in query: {:?}", parts);
                    0
                });
                s += t;
            }
            "3" => {
                // Type 3: Harvest plants with height >= H
                if parts.len() < 2 {
                    eprintln!("Invalid query for type 3: {:?}", parts);
                    continue;
                }
                let h: i64 = parts[1].parse().unwrap_or_else(|_| {
                    eprintln!("Failed to parse H in query: {:?}", parts);
                    0
                });
                let threshold = h - s;
                // Find all keys >= threshold
                let keys_to_remove: Vec<i64> = map.range(threshold..).map(|(&k, _)| k).collect();
                let mut count = 0;
                for key in keys_to_remove {
                    if let Some(cnt) = map.remove(&key) {
                        count += cnt;
                    }
                }
                println!("{}", count);
            }
            _ => {
                eprintln!("Unknown query type: {:?}", parts);
            }
        }
    }
}
