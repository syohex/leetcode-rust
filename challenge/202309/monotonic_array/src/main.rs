fn is_monotonic(nums: Vec<i32>) -> bool {
    fn cmp<T: Eq + Ord + Copy>(a: &[T]) -> i32 {
        if a[0] == a[1] {
            0
        } else if a[0] > a[1] {
            1
        } else {
            -1
        }
    }

    if nums.len() == 1 {
        true
    } else {
        let cmps: Vec<i32> = nums.windows(2).map(cmp).collect();
        cmps.iter().all(|&a| a >= 0) || cmps.iter().all(|&a| a <= 0)
    }
}

fn main() {
    let nums = vec![1, 2, 2, 3];
    let ret = is_monotonic(nums);
    println!("ret={ret}");
}

#[test]
fn test_is_monotonic() {
    assert!(is_monotonic(vec![1, 2, 2, 3]));
    assert!(is_monotonic(vec![6, 5, 4, 4]));
    assert!(is_monotonic(vec![1, 1, 1, 1, 1]));
    assert!(is_monotonic(vec![1]));
    assert!(!is_monotonic(vec![1, 3, 2]));
    assert!(is_monotonic(vec![1, 1, 0]));
}
