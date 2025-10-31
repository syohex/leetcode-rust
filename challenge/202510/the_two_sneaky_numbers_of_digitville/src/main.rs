fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    nums.sort_unstable();

    let mut ret = vec![0; 2];
    let mut diff = 0;
    for (i, n) in nums.into_iter().enumerate() {
        if n as usize + diff != i {
            ret[diff] = n;
            diff += 1;
            if diff == 2 {
                break;
            }
        }
    }
    ret
}

fn main() {
    let nums = vec![0, 3, 2, 1, 3, 2];
    let ret = get_sneaky_numbers(nums);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let nums = vec![2, 0, 2, 1, 0];
        let ret = get_sneaky_numbers(nums);
        assert_eq!(ret, [0, 2]);
    }
    {
        let nums = vec![0, 1, 1, 0];
        let ret = get_sneaky_numbers(nums);
        assert_eq!(ret, [0, 1]);
    }
    {
        let nums = vec![0, 3, 2, 1, 3, 2];
        let ret = get_sneaky_numbers(nums);
        assert_eq!(ret, [2, 3]);
    }
    {
        let nums = vec![7, 1, 5, 4, 3, 4, 6, 0, 9, 5, 8, 2];
        let ret = get_sneaky_numbers(nums);
        assert_eq!(ret, [4, 5]);
    }
}
