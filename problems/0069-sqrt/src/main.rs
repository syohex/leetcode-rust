fn my_sqrt(x: i32) -> i32 {
    let x = x as i64;
    let mut left = 0i64;
    let mut right = x;
    let mut ret = x;

    while left <= right {
        let mid = left + (right - left) / 2;
        let square = mid * mid;
        if square <= x {
            ret = mid;
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    ret as i32
}

fn main() {
    let ret = my_sqrt(4);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(my_sqrt(2147395599), 46339);
    assert_eq!(my_sqrt(4), 2);
    assert_eq!(my_sqrt(8), 2);
}
