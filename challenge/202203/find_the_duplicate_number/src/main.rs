fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut v = vec![false;nums.len() + 1];
    for n in nums {
        if v[n as usize] {
            return n;
        }

        v[n as usize] = true;
    }

    panic!("never reach here");
}

fn main() {
    let nums = vec![1, 3, 4, 2, 2];
    let ret = find_duplicate(nums);
    println!("ret={ret}");
}

#[test]
fn test_find_duplicate() {
    {
        let nums = vec![1, 3, 4, 2, 2];
        assert_eq!(find_duplicate(nums), 2);
    }
    {
        let nums = vec![3, 1, 3, 4, 2];
        assert_eq!(find_duplicate(nums), 3);
    }
}
