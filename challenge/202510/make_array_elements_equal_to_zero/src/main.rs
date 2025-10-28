fn count_valid_selections(nums: Vec<i32>) -> i32 {
    let sum: i32 = nums.iter().sum();
    let mut ret = 0;

    let mut left_sum = 0;
    for n in nums {
        left_sum += n;
        if n == 0 {
            let diff = (sum - 2 * left_sum).abs();
            match diff {
                0 => ret += 2,
                1 => ret += 1,
                _ => (),
            }
        }
    }

    ret
}

fn main() {
    let nums = vec![1, 0, 2, 0, 3];
    let ret = count_valid_selections(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![16, 13, 10, 0, 0, 0, 10, 6, 7, 8, 7];
        let ret = count_valid_selections(nums);
        assert_eq!(ret, 3);
    }
    {
        let nums = vec![1, 0, 2, 0, 3];
        let ret = count_valid_selections(nums);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![2, 3, 4, 0, 4, 1, 0];
        let ret = count_valid_selections(nums);
        assert_eq!(ret, 0);
    }
}
