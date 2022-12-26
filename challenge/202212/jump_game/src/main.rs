fn can_jump(nums: Vec<i32>) -> bool {
    let goal = nums.len() - 1;
    let mut q = vec![0usize];

    while let Some(p) = q.pop() {
        if p >= goal {
            return true;
        }

        for i in 1..=nums[p] {
            q.push(p + i as usize);
        }
    }

    false
}

fn main() {
    let nums = vec![2, 3, 1, 1, 4];
    let ret = can_jump(nums);
    println!("ret={ret}");
}

#[test]
fn test_can_jump() {
    {
        let nums = vec![2, 3, 1, 1, 4];
        assert!(can_jump(nums));
    }
    {
        let nums = vec![3, 2, 1, 0, 4];
        assert!(!can_jump(nums));
    }
}
