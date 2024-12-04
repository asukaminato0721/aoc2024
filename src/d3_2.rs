use std::fs::read_to_string;
#[test]
fn main() {
    let input = read_to_string("./src/d3.in").expect("Failed to read input file");
    let result = process_input(&input);
    println!("Sum of all enabled multiplication results: {}", result);
}

fn process_input(input: &str) -> i32 {
    let mut total = 0;
    let mut i = 0;
    let mut multiply_enabled = true;
    let chars: Vec<char> = input.chars().collect();

    while i < chars.len().saturating_sub(3) {
        // Check for do() instruction
        if check_exact_pattern(&chars[i..], &['d', 'o', '(', ')']) {
            multiply_enabled = true;
            i += 4;
            continue;
        }
        // Check for don't() instruction
        else if check_exact_pattern(&chars[i..], &['d', 'o', 'n', '\'', 't', '(', ')']) {
            multiply_enabled = false;
            i += 7;
            continue;
        }
        // Check for mul instruction
        else if check_mul_start(&chars[i..]) {
            if let Some((num1, len1)) = parse_number(&chars[i+4..]) {
                let after_num1 = i + 4 + len1;
                if after_num1 < chars.len() && chars[after_num1] == ',' {
                    if let Some((num2, len2)) = parse_number(&chars[after_num1+1..]) {
                        let after_num2 = after_num1 + 1 + len2;
                        if after_num2 < chars.len() && chars[after_num2] == ')' {
                            if multiply_enabled {
                                total += num1 * num2;
                            }
                            i = after_num2 + 1;
                            continue;
                        }
                    }
                }
            }
        }
        i += 1;
    }
    total
}

fn check_exact_pattern(chars: &[char], pattern: &[char]) -> bool {
    if chars.len() < pattern.len() {
        return false;
    }
    chars.iter().zip(pattern.iter()).all(|(a, b)| a == b)
}

fn check_mul_start(chars: &[char]) -> bool {
    check_exact_pattern(chars, &['m', 'u', 'l', '('])
}

fn parse_number(chars: &[char]) -> Option<(i32, usize)> {
    let mut num = 0;
    let mut len = 0;
    
    for &c in chars.iter().take(3) {  // max 3 digits
        if c.is_ascii_digit() {
            num = num * 10 + (c as i32 - '0' as i32);
            len += 1;
        } else {
            break;
        }
    }
    
    if len > 0 {
        Some((num, len))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)do()?mul(8,5))";
        assert_eq!(process_input(input), 48);  // 2*4 + 8*5
    }

    #[test]
    fn test_multiple_toggles() {
        let input = "mul(2,3)don't()mul(4,5)do()mul(6,7)";
        assert_eq!(process_input(input), 48);  // 2*3 + 6*7
    }

    #[test]
    fn test_initial_state() {
        let input = "mul(2,2)";
        assert_eq!(process_input(input), 4);  // multiplication starts enabled
    }
}