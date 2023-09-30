fn find_132_pattern(nums: Vec<i32>) -> bool {
    let mut mins = vec![0; nums.len()];
    mins[0] = nums[0];

    for i in 1..nums.len() {
        mins[i] = std::cmp::min(mins[i - 1], nums[i]);
    }

    let mut stack = vec![];
    for i in (1..nums.len()).rev() {
        if nums[i] == mins[i] {
            continue;
        }

        while !stack.is_empty() && *stack.last().unwrap() <= mins[i] {
            stack.pop();
        }

        if !stack.is_empty() && nums[i] > *stack.last().unwrap() {
            return true;
        }

        stack.push(nums[i]);
    }

    false
}

fn main() {
    let ret = find_132_pattern(vec![1, 2, 3, 4]);
    println!("ret={ret}");
}

#[test]
fn test_find_132_pattern() {
    assert!(!find_132_pattern(vec![1, 2, 3, 4]));
    assert!(find_132_pattern(vec![3, 1, 4, 2]));
    assert!(find_132_pattern(vec![-1, 3, 2, 0]));
}
