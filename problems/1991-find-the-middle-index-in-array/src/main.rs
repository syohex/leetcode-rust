fn find_middle_index(nums: Vec<i32>) -> i32 {
    let mut v = Vec::new();
    let mut sum = 0;

    v.push(0);

    for num in &nums {
        sum += num;
        v.push(sum);
    }

    for i in 1..=nums.len() {
        let left = v[i - 1];
        let right = sum - v[i];
        if left == right {
            return (i - 1) as i32;
        }
    }

    -1
}

fn main() {
    let nums = vec![2,3,-1,8,4];
    println!("ret={}", find_middle_index(nums));
}

#[test]
fn test_find_middle_index() {
    {
        let nums = vec![2,3,-1,8,4];
        assert_eq!(find_middle_index(nums), 3);
    }
    {
        let nums = vec![1,-1,4];
        assert_eq!(find_middle_index(nums), 2);
    }
    {
        let nums = vec![2,5];
        assert_eq!(find_middle_index(nums), -1);
    }
}
