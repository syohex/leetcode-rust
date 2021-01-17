fn sort_colors(nums: &mut Vec<i32>) {
    let mut v: [i32; 3] = [0, 0, 0];
    for n in nums.iter() {
        v[*n as usize] += 1;
    }

    let mut pos = 0usize;
    for (i, n) in v.iter().enumerate() {
        for _ in (0..*n).into_iter() {
            nums[pos] = i as i32;
            pos += 1;
        }
    }
}

fn main() {
    let mut nums = vec![2, 0, 2, 1, 1, 0];
    sort_colors(&mut nums);
    println!("nums = {:?}", nums);
}

#[test]
fn test_sort_colors() {
    {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
    }
    {
        let mut nums = vec![2, 0, 1];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 1, 2]);
    }
    {
        let mut nums = vec![0];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0]);
    }
    {
        let mut nums = vec![1];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![1]);
    }
}
