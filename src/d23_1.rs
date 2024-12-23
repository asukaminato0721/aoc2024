use std::collections::{HashMap, HashSet};
#[test]
fn main() {
    let input = include_str!("d23.in");

    let result = find_triplets(input);
    println!("Total triplets with 't' computer: {}", result);
}

fn find_triplets(input: &str) -> usize {
    // Create adjacency list representation
    let mut connections: HashMap<&str, HashSet<&str>> = HashMap::new();

    // Parse input and build connections
    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        connections.entry(a).or_default().insert(b);
        connections.entry(b).or_default().insert(a);
    }

    // Find all triplets
    let mut triplets = HashSet::new();

    for &computer1 in connections.keys() {
        // Get all neighbors of computer1
        if let Some(neighbors1) = connections.get(computer1) {
            // For each neighbor of computer1
            for &computer2 in neighbors1 {
                // Get all neighbors of computer2
                if let Some(neighbors2) = connections.get(computer2) {
                    // For each neighbor of computer2
                    for &computer3 in neighbors2 {
                        // Skip if it's the same as computer1
                        if computer3 == computer1 {
                            continue;
                        }

                        // Check if computer3 is connected to computer1
                        if connections[computer1].contains(computer3) {
                            // Create sorted triplet to avoid duplicates
                            let mut triplet = vec![computer1, computer2, computer3];
                            triplet.sort();

                            triplets.insert((triplet[0], triplet[1], triplet[2]));
                        }
                    }
                }
            }
        }
    }

    // Count triplets containing at least one computer starting with 't'
    triplets
        .iter()
        .filter(|&&(a, b, c)| a.starts_with('t') || b.starts_with('t') || c.starts_with('t'))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_triplets() {
        let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq";
        assert_eq!(find_triplets(input), 0);
    }
}
