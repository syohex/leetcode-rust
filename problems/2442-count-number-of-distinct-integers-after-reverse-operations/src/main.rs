fn count_distinct_integers(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    fn reverse(num: i32) -> i32 {
        let mut num = num;
        let mut ret = 0;

        while num > 0 {
            ret = ret * 10 + (num % 10);
            num /= 10;
        }

        ret
    }

    let mut s: HashSet<i32> = nums.iter().map(|num| *num).collect();
    for num in nums {
        s.insert(reverse(num));
    }

    s.len() as i32
}

fn main() {
    let nums = vec![1, 13, 10, 12, 31];
    let ret = count_distinct_integers(nums);
    println!("ret={ret}");
}

#[test]
fn test_count_distinct_integers() {
    {
        let nums = vec![1, 13, 10, 12, 31];
        let ret = count_distinct_integers(nums);
        assert_eq!(ret, 6);
    }
    {
        let nums = vec![2, 2, 2];
        let ret = count_distinct_integers(nums);
        assert_eq!(ret, 1);
    }
}
