fn parse_bool_expr(expression: String) -> bool {
    fn f(pos: usize, cs: &Vec<char>) -> (bool, usize) {
        match cs[pos] {
            't' => (true, pos + 1),
            'f' => (false, pos + 1),
            '!' => {
                let (expr, pos) = f(pos + 2, cs);
                (!expr, pos + 1)
            }
            '&' => {
                let (expr, pos) = f(pos + 2, cs);
                let mut ret = expr;
                let mut pos = pos;
                loop {
                    match cs[pos] {
                        ',' => {
                            let (v, p) = f(pos + 1, cs);
                            ret = ret & v;
                            pos = p;
                        }
                        ')' => {
                            return (ret, pos + 1);
                        }
                        _ => unreachable!("never reach here"),
                    }
                }
            }
            '|' => {
                let (expr, pos) = f(pos + 2, cs);
                let mut ret = expr;
                let mut pos = pos;
                loop {
                    match cs[pos] {
                        ',' => {
                            let (v, p) = f(pos + 1, cs);
                            ret = ret | v;
                            pos = p;
                        }
                        ')' => {
                            return (ret, pos + 1);
                        }
                        _ => unreachable!("never reach here"),
                    }
                }
            }
            _ => unreachable!("never reach here"),
        }
    }

    let cs = expression.chars().collect();
    f(0, &cs).0
}

fn main() {
    let ret = parse_bool_expr("|(f,f,f,t)".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(parse_bool_expr("&(|(f))".to_string()), false);
    assert_eq!(parse_bool_expr("|(f,f,f,t)".to_string()), true);
    assert_eq!(parse_bool_expr("&(t,t,f)".to_string()), false);
    assert_eq!(parse_bool_expr("&(t)".to_string()), true);
    assert_eq!(parse_bool_expr("!(&(&(!(&(f)),&(t),|(f,f,t)),&(t),&(t,t,f)))".to_string()), true);
    assert_eq!(parse_bool_expr("!(&(&(!(&(f)),&(t),|(f,f,t)),&(t),&(t,t,f)))".to_string()), true);
}
