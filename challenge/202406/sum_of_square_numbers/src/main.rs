fn judge_square_sum(c: i32) -> bool {
    fn search(left: i64, right: i64, target: i64) -> bool {
        let mut left = left;
        let mut right = right;

        while left <= right {
            let mid = left + (right - left) / 2;
            let p = mid * mid;
            if p == target {
                return true;
            }

            if p < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        false
    }

    let mut a = 0i64;
    let c = c as i64;
    while a * a <= c {
        let diff = c - (a * a);
        if search(a, diff, diff) {
            return true;
        }

        a += 1;
    }

    false
}

fn main() {
    println!("ret={}", judge_square_sum(10));
}

#[test]
fn test() {
    assert!(judge_square_sum(5));
    assert!(!judge_square_sum(3));
    assert!(judge_square_sum(1000000));
}
