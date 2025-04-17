fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
    let mut ret = 0;
    let len = nums.len();

    for i in 0..len - 1 {
        for j in i + 1..len {
            if nums[i] == nums[j] && (i * j) % k as usize == 0 {
                ret += 1;
            }
        }
    }

    ret
}

fn main() {
    let nums = vec![3, 1, 2, 2, 2, 1, 3];
    let ret = count_pairs(nums, 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![3, 1, 2, 2, 2, 1, 3];
        let ret = count_pairs(nums, 2);
        assert_eq!(ret, 4);
    }
    {
        let nums = vec![1, 2, 3, 4];
        let ret = count_pairs(nums, 1);
        assert_eq!(ret, 0);
    }
}
