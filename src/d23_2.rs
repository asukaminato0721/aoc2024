use std::collections::{HashMap, HashSet};
#[test]
fn main() {
    let input = include_str!("d23.in");

    let password = find_lan_party_password(input);
    println!("LAN party password: {}", password);
}

fn find_lan_party_password(input: &str) -> String {
    // Create adjacency list representation
    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();

    // Parse input and build connections
    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        connections
            .entry(a.to_string())
            .or_default()
            .insert(b.to_string());
        connections
            .entry(b.to_string())
            .or_default()
            .insert(a.to_string());
    }

    // Get all computers
    let computers: Vec<String> = connections.keys().cloned().collect();

    // Find the maximum clique
    let max_clique = find_maximum_clique(&computers, &connections);

    // Sort computers alphabetically and join with commas
    let mut password: Vec<String> = max_clique.into_iter().collect();
    password.sort();
    password.join(",")
}

fn find_maximum_clique(
    computers: &[String],
    connections: &HashMap<String, HashSet<String>>,
) -> HashSet<String> {
    let mut max_clique = HashSet::new();

    // Try every possible starting computer
    for start in computers {
        let mut current_clique = HashSet::new();
        current_clique.insert(start.clone());

        // Get all potential candidates (computers connected to start)
        let mut candidates: Vec<String> = connections[start].iter().cloned().collect();

        expand_clique(
            &mut current_clique,
            &mut candidates,
            connections,
            &mut max_clique,
        );
    }

    max_clique
}

fn expand_clique(
    current: &mut HashSet<String>,
    candidates: &mut Vec<String>,
    connections: &HashMap<String, HashSet<String>>,
    max_clique: &mut HashSet<String>,
) {
    // Update max_clique if current is larger
    if current.len() > max_clique.len() {
        max_clique.clear();
        max_clique.extend(current.iter().cloned());
    }

    // Try each candidate
    while let Some(candidate) = candidates.pop() {
        // Check if candidate is connected to all computers in current clique
        if current.iter().all(|c| connections[c].contains(&candidate)) {
            // Add candidate to current clique
            current.insert(candidate.clone());

            // Create new candidates list with only computers connected to everything
            let mut new_candidates: Vec<String> = candidates
                .iter()
                .cloned()
                .filter(|c| current.iter().all(|curr| connections[curr].contains(c)))
                .collect();

            // Recursively expand clique
            expand_clique(current, &mut new_candidates, connections, max_clique);

            // Remove candidate for backtracking
            current.remove(&candidate);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_lan_party_password() {
        let input = "ka-co\nta-co\nde-co\nta-ka\nde-ta\nka-de";
        assert_eq!(find_lan_party_password(input), "co,de,ka,ta");
    }
}
