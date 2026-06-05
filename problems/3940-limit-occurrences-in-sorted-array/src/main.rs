fn limit_occurrences(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut prev = nums[0];
    let mut count = 1;
    let mut ret = vec![prev];
    for n in nums.into_iter().skip(1) {
        if n == prev {
            if count >= k {
                continue;
            }
            count += 1;
        } else {
            prev = n;
            count = 1;
        }
        ret.push(n);
    }

    ret
}

fn main() {
    let ret = limit_occurrences(vec![1, 1, 1, 2, 2, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4], 2);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    assert_eq!(
        limit_occurrences(vec![1, 1, 1, 2, 2, 3], 2),
        vec![1, 1, 2, 2, 3]
    );
    assert_eq!(limit_occurrences(vec![1, 2, 3], 1), vec![1, 2, 3]);
}
