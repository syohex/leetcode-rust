fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut prev = nums[0];
    let mut pos = 1;

    for i in 1..nums.len() {
        if nums[i] != prev {
            nums[pos] = nums[i];
            pos += 1;

            prev = nums[i];
        }
    }

    pos as i32
}

fn main() {
    let mut nums = vec![1, 1, 2];
    let ret = remove_duplicates(&mut nums);
    println!(
        "ret={ret}, nums={:?}",
        nums.into_iter().take(ret as usize).collect::<Vec<i32>>()
    );
}

#[test]
fn test_remove_duplicates() {
    {
        let mut nums = vec![1, 1, 2];
        let expected = vec![1, 2];
        let ret = remove_duplicates(&mut nums);
        assert_eq!(ret, 2);
        assert_eq!(
            nums.into_iter().take(ret as usize).collect::<Vec<i32>>(),
            expected
        );
    }
    {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let expected = vec![0, 1, 2, 3, 4];
        let ret = remove_duplicates(&mut nums);
        assert_eq!(ret, 5);
        assert_eq!(
            nums.into_iter().take(ret as usize).collect::<Vec<i32>>(),
            expected
        );
    }
    {
        let mut nums = vec![1, 1, 1, 1, 1];
        let expected = vec![1];
        let ret = remove_duplicates(&mut nums);
        assert_eq!(ret, 1);
        assert_eq!(
            nums.into_iter().take(ret as usize).collect::<Vec<i32>>(),
            expected
        );
    }
}
