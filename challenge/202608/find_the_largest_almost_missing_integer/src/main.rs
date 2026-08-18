fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let len = nums.len();
    let mut freq = [0; 51];
    for &n in &nums {
        freq[n as usize] += 1;
    }

    if k == 1 {
        for (i, f) in freq.into_iter().enumerate().rev() {
            if f == 1 {
                return i as i32;
            }
        }

        -1
    } else if k == len {
        nums.into_iter().max().unwrap()
    } else {
        let (first, last) = (nums[0], nums[len - 1]);
        let firsts = freq[first as usize];
        let lasts = freq[last as usize];

        match (firsts == 1, lasts == 1) {
            (true, true) => std::cmp::max(first, last),
            (true, false) => first,
            (false, true) => last,
            (false, false) => -1,
        }
    }
}

fn main() {
    let ret = largest_integer(vec![3, 9, 2, 1, 7], 7);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(largest_integer(vec![1, 2, 9, 9, 4, 5], 1), 5);
    assert_eq!(largest_integer(vec![4, 4, 2, 2, 2, 0, 5, 3, 4, 4], 3), -1);
    assert_eq!(largest_integer(vec![0, 0], 2), 0);
    assert_eq!(largest_integer(vec![3, 9, 2, 1, 7], 3), 7);
    assert_eq!(largest_integer(vec![3, 9, 7, 2, 1, 7], 3), 3);
    assert_eq!(largest_integer(vec![0, 0], 1), -1);
    assert_eq!(largest_integer(vec![1, 2, 9, 4, 5], 1), 9);
}
