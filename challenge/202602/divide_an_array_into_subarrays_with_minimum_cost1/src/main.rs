fn minimum_cost(nums: Vec<i32>) -> i32 {
    let mut min1 = 100;
    let mut min2 = 100;

    for &n in nums.iter().skip(1) {
        if n < min1 {
            min2 = min1;
            min1 = n;
        } else if n < min2 {
            min2 = n;
        }
    }

    nums[0] + min1 + min2
}

fn main() {
    let ret = minimum_cost(vec![1, 2, 3, 12]);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    assert_eq!(minimum_cost(vec![1, 2, 3, 12]), 6);
    assert_eq!(minimum_cost(vec![5, 4, 3]), 12);
    assert_eq!(minimum_cost(vec![10, 3, 1, 1]), 12);
}
