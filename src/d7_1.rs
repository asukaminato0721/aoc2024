use std::fs::read_to_string;

#[derive(Debug)]
struct Equation {
    test_value: i64,
    numbers: Vec<i64>,
}

impl Equation {
    fn parse(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            return None;
        }

        let test_value = parts[0].trim().parse().ok()?;
        let numbers = parts[1]
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();

        Some(Equation {
            test_value,
            numbers,
        })
    }

    fn evaluate(&self, operators: &[char]) -> i64 {
        let mut result = self.numbers[0];
        
        for (i, &op) in operators.iter().enumerate() {
            match op {
                '+' => result += self.numbers[i + 1],
                '*' => result *= self.numbers[i + 1],
                _ => unreachable!(),
            }
        }
        
        result
    }

    fn is_valid(&self) -> bool {
        let num_operators = self.numbers.len() - 1;
        let operators = ['+', '*'];
        
        // Generate all possible operator combinations
        let mut valid = false;
        for i in 0..(1 << num_operators) {
            let mut current_ops = Vec::with_capacity(num_operators);
            
            for j in 0..num_operators {
                current_ops.push(if (i & (1 << j)) == 0 { '+' } else { '*' });
            }
            
            if self.evaluate(&current_ops) == self.test_value {
                valid = true;
                break;
            }
        }
        
        valid
    }
}
#[test]
fn main() {
    let input = read_to_string("src/d7.in").expect("Failed to read input file");
    
    let result: i64 = input
        .lines()
        .filter_map(|line| Equation::parse(line))
        .filter(|eq| eq.is_valid())
        .map(|eq| eq.test_value)
        .sum();
        
    println!("Total calibration result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        let result: i64 = input
            .lines()
            .filter_map(|line| Equation::parse(line))
            .filter(|eq| eq.is_valid())
            .map(|eq| eq.test_value)
            .sum();

        assert_eq!(result, 3749);
    }

    #[test]
    fn test_evaluation() {
        let eq = Equation {
            test_value: 190,
            numbers: vec![10, 19],
        };
        assert_eq!(eq.evaluate(&['*']), 190);
        assert_eq!(eq.evaluate(&['+']), 29);
    }
}