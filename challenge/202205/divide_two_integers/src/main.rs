fn divide(dividend: i32, divisor: i32) -> i32 {
    if divisor == 1 {
        return dividend;
    }
    if divisor == -1 {
        if dividend == std::i32::MIN {
            return std::i32::MAX;
        }

        return -dividend;
    }

    let dividend = dividend as i64;
    let divisor = divisor as i64;

    let (mut a, b, negative) = if dividend < 0 && divisor < 0 {
        (-dividend, -divisor, false)
    } else if dividend < 0 {
        (-dividend, divisor, true)
    } else if divisor < 0 {
        (dividend, -divisor, true)
    } else {
        (dividend, divisor, false)
    };

    let mut ret = 0;
    while a >= b {
        a -= b;
        ret += 1;
    }

    if negative {
        -ret
    } else {
        ret
    }
}

fn main() {
    let dividend = 10;
    let divisor = 3;
    let ret = divide(dividend, divisor);
    println!("ret={ret}");
}

#[test]
fn test_divide() {
    let table = [
        (10, 3, 3),
        (7, -3, -2),
        (10, 1, 10),
        (10, -1, -10),
        (-10, -1, 10),
        (std::i32::MIN, -1, std::i32::MAX),
        (std::i32::MIN, 1, std::i32::MIN),
        (std::i32::MIN, 2, -1073741824),
    ];
    for (a, b, expected) in table {
        assert_eq!(divide(a, b), expected);
    }
}
