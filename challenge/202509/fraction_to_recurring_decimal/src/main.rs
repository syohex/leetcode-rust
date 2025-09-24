fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
    use std::collections::HashMap;

    if numerator == 0 {
        return "0".to_string();
    }

    let mut ret = String::new();
    if (numerator < 0 || denominator < 0) && !(numerator < 0 && denominator < 0) {
        ret.push('-');
    }

    let a = (numerator as i64).abs();
    let b = (denominator as i64).abs();

    ret.push_str(&(a / b).to_string());
    let mut rem = a % b;
    if rem == 0 {
        return ret;
    }

    ret.push('.');

    let mut h = HashMap::new();
    while rem != 0 {
        if let Some(pos) = h.get(&rem) {
            ret.insert(*pos, '(');
            ret.push(')');
            return ret;
        }

        h.insert(rem, ret.len());
        rem *= 10;
        ret.push_str(&(rem / b).to_string());
        rem %= b;
    }

    ret
}

fn main() {
    let ret = fraction_to_decimal(1, 3);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(fraction_to_decimal(1, 2), "0.5");
    assert_eq!(fraction_to_decimal(2, 1), "2");
    assert_eq!(fraction_to_decimal(4, 333), "0.(012)");
}
