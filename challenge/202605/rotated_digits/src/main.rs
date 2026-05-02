fn rotated_digits(n: i32) -> i32 {
    let mut ret = 0;
    let rotated_values = [0, 1, 5, -1, -1, 2, 9, -1, 8, 6];
    for i in 1..=n {
        let mut num = i as usize;
        let mut v = 0;
        let mut base = 1;
        let mut ok = true;
        while num > 0 {
            let m = rotated_values[num % 10];
            if m < 0 {
                ok = false;
                break;
            }

            v += base * m;
            num /= 10;
            base *= 10;
        }

        if ok && v != i {
            ret += 1;
        }
    }

    ret
}

fn main() {
    let ret = rotated_digits(10);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(rotated_digits(10), 4);
    assert_eq!(rotated_digits(1), 0);
    assert_eq!(rotated_digits(2), 1);
}
