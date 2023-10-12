fn minimum_right_shifts(nums: Vec<i32>) -> i32 {
    let mut min = std::i32::MAX;
    let mut min_index = nums.len();

    for (i, &num) in nums.iter().enumerate() {
        if num < min {
            min_index = i;
            min = num;
        }
    }

    let mut prev = nums[min_index];
    for i in 1..nums.len() {
        let index = (min_index + i) % nums.len();
        if prev > nums[index] {
            return -1;
        }

        prev = nums[index];
    }

    ((nums.len() - min_index) % nums.len()) as i32
}

fn main() {
    let nums = vec![3, 4, 5, 1, 2];
    let ret = minimum_right_shifts(nums);
    println!("ret={ret}");
}

#[test]
fn test_minimum_right_shifts() {
    {
        let nums = vec![3, 4, 5, 1, 2];
        let ret = minimum_right_shifts(nums);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![1, 3, 5];
        let ret = minimum_right_shifts(nums);
        assert_eq!(ret, 0);
    }
    {
        let nums = vec![2, 1, 4];
        let ret = minimum_right_shifts(nums);
        assert_eq!(ret, -1);
    }
}
