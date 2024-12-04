use std::fs::read_to_string;
#[test]
fn main() {
    let input = read_to_string("./src/d3.in").expect("Failed to read input file");
    let result = process_input(&input);
    println!("Sum of all multiplication results: {}", result);
}

fn process_input(input: &str) -> i32 {
    let mut total = 0;
    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();

    while i < chars.len().saturating_sub(6) {  // minimum valid instruction is mul(1,1)
        if check_mul_start(&chars[i..]) {
            if let Some((num1, len1)) = parse_number(&chars[i+4..]) {
                let after_num1 = i + 4 + len1;
                if after_num1 < chars.len() && chars[after_num1] == ',' {
                    if let Some((num2, len2)) = parse_number(&chars[after_num1+1..]) {
                        let after_num2 = after_num1 + 1 + len2;
                        if after_num2 < chars.len() && chars[after_num2] == ')' {
                            total += num1 * num2;
                            i = after_num2;
                        }
                    }
                }
            }
        }
        i += 1;
    }
    total
}

fn check_mul_start(chars: &[char]) -> bool {
    chars.len() >= 4 &&
    chars[0] == 'm' &&
    chars[1] == 'u' &&
    chars[2] == 'l' &&
    chars[3] == '('
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
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(11,8)mul(8,5))";
        assert_eq!(process_input(input), 161);
    }

    #[test]
    fn test_invalid_instructions() {
        let input = "mul(4* mul(6,9! ?(12,34) mul ( 2 , 4 )";
        assert_eq!(process_input(input), 0);
    }

    #[test]
    fn test_multiple_valid_instructions() {
        let input = "mul(44,46)mul(123,4)";
        assert_eq!(process_input(input), 2024 + 492);
    }
}