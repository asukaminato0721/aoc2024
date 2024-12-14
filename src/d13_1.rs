use std::error::Error;
use std::str::FromStr;

#[derive(Debug)]
struct ClawMachine {
    button_a: (i64, i64),  // (X, Y) movement for button A
    button_b: (i64, i64),  // (X, Y) movement for button B
    prize: (i64, i64),     // (X, Y) coordinates of prize
}

impl FromStr for ClawMachine {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        if lines.len() != 3 {
            return Err("Invalid input format".into());
        }

        // Parse Button A
        let a_parts: Vec<&str> = lines[0].split(", ").collect();
        let ax = a_parts[0].split("+").nth(1).ok_or("Invalid X format")?.parse::<i64>()?;
        let ay = a_parts[1].split("+").nth(1).ok_or("Invalid Y format")?.parse::<i64>()?;

        // Parse Button B
        let b_parts: Vec<&str> = lines[1].split(", ").collect();
        let bx = b_parts[0].split("+").nth(1).ok_or("Invalid X format")?.parse::<i64>()?;
        let by = b_parts[1].split("+").nth(1).ok_or("Invalid Y format")?.parse::<i64>()?;

        // Parse Prize coordinates
        let p_parts: Vec<&str> = lines[2].split(", ").collect();
        let px = p_parts[0].split("=").nth(1).ok_or("Invalid X format")?.parse::<i64>()?;
        let py = p_parts[1].split("=").nth(1).ok_or("Invalid Y format")?.parse::<i64>()?;

        Ok(ClawMachine {
            button_a: (ax, ay),
            button_b: (bx, by),
            prize: (px, py),
        })
    }
}

fn solve_machine(machine: &ClawMachine) -> Option<i64> {
    // Try all combinations of button presses up to 100 each
    for a in 0..=100 {
        for b in 0..=100 {
            let x = a * machine.button_a.0 + b * machine.button_b.0;
            let y = a * machine.button_a.1 + b * machine.button_b.1;
            
            if x == machine.prize.0 && y == machine.prize.1 {
                // Calculate token cost: 3 tokens for A, 1 token for B
                return Some(3 * a + b);
            }
        }
    }
    None
}

fn solve_puzzle(input: &str) -> i64 {
    let machines: Vec<ClawMachine> = input
        .split("\n\n")
        .filter(|s| !s.trim().is_empty())
        .filter_map(|s| s.parse().ok())
        .collect();

    machines
        .iter()
        .filter_map(solve_machine)
        .sum()
}
#[test]
fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("d13.in");

    let result = solve_puzzle(input);
    println!("Minimum tokens needed: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

        assert_eq!(solve_puzzle(input), 480);
    }

    #[test]
    fn test_single_machine() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400";

        let machine: ClawMachine = input.parse().unwrap();
        assert_eq!(solve_machine(&machine), Some(280));
    }
}