fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
    use std::cmp::Ordering;

    let mut nums = nums;
    nums.sort_by(|a, b| {
        if a % 2 == 0 {
            Ordering::Less
        } else if b % 2 == 0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
    nums
}

fn main() {
    let nums = vec![3, 1, 2, 4];
    let ret = sort_array_by_parity(nums);
    println!("ret={ret:?}");
}

#[test]
fn test_sort_array_by_parity() {
    {
        let nums = vec![3, 1, 2, 4];
        let ret = sort_array_by_parity(nums);
        assert!(ret[0] % 2 == 0);
        assert!(ret[1] % 2 == 0);
        assert!(ret[2] % 2 != 0);
        assert!(ret[3] % 2 != 0);
    }
    {
        let nums = vec![0];
        let ret = sort_array_by_parity(nums);
        assert!(ret[0] % 2 == 0);
    }
}
