use std::collections::{HashMap, HashSet, VecDeque};

fn parse_input(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut parts = input.trim().split("\n\n");
    
    // Parse rules
    let rules: Vec<(u32, u32)> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut nums = line.split('|')
                .map(|n| n.parse::<u32>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .collect();
    
    // Parse updates
    let updates: Vec<Vec<u32>> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect()
        })
        .collect();
    
    (rules, updates)
}

fn build_graph(rules: &[(u32, u32)], pages: &[u32]) -> HashMap<u32, Vec<u32>> {
    let page_set: HashSet<_> = pages.iter().cloned().collect();
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
    
    // Only include rules relevant to the current update
    for &(from, to) in rules {
        if page_set.contains(&from) && page_set.contains(&to) {
            graph.entry(from)
                .or_default()
                .push(to);
            // Ensure all nodes are in the graph even if they have no outgoing edges
            graph.entry(to).or_default();
        }
    }
    
    graph
}

fn calculate_in_degrees(graph: &HashMap<u32, Vec<u32>>, pages: &[u32]) -> HashMap<u32, u32> {
    let mut in_degrees: HashMap<u32, u32> = pages.iter().map(|&p| (p, 0)).collect();
    
    for (_node, edges) in graph {
        for &dest in edges {
            *in_degrees.entry(dest).or_default() += 1;
        }
    }
    
    in_degrees
}

fn is_valid_order(rules: &[(u32, u32)], update: &[u32]) -> bool {
    let graph = build_graph(rules, update);
    let mut in_degrees = calculate_in_degrees(&graph, update);
    let mut result = Vec::new();
    let mut queue: VecDeque<_> = in_degrees
        .iter()
        .filter(|(&node, &deg)| deg == 0)
        .map(|(&node, _)| node)
        .collect();
    
    while let Some(node) = queue.pop_front() {
        result.push(node);
        
        if let Some(edges) = graph.get(&node) {
            for &dest in edges {
                let deg = in_degrees.get_mut(&dest).unwrap();
                *deg -= 1;
                if *deg == 0 {
                    queue.push_back(dest);
                }
            }
        }
    }
    
    // Check if we got all nodes and they match the original order
    result.len() == update.len() && 
        update.iter().enumerate().all(|(i, &page)| {
            let pos_in_result = result.iter().position(|&x| x == page).unwrap();
            pos_in_result <= i
        })
}

fn get_correct_order(rules: &[(u32, u32)], update: &[u32]) -> Vec<u32> {
    let graph = build_graph(rules, update);
    let mut in_degrees = calculate_in_degrees(&graph, update);
    let mut result = Vec::new();
    let mut queue: VecDeque<_> = in_degrees
        .iter()
        .filter(|(&node, &deg)| deg == 0)
        .map(|(&node, _)| node)
        .collect();
    
    // Sort queue to ensure deterministic ordering when multiple nodes have 0 in-degree
    let mut queue_vec: Vec<_> = queue.drain(..).collect();
    queue_vec.sort_unstable_by(|a, b| b.cmp(a));  // Sort in reverse order
    queue = queue_vec.into_iter().collect();

    while let Some(node) = queue.pop_front() {
        result.push(node);
        
        if let Some(edges) = graph.get(&node) {
            for &dest in edges {
                let deg = in_degrees.get_mut(&dest).unwrap();
                *deg -= 1;
                if *deg == 0 {
                    queue.push_back(dest);
                }
            }
            // Sort queue again to maintain deterministic ordering
            let mut queue_vec: Vec<_> = queue.drain(..).collect();
            queue_vec.sort_unstable_by(|a, b| b.cmp(a));
            queue = queue_vec.into_iter().collect();
        }
    }
    
    result
}

fn get_middle_number(update: &[u32]) -> u32 {
    update[update.len() / 2]
}
#[test]
fn main() {
    let input = include_str!("d5.in");
    let (rules, updates) = parse_input(input);
    
    let mut sum = 0;
    for update in &updates {
        if !is_valid_order(&rules, update) {
            let correct_order = get_correct_order(&rules, update);
            sum += get_middle_number(&correct_order);
        }
    }
    
    println!("Sum of middle numbers from corrected invalid updates: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_example_part2() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        
        let (rules, updates) = parse_input(input);
        let mut sum = 0;
        for update in &updates {
            if !is_valid_order(&rules, update) {
                let correct_order = get_correct_order(&rules, update);
                sum += get_middle_number(&correct_order);
            }
        }
        assert_eq!(sum, 123);
    }
}