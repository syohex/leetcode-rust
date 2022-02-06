fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut index = 0;
    let mut count = 0;
    let mut prev = std::i32::MIN;

    for i in 0..nums.len() {
        if nums[i] == prev {
            count += 1;
            if count >= 3 {
                continue;
            }
        } else {
            prev = nums[i];
            count = 1;
        }

        nums[index] = nums[i];
        index += 1;
    }

    index as i32
}

fn main() {
    let mut nums = vec![1, 1, 1, 2, 2, 3];
    let ret = remove_duplicates(&mut nums);
    println!("ret={ret}, nums={:?}", nums);
}

#[test]
fn test_remove_duplicates() {
    {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let expected = vec![1, 1, 2, 2, 3];
        let ret = remove_duplicates(&mut nums);
        let nums: Vec<i32> = nums.into_iter().take(ret as usize).collect();
        assert_eq!(nums, expected);
    }
    {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        let expected = vec![0, 0, 1, 1, 2, 3, 3];
        let ret = remove_duplicates(&mut nums);
        let nums: Vec<i32> = nums.into_iter().take(ret as usize).collect();
        assert_eq!(nums, expected);
    }
}
