fn result_arrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut consecutives = 1;
    for i in (1..k).rev() {
        if nums[i - 1] + 1 == nums[i] {
            consecutives += 1;
            continue;
        }

        break;
    }

    let mut ret = if consecutives == k {
        vec![nums[k - 1]]
    } else {
        vec![-1]
    };

    for i in k..nums.len() {
        if consecutives == k {
            consecutives -= 1;
        }

        if nums[i - 1] + 1 == nums[i] {
            consecutives += 1;
        } else {
            consecutives = 1;
        }

        if consecutives == k {
            ret.push(nums[i]);
        } else {
            ret.push(-1);
        }
    }

    ret
}
fn main() {
    let nums = vec![1, 2, 3, 4, 3, 2, 5];
    let ret = result_arrays(nums, 3);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 2, 3, 4, 3, 2, 5];
        let expected = vec![3, 4, -1, -1, -1];
        let ret = result_arrays(nums, 3);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![2, 2, 2, 2, 2];
        let expected = vec![-1, -1];
        let ret = result_arrays(nums, 4);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![3, 2, 3, 2, 3, 2];
        let expected = vec![-1, 3, -1, 3, -1];
        let ret = result_arrays(nums, 2);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![1, 3, 4];
        let expected = vec![-1, 4];
        let ret = result_arrays(nums, 2);
        assert_eq!(ret, expected);
    }
}
