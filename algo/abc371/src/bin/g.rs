use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        n: usize,
        p: [usize; n], // permutation P (1-based indexing)
        a: [usize; n], // array A
    }

    let mut p_zero_based: Vec<usize> = p.iter().map(|&x| x - 1).collect();
    let mut visited = vec![false; n];
    let mut cycles = Vec::new();
    for i in 0..n {
        if !visited[i] {
            let mut positions = Vec::new();
            let mut j = i;
            while !visited[j] {
                visited[j] = true;
                positions.push(j);
                j = p_zero_based[j];
            }
            cycles.push(positions);
        }
    }

    // Map to store the final values at each position
    let mut result = vec![0usize; n];

    // Process cycles in order of their minimum position
    cycles.sort_by_key(|cycle| *cycle.iter().min().unwrap());

    // Positions that have been assigned so far
    let mut assigned_positions = vec![];

    for cycle in cycles {
        let mut elements = cycle.iter().map(|&idx| a[idx]).collect::<Vec<_>>();
        let len = elements.len();

        // Generate all rotations
        let mut rotations = Vec::new();
        for shift in 0..len {
            let rotated = elements
                .iter()
                .cycle()
                .skip(shift)
                .take(len)
                .cloned()
                .collect::<Vec<_>>();
            rotations.push(rotated);
        }

        // Sort rotations based on their impact on the overall sequence
        rotations.sort_by(|r1, r2| {
            for (&pos, (&e1, &e2)) in cycle.iter().zip(r1.iter().zip(r2.iter())) {
                if assigned_positions.contains(&pos) {
                    continue;
                }
                if e1 != e2 {
                    return e1.cmp(&e2);
                }
            }
            Ordering::Equal
        });

        // Choose the rotation that minimizes the overall sequence
        let chosen_rotation = rotations[0].clone();

        // Assign the chosen rotation to the result
        for (&pos, &val) in cycle.iter().zip(chosen_rotation.iter()) {
            result[pos] = val;
            assigned_positions.push(pos);
        }
    }

    // Output the result
    for (i, val) in result.iter().enumerate() {
        print!("{}", val);
        if i + 1 < n {
            print!(" ");
        } else {
            println!();
        }
    }
}
