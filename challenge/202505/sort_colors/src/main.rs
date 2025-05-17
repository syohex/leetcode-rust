fn sort_colors(nums: &mut Vec<i32>) {
    let mut pos = 0;
    let mut pos0 = 0;
    let mut pos2 = nums.len() as i32 - 1;

    while pos <= pos2 {
        if nums[pos as usize] == 0 {
            nums.swap(pos as usize, pos0 as usize);
            pos0 += 1;
            pos += 1;
        } else if nums[pos as usize] == 2 {
            nums.swap(pos as usize, pos2 as usize);
            pos2 -= 1;
        } else {
            pos += 1;
        }
    }
}

fn main() {
    let mut nums = vec![2, 0, 2, 1, 1, 0];
    sort_colors(&mut nums);
    println!("ret={nums:?}");
}

#[test]
fn test() {
    {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        let expected = vec![0, 0, 1, 1, 2, 2];
        sort_colors(&mut nums);
        assert_eq!(nums, expected);
    }
    {
        let mut nums = vec![2, 0, 1];
        let expected = vec![0, 1, 2];
        sort_colors(&mut nums);
        assert_eq!(nums, expected);
    }
    {
        let mut nums = vec![2];
        let expected = vec![2];
        sort_colors(&mut nums);
        assert_eq!(nums, expected);
    }
}
