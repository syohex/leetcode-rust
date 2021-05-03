fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut ret = vec![];
    let mut acc = 0;
    for n in nums {
        ret.push(acc + n);
        acc += n;
    }

    ret
}

fn main() {
    let ret = running_sum(vec![1, 2, 3, 4]);
    println!("ret={:?}", ret);
}

#[test]
fn test_running_sum() {
    assert_eq!(running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
    assert_eq!(running_sum(vec![1, 1, 1, 1, 1]), vec![1, 2, 3, 4, 5]);
    assert_eq!(running_sum(vec![3, 1, 2, 10, 1]), vec![3, 4, 6, 16, 17]);
}
