fn triangle_number(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    if len < 3 {
        return 0;
    }

    let mut nums = nums;
    nums.sort_unstable();

    let mut ret = 0;
    for i in 0..len {
        for j in (i + 1)..len {
            let mut left = j + 1;
            let mut right = len - 1;

            while left <= right {
                let mid = left + (right - left) / 2;
                if nums[i] + nums[j]  > nums[mid] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }

            ret += left - 1 - j;
        }
    }

    ret as i32
}

fn main() {
    let nums = vec![2, 2, 3, 4];
    let ret = triangle_number(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![0, 0, 0];
        let ret = triangle_number(nums);
        assert_eq!(ret, 0);
    }
    {
        let nums = vec![0, 0];
        let ret = triangle_number(nums);
        assert_eq!(ret, 0);
    }
    {
        let nums = vec![2, 2, 3, 4];
        let ret = triangle_number(nums);
        assert_eq!(ret, 3);
    }
    {
        let nums = vec![4, 2, 3, 4];
        let ret = triangle_number(nums);
        assert_eq!(ret, 4);
    }
}
