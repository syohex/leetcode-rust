fn longest_subarray(nums: Vec<i32>) -> i32 {
    let mut i = 0;
    let len = nums.len();
    let mut v = vec![];
    while i < len {
        if nums[i] == 0 {
            i += 1;
            continue;
        }

        let mut count = 1;
        let start = i;
        i += 1;
        while i < len && nums[i] == 1 {
            count += 1;
            i += 1;
        }

        v.push((count, start, i));
        i += 1;
    }

    match v.len() {
        0 => 0,
        1 => {
            if v[0].1 == 0 && v[0].2 == len {
                v[0].0 - 1
            } else {
                v[0].0
            }
        }
        _ => {
            let mut ret = v[0].0;
            for j in 1..v.len() {
                if v[j].1 - v[j - 1].2 == 1 {
                    ret = std::cmp::max(ret, v[j].0 + v[j - 1].0);
                } else {
                    ret = std::cmp::max(ret, v[j].0);
                }
            }
            ret
        }
    }
}

fn main() {
    let nums = vec![0, 1, 1, 1, 0, 0, 1, 1, 0];
    let ret = longest_subarray(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![0, 1, 1, 1, 0, 0, 1, 1, 0];
        let ret = longest_subarray(nums);
        assert_eq!(ret, 3);
    }
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
        let nums = vec![1, 1, 1];
        let ret = longest_subarray(nums);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![1, 0, 0, 0, 0];
        let ret = longest_subarray(nums);
        assert_eq!(ret, 1);
    }
    {
        let nums = vec![1, 0, 1, 0, 1];
        let ret = longest_subarray(nums);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![1, 0, 0, 1, 0];
        let ret = longest_subarray(nums);
        assert_eq!(ret, 1);
    }
}
