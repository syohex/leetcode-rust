fn longest_balanced(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    let mut ret = 0;
    let len = nums.len();
    for i in 0..len {
        let mut evens = HashSet::new();
        let mut odds = HashSet::new();

        for j in i..len {
            if nums[j] % 2 == 0 {
                evens.insert(nums[j]);
            } else {
                odds.insert(nums[j]);
            }

            if evens.len() == odds.len() {
                ret = std::cmp::max(ret, j - i + 1);
            }
        }
    }

    ret as i32
}

fn main() {
    let nums = vec![2, 5, 4, 3];
    let ret = longest_balanced(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![2, 5, 4, 3];
        let ret = longest_balanced(nums);
        assert_eq!(ret, 4);
    }
    {
        let nums = vec![3, 2, 2, 5, 4];
        let ret = longest_balanced(nums);
        assert_eq!(ret, 5);
    }
    {
        let nums = vec![1, 2, 3, 2];
        let ret = longest_balanced(nums);
        assert_eq!(ret, 3);
    }
}
