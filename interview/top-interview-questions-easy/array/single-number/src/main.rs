fn single_number(nums: Vec<i32>) -> i32 {
    let mut ret = 0;
    for n in nums {
        ret ^= n;
    }

    ret
}

fn main() {
    let ret = single_number(vec![2, 2, 1]);
    println!("ret={}", ret);
}

#[test]
fn test_single_number() {
    assert_eq!(single_number(vec![2, 2, 1]), 1);
    assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4);
    assert_eq!(single_number(vec![1]), 1);
}
