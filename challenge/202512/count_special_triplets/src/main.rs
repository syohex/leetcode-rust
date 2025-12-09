fn special_triplets(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut freq = HashMap::new();
    for &num in &nums {
        *freq.entry(num).or_insert(0i64) += 1;
    }

    let modulo = 1_000_000_007i64;
    let mut ret = 0;

    let mut checked = HashMap::new();
    for num in nums {
        let key = num * 2;

        let left = *checked.get(&key).unwrap_or(&0);
        // default case num=0
        *checked.entry(num).or_insert(0) += 1;

        let right = *freq.get(&key).unwrap_or(&0) - *checked.get(&key).unwrap_or(&0);

        ret = (ret + left * right % modulo) % modulo;
    }

    ret as i32
}

fn main() {
    let ret = special_triplets(vec![8, 4, 2, 8, 4]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(special_triplets(vec![6, 3, 6]), 1);
    assert_eq!(special_triplets(vec![0, 1, 0, 0]), 1);
    assert_eq!(special_triplets(vec![8, 4, 2, 8, 4]), 2);
}
