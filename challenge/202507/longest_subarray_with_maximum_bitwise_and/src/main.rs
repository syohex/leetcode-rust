fn longest_subarray(nums: Vec<i32>) -> i32 {
    let max = *nums.iter().max().unwrap();

    let mut ret = 0;
    let mut i = 0;
    while i < nums.len() {
        if nums[i] == max {
            let orig = i;

            i += 1;
            while i < nums.len() && nums[i] == max {
                i += 1;
            }

            ret = std::cmp::max(ret, i - orig);
        } else {
            i += 1;
        }
    }

    ret as i32
}

fn main() {
    let nums = vec![1, 2, 3, 3, 2, 2];
    let ret = longest_subarray(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 2, 3, 3, 2, 2];
        let ret = longest_subarray(nums);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![1, 2, 3, 4];
        let ret = longest_subarray(nums);
        assert_eq!(ret, 1);
    }
}
