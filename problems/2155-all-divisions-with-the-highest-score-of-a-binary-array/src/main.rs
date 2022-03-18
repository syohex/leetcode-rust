fn max_score_indices(nums: Vec<i32>) -> Vec<i32> {
    let mut left = 0;
    let mut right = nums.iter().filter(|&&n| n == 1).count();

    let mut ret = vec![0];
    let mut max = left + right;
    for (i, n) in nums.into_iter().enumerate() {
        if n == 0 {
            left += 1;
        } else {
            right -= 1;
        }

        let score = left + right;
        if score > max {
            ret = vec![(i + 1) as i32];
            max = score;
        } else if score == max {
            ret.push((i + 1) as i32);
        }
    }

    ret
}

fn main() {
    let nums = vec![0, 0, 1, 0];
    let ret = max_score_indices(nums);
    println!("ret={:?}", ret);
}

#[test]
fn test_max_score_indices() {
    {
        let nums = vec![0, 0, 1, 0];
        let expected = vec![2, 4];
        let ret = max_score_indices(nums);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![0, 0, 0];
        let expected = vec![3];
        let ret = max_score_indices(nums);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![1, 1];
        let expected = vec![0];
        let ret = max_score_indices(nums);
        assert_eq!(ret, expected);
    }
}
