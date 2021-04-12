fn array_sign(nums: Vec<i32>) -> i32 {
    let mut ret = 1;
    for &num in &nums {
        ret = match num {
            n if n > 0 => ret,
            n if n < 0 => -ret,
            _ => return 0,
        };
    }

    ret
}

fn main() {
    let v = vec![-1, -2, -3, -4, 3, 2, 1];
    let ret = array_sign(v);
    println!("ret={}", ret);
}

#[test]
fn test_array_sign() {
    assert_eq!(array_sign(vec![-1, -2, -3, -4, 3, 2, 1]), 1);
    assert_eq!(array_sign(vec![1, 5, 0, 2, -3]), 0);
    assert_eq!(array_sign(vec![-1, 1, -1, 1, -1]), -1);
}
