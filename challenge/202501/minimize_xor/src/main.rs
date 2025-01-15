fn minimize_xor(num1: i32, num2: i32) -> i32 {
    let mut ones = num2.count_ones() as i32;
    let mut ret = 0;

    let num1 = num1 as u32;
    let mut mask = 0x80000000;
    for _ in 0..32 {
        if num1 & mask != 0 {
            ret |= mask;
            ones -= 1;
            if ones == 0 {
                break;
            }
        }

        mask >>= 1;
    }

    mask = 1;
    while ones > 0 {
        if num1 & mask == 0 {
            ret |= mask;
            ones -= 1;
        }

        mask <<= 1;
    }

    ret as i32
}

fn main() {
    let ret = minimize_xor(25, 72);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(minimize_xor(3, 5), 3);
    assert_eq!(minimize_xor(1, 12), 3);
    assert_eq!(minimize_xor(25, 72), 24);
}
