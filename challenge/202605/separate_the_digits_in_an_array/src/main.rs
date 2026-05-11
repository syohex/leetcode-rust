fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
    nums.into_iter().fold(vec![], |acc, n| {
        n.to_string().bytes().fold(acc, |mut acc, b| {
            acc.push((b - b'0') as i32);
            acc
        })
    })
}

fn main() {
    let ret = separate_digits(vec![13, 25, 83, 77]);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    assert_eq!(
        separate_digits(vec![13, 25, 83, 77]),
        vec![1, 3, 2, 5, 8, 3, 7, 7]
    );
    assert_eq!(separate_digits(vec![7, 1, 3, 9]), vec![7, 1, 3, 9]);
}
