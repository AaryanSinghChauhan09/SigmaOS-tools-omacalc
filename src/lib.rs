#![forbid(unsafe_code)]
//! omacalc - Desktop calculator engine for SigmaOS

pub struct Calculator {
    history: Vec<(String, f64)>,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }

    pub fn evaluate(&mut self, expr: &str) -> Result<f64, String> {
        let result = Self::eval_expr(expr.trim())?;
        self.history.push((expr.to_string(), result));
        Ok(result)
    }

    fn eval_expr(expr: &str) -> Result<f64, String> {
        let expr = expr.trim();
        let mut depth = 0i32;
        let mut last_add_sub = None;
        for (i, c) in expr.char_indices() {
            match c {
                '(' => depth += 1,
                ')' => depth -= 1,
                '+' | '-' if depth == 0 && i > 0 => {
                    last_add_sub = Some((i, c));
                }
                _ => {}
            }
        }
        if let Some((i, op)) = last_add_sub {
            let left = Self::eval_expr(&expr[..i])?;
            let right = Self::eval_expr(&expr[i + 1..])?;
            return Ok(if op == '+' {
                left + right
            } else {
                left - right
            });
        }

        depth = 0;
        let mut last_mul_div = None;
        for (i, c) in expr.char_indices() {
            match c {
                '(' => depth += 1,
                ')' => depth -= 1,
                '*' | '/' if depth == 0 => {
                    last_mul_div = Some((i, c));
                }
                _ => {}
            }
        }
        if let Some((i, op)) = last_mul_div {
            let left = Self::eval_expr(&expr[..i])?;
            let right = Self::eval_expr(&expr[i + 1..])?;
            if op == '/' && right == 0.0 {
                return Err("Division by zero".to_string());
            }
            return Ok(if op == '*' {
                left * right
            } else {
                left / right
            });
        }

        if expr.starts_with('(') && expr.ends_with(')') {
            return Self::eval_expr(&expr[1..expr.len() - 1]);
        }

        expr.parse::<f64>()
            .map_err(|_| format!("Cannot parse expression: {}", expr))
    }

    pub fn history_len(&self) -> usize {
        self.history.len()
    }

    pub fn last_result(&self) -> Option<f64> {
        self.history.last().map(|(_, r)| *r)
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}


#[cfg(test)]
mod oma_calc_tests {
    use super::*;

    #[test]
    fn test_order_of_operations_aaa() {
        // Arrange
        let mut calc = Calculator::new();
        // Act
        let res = calc.evaluate("2 + 3 * 4").unwrap();
        // Assert
        assert_eq!(res, 14.0);
    }

    #[test]
    fn test_parentheses_precedence_aaa() {
        // Arrange
        let mut calc = Calculator::new();
        // Act
        let res = calc.evaluate("(2 + 3) * 4").unwrap();
        // Assert
        assert_eq!(res, 20.0);
    }

    #[test]
    fn test_floating_point_division_aaa() {
        // Arrange
        let mut calc = Calculator::new();
        // Act
        let res = calc.evaluate("7 / 2").unwrap();
        // Assert
        assert_eq!(res, 3.5);
    }

    #[test]
    fn test_invalid_syntax_error_aaa() {
        // Arrange
        let mut calc = Calculator::new();
        // Act
        let res = calc.evaluate("2 + abc");
        // Assert
        assert!(res.is_err());
    }
}
