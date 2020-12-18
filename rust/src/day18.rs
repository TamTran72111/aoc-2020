use crate::utilities::read_lines;

#[derive(Default)]
struct Calculator {
    nums: Vec<i64>,
}

impl Calculator {
    fn perform_operation(&mut self, operation: char) {
        if operation == '+' {
            let a = self.nums.pop().unwrap();
            let b = self.nums.pop().unwrap();
            self.nums.push(a + b);
        } else if operation == '*' {
            let a = self.nums.pop().unwrap();
            let b = self.nums.pop().unwrap();
            self.nums.push(a * b);
        }
    }

    fn store(&mut self, num: i64) {
        self.nums.push(num);
    }

    fn get_result(&self) -> i64 {
        self.nums[0]
    }
}

fn calculate_expression(expr: &str, advanced: bool) -> i64 {
    if expr.contains('(') {
        let mut opens = 0;
        let mut start = 0;
        for (i, c) in expr.chars().enumerate() {
            if c == '(' {
                opens += 1;
                if opens == 1 {
                    start = i;
                }
            } else if c == ')' {
                opens -= 1;
                if opens == 0 {
                    let mut new_expr = expr[..start].to_string();
                    let sub_expression_result =
                        calculate_expression(&expr[start + 1..i], advanced).to_string();
                    new_expr.push_str(sub_expression_result.as_str());
                    new_expr.push_str(&expr[i + 1..]);
                    return calculate_expression(new_expr.as_str(), advanced);
                }
            }
        }
        unreachable!("Only reach on invalid expression")
    } else {
        let mut calculator = Calculator::default();
        let mut curr_num = String::new();
        let mut operation = ' ';
        let mut operation_stack = vec![];

        for c in expr.replace(' ', "").chars() {
            if c.is_ascii_digit() {
                curr_num.push(c);
            } else {
                if !curr_num.is_empty() {
                    calculator.store(curr_num.parse().unwrap());
                    curr_num.clear();
                }

                if !advanced || operation == '+' {
                    calculator.perform_operation(operation);
                } else if advanced {
                    operation_stack.push(operation)
                }
                operation = c;
            }
        }

        if !curr_num.is_empty() {
            calculator.store(curr_num.parse().unwrap());
        }

        operation_stack.push(operation);

        while !operation_stack.is_empty() {
            calculator.perform_operation(operation_stack.pop().unwrap());
        }

        calculator.get_result()
    }
}

pub fn main() {
    println!("Day 18");
    let expressions = read_lines("../inputs/day18.txt");

    println!(
        "\tPart 1: {}",
        expressions
            .iter()
            .map(|expr| calculate_expression(expr.as_str(), false))
            .sum::<i64>()
    );
    println!(
        "\tPart 2: {}",
        expressions
            .iter()
            .map(|expr| calculate_expression(expr.as_str(), true))
            .sum::<i64>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_expression() {
        assert_eq!(calculate_expression("1 + 2 * 3 + 4 * 5 + 6", false), 71);
        assert_eq!(
            calculate_expression("1 + (2 * 3) + (4 * (5 + 6))", false),
            51
        );
        assert_eq!(calculate_expression("2 * 3 + (4 * 5)", false), 26);
        assert_eq!(
            calculate_expression("5 + (8 * 3 + 9 + 3 * 4 * 3)", false),
            437
        );
        assert_eq!(
            calculate_expression("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", false),
            12240
        );
        assert_eq!(
            calculate_expression("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", false),
            13632
        );
    }

    #[test]
    fn test_calculate_expression_advanced() {
        assert_eq!(calculate_expression("1 + 2 * 3 + 4 * 5 + 6", true), 231);
        assert_eq!(
            calculate_expression("1 + (2 * 3) + (4 * (5 + 6))", true),
            51
        );
        assert_eq!(calculate_expression("2 * 3 + (4 * 5)", true), 46);
        assert_eq!(
            calculate_expression("5 + (8 * 3 + 9 + 3 * 4 * 3)", true),
            1445
        );
        assert_eq!(
            calculate_expression("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", true),
            669060
        );
        assert_eq!(
            calculate_expression("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", true),
            23340
        );
        assert_eq!(
            calculate_expression(
                "3 + (2 * 2 + (7 * 3) * 2) + 7 + 4 + (2 + 6 * 4 + 9 * 4 * 5)",
                true
            ),
            2186
        );
    }
}
