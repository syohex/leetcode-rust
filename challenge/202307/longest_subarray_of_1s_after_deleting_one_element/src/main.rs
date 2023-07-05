fn longest_subarray(nums: Vec<i32>) -> i32 {
    let mut v = vec![];
    let len = nums.len();
    let mut start = len + 1;
    for (i, n) in nums.into_iter().enumerate() {
        if n == 1 {
            if start == len + 1 {
                start = i;
            }
        }

        if (n == 0 || i == len - 1) && start != len + 1 {
            if n == 0 {
                v.push((start, i - start));
            } else {
                v.push((start, i - start + 1));
            }
            start = len + 1;
        }
    }

    let len2 = v.len();
    if len2 == 1 {
        if v[0].1 == len {
            (len - 1) as i32
        } else {
            v[0].1 as i32
        }
    } else if len2 == 0 {
        0
    } else {
        let mut ret = 0;
        dbg!(&v);
        for i in 0..(len2 - 1) {
            if v[i].0 + v[i].1 + 1 == v[i + 1].0 {
                ret = std::cmp::max(ret, (v[i].1 + v[i + 1].1) as i32);
            } else {
                ret = std::cmp::max(ret, v[i].1 as i32);
            }
        }

        ret
    }
}

fn main() {
    let nums = vec![0, 1, 1, 1, 0, 1, 1, 0, 1];
    let ret = longest_subarray(nums);
    println!("ret={ret}");
}

#[test]
fn test_longest_subarray() {
    {
        let nums = vec![1, 1, 0, 1];
        let ret = longest_subarray(nums);
        assert_eq!(ret, 3);
    }
    {
        let nums = vec![0, 1, 1, 1, 0, 1, 1, 0, 1];
        let ret = longest_subarray(nums);
        assert_eq!(ret, 5);
    }
    {
        let nums = vec![0, 1, 1, 1, 0, 0, 1, 1, 0, 1];
        let ret = longest_subarray(nums);
        assert_eq!(ret, 3);
    }
    {
        let nums = vec![1, 1, 1];
        let ret = longest_subarray(nums);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![0, 0, 0];
        let ret = longest_subarray(nums);
        assert_eq!(ret, 0);
    }
    {
        let nums = vec![1, 0, 0, 0, 0];
        let ret = longest_subarray(nums);
        assert_eq!(ret, 1);
    }
    {
        let nums = vec![1, 0, 1, 0, 1, 0];
        let ret = longest_subarray(nums);
        assert_eq!(ret, 2);
    }
}
