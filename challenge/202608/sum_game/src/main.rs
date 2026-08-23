fn sum_game(num: String) -> bool {
    let len = num.len();
    let mut q1 = 0;
    let mut sum1 = 0;

    for b in num.bytes().take(len / 2) {
        if b == b'?' {
            q1 += 1;
        } else {
            sum1 += (b - b'0') as i32;
        }
    }

    let mut q2 = 0;
    let mut sum2 = 0;
    for b in num.bytes().skip(len / 2) {
        if b == b'?' {
            q2 += 1;
        } else {
            sum2 += (b - b'0') as i32;
        }
    }

    if (q1 + q2) % 2 == 1 {
        true
    } else {
        let diff = sum1 - sum2;
        diff != (q2 - q1) * 9 / 2
    }
}

fn main() {
    let ret = sum_game("?3295???".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(!sum_game("5023".to_string()));
    assert!(sum_game("25??".to_string()));
    assert!(!sum_game("?3295???".to_string()));
}
