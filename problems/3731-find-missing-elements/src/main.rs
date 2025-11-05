fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    nums.sort_unstable();

    let mut ret = vec![];
    let mut prev = nums[0];
    for n in nums.into_iter().skip(1) {
        if n - prev != 1 {
            for i in (prev + 1)..n {
                ret.push(i);
            }
        }

        prev = n;
    }

    ret
}

fn main() {
    let ret = find_missing_elements(vec![1, 9]);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    assert_eq!(find_missing_elements(vec![1, 4, 2, 5]), [3]);
    assert_eq!(find_missing_elements(vec![7, 8, 6, 9]), []);
    assert_eq!(find_missing_elements(vec![5, 1]), [2, 3, 4]);
}
