fn is_good(nums: Vec<i32>) -> bool {
    let len = nums.len();
    let mut v = vec![0; len + 1];
    let limit = len as i32 - 1;

    for n in nums {
        if n > limit {
            return false;
        }

        if n != limit && v[n as usize] != 0 {
            return false;
        }

        v[n as usize] += 1;
    }

    v[limit as usize] == 2
}

fn main() {
    let ret = is_good(vec![2, 1, 3]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(!is_good(vec![2, 1, 3]));
    assert!(is_good(vec![1, 3, 3, 2]));
    assert!(is_good(vec![1, 1]));
}
