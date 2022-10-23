fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let mut v = vec![1; nums.len()];
    for num in nums {
        v[(num - 1) as usize] -= 1;
    }

    let mut ret = vec![0; 2];
    for (i, num) in v.into_iter().enumerate() {
        if num < 0 {
            ret[0] = i as i32 + 1;
        } else if num > 0 {
            ret[1] = i as i32 + 1;
        }
    }

    ret
}

fn main() {
    let nums = vec![1, 2, 2, 4];
    let ret = find_error_nums(nums);
    println!("ret={:?}", ret);
}

#[test]
fn test_find_error_nums() {
    {
        let nums = vec![1, 2, 2, 4];
        let expected = vec![2, 3];
        let ret = find_error_nums(nums);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![1, 1];
        let expected = vec![1, 2];
        let ret = find_error_nums(nums);
        assert_eq!(ret, expected);
    }
}
