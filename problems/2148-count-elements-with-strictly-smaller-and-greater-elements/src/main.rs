fn count_elements(nums: Vec<i32>) -> i32 {
    let min = nums.iter().min().unwrap();
    let max = nums.iter().max().unwrap();

    nums.iter().filter(|n| **n > *min && **n < *max).count() as i32
}

fn main() {
    let nums = vec![11, 7, 2, 15];
    let ret = count_elements(nums);
    println!("ret={ret}");
}

#[test]
fn test_count_element() {
    {
        let nums = vec![11, 7, 2, 15];
        assert_eq!(count_elements(nums), 2);
    }
    {
        let nums = vec![-3, 3, 3, 90];
        assert_eq!(count_elements(nums), 2);
    }
}
