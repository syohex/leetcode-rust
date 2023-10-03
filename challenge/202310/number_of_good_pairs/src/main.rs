fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let len = nums.len();

    let mut ret = 0;
    for i in 0..(len - 1) {
        for j in (i + 1)..len {
            if nums[i] == nums[j] {
                ret += 1;
            }
        }
    }

    ret
}

fn main() {
    let nums = vec![1, 2, 3, 1, 1, 3];
    let ret = num_identical_pairs(nums);
    println!("ret={ret}");
}

#[test]
fn test_num_identical_pairs() {
    {
        let nums = vec![1, 2, 3, 1, 1, 3];
        let ret = num_identical_pairs(nums);
        assert_eq!(ret, 4);
    }
    {
        let nums = vec![1, 1, 1, 1];
        let ret = num_identical_pairs(nums);
        assert_eq!(ret, 6);
    }
    {
        let nums = vec![1, 2, 3, 1, 1, 3];
        let ret = num_identical_pairs(nums);
        assert_eq!(ret, 4);
    }
}
