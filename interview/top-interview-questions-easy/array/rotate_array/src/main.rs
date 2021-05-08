fn rotate(nums: &mut Vec<i32>, k: i32) {
    let k = k as usize % nums.len();
    let v: Vec<i32> = (0..nums.len())
        .map(|i| {
            let index = (nums.len() - k + i) % nums.len();
            nums[index]
        })
        .collect();

    for i in 0..nums.len() {
        nums[i] = v[i];
    }
}

fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    rotate(&mut nums, 3);
    println!("ret={:?}", nums);
}

#[test]
fn test_rotate() {
    {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        let expected = vec![5, 6, 7, 1, 2, 3, 4];
        rotate(&mut nums, 3);
        assert_eq!(nums, expected);
    }
    {
        let mut nums = vec![-1, -100, 3, 99];
        let expected = vec![3, 99, -1, -100];
        rotate(&mut nums, 2);
        assert_eq!(nums, expected);
    }
}
