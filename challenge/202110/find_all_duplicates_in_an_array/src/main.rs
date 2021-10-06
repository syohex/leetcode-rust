fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
    let mut v = vec![1; nums.len()];
    let mut ret = vec![];
    for num in &nums {
        let idx = (*num - 1) as usize;
        v[idx] *= -1;
        if v[idx] > 0 {
            ret.push(*num);
        }
    }

    ret
}

fn main() {
    let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
    println!("ret={:?}", find_duplicates(nums));
}

#[test]
fn test_find_duplicates() {
    {
        let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
        assert_eq!(find_duplicates(nums), vec![2, 3]);
    }
    {
        let nums = vec![1, 1, 2];
        assert_eq!(find_duplicates(nums), vec![1]);
    }
    {
        let nums = vec![1];
        assert_eq!(find_duplicates(nums), vec![]);
    }
}
