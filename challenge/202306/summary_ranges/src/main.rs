fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut ret = vec![];
    if nums.is_empty() {
        return ret;
    }

    let mut tmp = String::new();
    let mut start = nums[0];
    let mut prev = nums[0];

    for num in nums.into_iter().skip(1) {
        if num - 1 != prev {
            if start == prev {
                tmp.push_str(&start.to_string());
            } else {
                tmp.push_str(&format!("{start}->{prev}"));
            }

            ret.push(tmp);

            tmp = String::new();
            start = num;
        }
        prev = num;
    }

    if start == prev {
        ret.push(start.to_string());
    } else {
        ret.push(format!("{start}->{prev}"));
    }

    ret
}

fn main() {
    let nums = vec![0, 1, 2, 4, 5, 7];
    let ret = summary_ranges(nums);
    println!("ret={ret:?}");
}

#[test]
fn test_summary_ranges() {
    {
        let nums = vec![0, 1, 2, 4, 5, 7];
        let expected = vec!["0->2".to_string(), "4->5".to_string(), "7".to_string()];
        let ret = summary_ranges(nums);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![0, 2, 3, 4, 6, 8, 9];
        let expected = vec![
            "0".to_string(),
            "2->4".to_string(),
            "6".to_string(),
            "8->9".to_string(),
        ];
        let ret = summary_ranges(nums);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![];
        let ret = summary_ranges(nums);
        assert!(ret.is_empty());
    }
    {
        let nums = vec![0, 2, 4, 6];
        let expected = vec![
            "0".to_string(),
            "2".to_string(),
            "4".to_string(),
            "6".to_string(),
        ];
        let ret = summary_ranges(nums);
        assert_eq!(ret, expected);
    }
}
