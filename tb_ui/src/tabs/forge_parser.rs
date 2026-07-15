use crate::state::Pill;
use tb_core::ast::Expr;

pub fn compile_pills(pills: &[Pill]) -> Result<Expr, String> {
    if pills.is_empty() {
        return Err("Expression is empty.".to_string());
    }

    let mut pos = 0;
    let expr = parse_expression(pills, &mut pos)?;

    if pos < pills.len() {
        return Err(format!("Unexpected trailing tokens starting at index {}: {:?}", pos, pills[pos]));
    }

    Ok(expr)
}

fn parse_expression(pills: &[Pill], pos: &mut usize) -> Result<Expr, String> {
    let mut left = parse_term(pills, pos)?;

    while *pos < pills.len() {
        match &pills[*pos] {
            Pill::Operator(op) if op == "+" || op == "-" => {
                let operator = op.clone();
                *pos += 1;
                let right = parse_term(pills, pos)?;
                if operator == "+" {
                    left = Expr::Add { lhs: Box::new(left), rhs: Box::new(right) };
                } else {
                    left = Expr::Sub { lhs: Box::new(left), rhs: Box::new(right) };
                }
            }
            _ => break,
        }
    }

    Ok(left)
}

fn parse_term(pills: &[Pill], pos: &mut usize) -> Result<Expr, String> {
    let mut left = parse_factor(pills, pos)?;

    while *pos < pills.len() {
        match &pills[*pos] {
            Pill::Operator(op) if op == "*" || op == "/" => {
                let operator = op.clone();
                *pos += 1;
                let right = parse_factor(pills, pos)?;
                if operator == "*" {
                    left = Expr::Mul { lhs: Box::new(left), rhs: Box::new(right) };
                } else {
                    left = Expr::Div { lhs: Box::new(left), rhs: Box::new(right) };
                }
            }
            _ => break,
        }
    }

    Ok(left)
}

fn parse_factor(pills: &[Pill], pos: &mut usize) -> Result<Expr, String> {
    if *pos >= pills.len() {
        return Err("Unexpected end of expression.".to_string());
    }

    let pill = &pills[*pos];
    *pos += 1;

    match pill {
        Pill::Constant(val) => Ok(Expr::Constant { value: *val }),
        Pill::Source(name) => {
            match name.as_str() {
                "Close" => Ok(Expr::Close),
                "Open" => Ok(Expr::Open),
                "High" => Ok(Expr::High),
                "Low" => Ok(Expr::Low),
                "Volume" => Ok(Expr::Volume),
                _ => Ok(Expr::Placeholder),
            }
        }
        Pill::Indicator(name, var_name) => {
            // Function call, expect optional brackets or just next factor
            // For simplicity, if we see '(', we parse an expression. Otherwise we just parse a factor.
            let inner_expr = if *pos < pills.len() && pills[*pos] == Pill::OpenBracket {
                *pos += 1; // consume '('
                let inner = parse_expression(pills, pos)?;
                if *pos >= pills.len() || pills[*pos] != Pill::CloseBracket {
                    return Err(format!("Expected ')' after argument for {}", name));
                }
                *pos += 1; // consume ')'
                inner
            } else {
                parse_factor(pills, pos)?
            };
            
            // Note: periods are hardcoded for now until we integrate fully with ParamPlaceholder.
            // But we can bake a default integer since the GA will mutate the parameter dynamically later.
            let default_period = 14;

            match name.as_str() {
                "SMA" => Ok(Expr::Sma { source: Box::new(inner_expr), period: default_period }),
                "EMA" => Ok(Expr::Ema { source: Box::new(inner_expr), period: default_period }),
                "MACD" => Ok(tb_indicators::templates::macd_line(inner_expr, 12, 26)),
                _ => Err(format!("Unknown indicator function: {}", name)),
            }
        }
        Pill::OpenBracket => {
            let inner = parse_expression(pills, pos)?;
            if *pos >= pills.len() || pills[*pos] != Pill::CloseBracket {
                return Err("Mismatched parenthesis: expected ')'".to_string());
            }
            *pos += 1; // consume ')'
            Ok(inner)
        }
        _ => Err(format!("Unexpected token: {:?}", pill)),
    }
}
