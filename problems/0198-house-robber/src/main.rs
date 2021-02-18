fn rob(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut prev_max = 0;

    for &n in &nums {
        let tmp = max;
        max = std::cmp::max(prev_max + n, max);
        prev_max = tmp;
    }

    max
}

fn main() {
    println!("rob([1,2,3,1]={}", rob(vec![1, 2, 3, 1]));
}

#[test]
fn test_rob() {
    assert_eq!(rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(rob(vec![2, 7, 9, 3, 1]), 12);
}
