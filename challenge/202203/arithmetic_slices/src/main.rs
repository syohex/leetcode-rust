fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    if nums.len() <= 2 {
        return 0;
    }

    let count_slices = |count: i32| -> i32 {
        let n = count - 3 + 1;
        (n * (1 + n)) / 2
    };

    let mut prev2 = nums[0];
    let mut prev1 = nums[1];
    let mut ret = 0;
    let mut count = 2;

    for num in nums.into_iter().skip(2) {
        if (num - prev1) == (prev1 - prev2) {
            count += 1;
        } else {
            if count >= 3 {
                ret += count_slices(count);
            }

            count = 2;
        }

        prev2 = prev1;
        prev1 = num;
    }

    if count >= 3 {
        ret += count_slices(count);
    }

    ret
}

fn main() {
    let nums = vec![1, 2, 3, 4];
    let ret = number_of_arithmetic_slices(nums);
    println!("ret={ret}");
}

#[test]
fn test_number_of_arithmetic_slices() {
    {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(number_of_arithmetic_slices(nums), 3);
    }
    {
        let nums = vec![1];
        assert_eq!(number_of_arithmetic_slices(nums), 0);
    }
    {
        let nums = vec![7, 7, 7, 7];
        assert_eq!(number_of_arithmetic_slices(nums), 3);
    }
    {
        let nums = vec![3, -1, -5, -9];
        assert_eq!(number_of_arithmetic_slices(nums), 3);
    }
}
